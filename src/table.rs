use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs,
    path::Path,
};

use anyhow::{anyhow, Result};
use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;
use rand::{seq::{SliceRandom, IteratorRandom}, thread_rng};

use crate::dice::Dice;

#[derive(Parser)]
#[grammar = "table.pest"]
struct TableParser;

#[derive(Debug)]
enum LineItemSection {
    Text(String),
    Dice(Dice),
    Reference(String),
}

#[derive(Debug)]
struct LineItem {
    weight: i32,
    sections: Vec<LineItemSection>,
    _references: HashSet<String>,
}

#[derive(Debug)]
struct Table {
    name: String,
    line_items: Vec<LineItem>,
    sub_tables: Tables,
}

#[derive(Debug)]
pub struct Tables {
    map: HashMap<String, Table>,
    first_table: Option<String>,
}

impl Tables {
	const MAX_DEPTH: usize = 400;

    pub fn new<P>(path: P) -> Result<Tables>
    where
        P: AsRef<Path> + Debug,
    {
        let unparsed_file = fs::read_to_string(&path)?;
        let document = TableParser::parse(Rule::document, &unparsed_file)?;
        let tables = Self::parse_tables(document)?;
        Ok(tables)
    }

	pub fn any_name(&self) -> Option<&String> {
		self.map.keys().choose(&mut thread_rng())
	}

	pub fn first_name(&self) -> Option<&String> {
		self.first_table.as_ref()
	}

    fn parse_tables(pairs: Pairs<Rule>) -> Result<Tables> {
        let mut map: HashMap<String, Table> = HashMap::new();
        let mut first_table: Option<String> = None;
        let mut first = true;
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::table_1
                | Rule::table_2
                | Rule::table_3
                | Rule::table_4
                | Rule::table_5
                | Rule::table_6 => {
                    let table = Self::parse_table(&mut pair.into_inner())?;
                    if first {
                        first_table = Some(table.name.clone());
                        first = false
                    }
                    if map.contains_key(&table.name) {
                        return Err(anyhow!("Duplicate table detected: \"{}\"", table.name));
                    } else {
                        map.insert(table.name.clone(), table);
                    }
                }
                _ => {}
            }
        }

        Ok(Tables { map, first_table })
        
    }

    fn parse_table(pairs: &mut Pairs<Rule>) -> Result<Table> {
        let name = pairs
            .next()
            .expect("table missing header")
            .as_str()
            .to_string();
        let line_items = Self::parse_line_items(
            &mut pairs.next().expect("table missing line items").into_inner(),
        )?;
        let sub_tables = if let Some(pair) = pairs.next() {
            Self::parse_tables(pair.into_inner())?
        } else {
            Tables {
                map: HashMap::new(),
                first_table: None,
            }
        };
        Ok(Table {
            name,
            line_items,
            sub_tables,
        })
    }

    fn parse_line_items(pairs: &mut Pairs<Rule>) -> Result<Vec<LineItem>> {
        let mut line_items = Vec::new();
        for pair in pairs.into_iter() {
            line_items.push(Self::parse_line_item(pair.into_inner())?);
        }
        Ok(line_items)
    }

    fn parse_line_item(pairs: Pairs<Rule>) -> Result<LineItem> {
        let mut weight = 1;
        let mut sections = Vec::new();
        let mut references = HashSet::new();
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::weight => weight = pair.as_str().parse::<i32>()?,
                Rule::line_text => sections.push(LineItemSection::Text(pair.as_str().to_string())),
                Rule::dice => sections.push(LineItemSection::Dice(Dice::parse(pair.as_str())?)),
                Rule::section_link => {
                    let reference = pair.as_str().to_string();
                    references.insert(reference.clone());
                    sections.push(LineItemSection::Reference(reference))
                }
                _ => {}
            }
        }
        Ok(LineItem {
            weight,
            sections,
            _references: references,
        })
    }

    pub fn roll(&self, table_name: &String) -> Result<String> {
        self.roll_rec(self, table_name, true, 0)
    }

    fn roll_rec(
		&self,
        tables: &Tables,
        table_name: &String,
        check_global: bool,
        depth: usize,
    ) -> Result<String> {
        if depth == Self::MAX_DEPTH {
            return Err(anyhow!("Max reference depth ({}) reached for table \"{}\", last tables searched: {:#?}", Self::MAX_DEPTH, table_name, tables.map));
        }
        if let Some(table) = tables.map.get(table_name) {
            let mut output: Vec<String> = Vec::new();
            let line_item = table
                .line_items
                .choose_weighted(&mut thread_rng(), |item| item.weight)?;
            for section in line_item.sections.iter() {
                output.push(match section {
                    LineItemSection::Text(text) => text.to_owned(),
                    LineItemSection::Dice(dice) => dice.roll().to_string(),
                    LineItemSection::Reference(reference) => {
						if table.sub_tables.map.is_empty() {
							self.roll_rec(self, &reference, false, depth + 1)?
						} else {
							self.roll_rec(&table.sub_tables, &reference, true, depth + 1)?
						}
					},
                })
            }
            return Ok(output.join(""));
        } else {
			for (_, table) in tables.map.iter() {
				if table.sub_tables.map.is_empty() {
					continue;
				}
				match self.roll_rec(&table.sub_tables, table_name, false, depth + 1) {
					Ok(ok) => return Ok(ok),
					Err(err) => eprintln!("{:#?}", err),
				}
			}
			if check_global {
				return Ok(self.roll_rec(&self, table_name, false, depth + 1)?)
			} else {
				Err(anyhow!("Could not find table: {}", table_name))
			}
        }
    }
}
