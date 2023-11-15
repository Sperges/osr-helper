// use anyhow::{Result, anyhow, Context};
// use pest::iterators::Pairs;
// use pest::Parser;
// use pest_derive::Parser;
// use rand::seq::SliceRandom;
// use std::collections::HashMap;
// use std::fmt::Debug;
// use std::fs;
// use std::path::Path;

// #[derive(Parser)]
// #[grammar = "table.pest"]
// struct TableParser;

// #[derive(Debug)]
// pub struct LineItem {
// 	weight: i32,
// 	sections: Vec<LineItemSection>
// }

// #[derive(Debug)]
// pub enum LineItemSection {
//     Text(String),
//     Dice(String),
//     Link(String),
// }

// #[derive(Debug)]
// pub struct RollTable {
// 	pub header: String,
//     pub line_items: Vec<LineItem>,
//     pub sub_tables: Box<HashMap<String, RollTable>>,
// }

// impl RollTable {
// 	fn choose_weighted_line_item(&self) -> Result<&LineItem> {
// 		self.line_items.choose_weighted(&mut rand::thread_rng(), |item| item.weight).with_context(|| format!("not a good time"))
// 	}

// 	// fn choose_from_roll(&self, roll: i32) -> Result<&LineItem> {
// 	// 	let mut total = 0;
// 	// 	for line_item in self.line_items.iter() {
// 	// 		total += line_item.weight;
// 	// 		if total >= roll {
// 	// 			return Ok(&line_item);
// 	// 		}
// 	// 	}
// 	// 	return self.line_items.last().ok_or(anyhow!(format!("No items in table")))
// 	// }
// }

// #[derive(Debug)]
// pub struct RollTables {
// 	tables: HashMap<String, RollTable>,
// }

// impl RollTables {

//     pub fn roll(&self, tables: &HashMap<String, RollTable>, table_header: &String, check_full: bool) -> Result<String> {
//         if let Some(table) = tables.get(table_header) {
// 			let mut output: Vec<String> = Vec::new();

// 			let selection = table.choose_weighted_line_item()?;
// 			for section in selection.sections.iter() {
// 				output.push(match section {
// 					LineItemSection::Text(text) => text.to_owned(),
// 					LineItemSection::Dice(expression) => {
// 						let eval = crate::dice::inner_roll(expression)?;
// 						format!("{:#?}", eval.dice_results)
// 					},
// 					LineItemSection::Link(link) => self.roll(&table.sub_tables, link, true)?,
// 				})
// 			}

// 			return Ok(output.join(""))
// 		} else {
// 			for (_, table) in tables.iter() {
// 				match self.roll(&table.sub_tables, table_header, false) {
// 					Ok(ok) => return Ok(ok),
// 					Err(err) => println!("{:#?}", err),
// 				}
// 			}

// 			if check_full {
// 				match self.roll(&self.tables, table_header, false) {
// 					Ok(ok) => return Ok(ok),
// 					Err(err) => println!("{:#?}", err),
// 				}
// 			}

// 		}
// 		Err(anyhow!("Could not find table: {}", table_header))
//     }

// 	fn validate_tables(&self) -> (bool, Vec<String>) {
// 		let mut errs = Vec::new();
// 		for (_, table) in self.tables.iter() {
// 			errs.append(&mut self.validate_links(&table));
// 		}
// 		(errs.is_empty(), errs)
// 	}

//     fn validate_links(&self, table: &RollTable) -> Vec<String> {
// 		let mut errs = Vec::new();
// 		for line_item in table.line_items.iter() {
// 			for section in line_item.sections.iter() {
// 				match section {
// 					LineItemSection::Link(link) => if !self.has_link(table, link, true) {
// 						errs.push(format!("{} has an illegal reference to table {}", table.header, link));
// 					},
// 					_ => {},
// 				}
// 			}
// 		}

// 		for (_, sub_table) in table.sub_tables.iter() {
// 			errs.append(&mut self.validate_links(sub_table));
// 		}

//         errs
//     }

// 	fn has_link(&self, table: &RollTable, link: &String, check_full: bool) -> bool {
// 		for (header, sub_table) in table.sub_tables.iter() {
// 			if header == link {
// 				return true;
// 			}
// 			if self.has_link(sub_table, link, false) {
// 				return true;
// 			}
// 		}

// 		if check_full {
// 			for (header, table) in self.tables.iter() {
// 				if header == link {
// 					return true;
// 				}
// 				if self.has_link(table, link, false) {
// 					return true;
// 				}
// 			}
// 		}

// 		return false;
// 	}
// }

// pub fn roll<P>(path: P, table: &String, _roll: &Option<String>) -> Result<()>
// where
//     P: AsRef<Path> + Debug,
// {
//     let unparsed_file = fs::read_to_string(path)?;

//     let document = TableParser::parse(Rule::document, &unparsed_file)?;

//     let tables = RollTables {
// 		tables: parse_roll_tables(document)?,
// 	};

// 	let (links_valid, errs) = tables.validate_tables();

// 	if !links_valid {
// 		for err in errs.iter() {
// 			eprintln!("{:#?}", err);
// 		}
// 	} else {
// 		println!("{}", tables.roll(&tables.tables, table, true)?);
// 	}

//     Ok(())
// }

// fn parse_roll_tables(pairs: Pairs<Rule>) -> Result<HashMap<String, RollTable>> {
//     let mut tables: HashMap<String, RollTable> = HashMap::new();
//     for pair in pairs.into_iter() {
//         match pair.as_rule() {
//             Rule::table_1
//             | Rule::table_2
//             | Rule::table_3
//             | Rule::table_4
//             | Rule::table_5
//             | Rule::table_6 => {
//                 let (header, roll_table) = parse_roll_table(pair.into_inner())?;
//                 tables.insert(header, roll_table);
//             }
//             _ => {}
//         }
//     }
//     Ok(tables)
// }

// fn parse_roll_table(pairs: Pairs<Rule>) -> Result<(String, RollTable)> {
//     let mut header: String = String::new();
//     let mut line_items: Vec<LineItem> = Vec::new();
//     let mut sub_tables: HashMap<String, RollTable> = HashMap::new();

//     for pair in pairs {
//         match pair.as_rule() {
//             Rule::header_title => {
//                 header.insert_str(0, pair.as_str());
//             }
//             Rule::line_items => {
//                 line_items = parse_line_items(pair.into_inner())?;
//             }
//             Rule::table_1_subtables
//             | Rule::table_2_subtables
//             | Rule::table_3_subtables
//             | Rule::table_4_subtables
//             | Rule::table_5_subtables => {
//                 sub_tables = parse_roll_tables(pair.into_inner())?;
//             }
//             _ => {}
//         }
//     }

//     Ok((
//         header.clone(),
//         RollTable {
// 			header,
//             line_items,
//             sub_tables: Box::new(sub_tables),
//         },
//     ))
// }

// fn parse_line_items(pairs: Pairs<Rule>) -> Result<Vec<LineItem>> {

//     let mut line_items = Vec::new();
//     for pair in pairs {
// 		let mut weight: i32 = 1;
//         let mut sections = Vec::new();
//         for line_item in pair.into_inner().into_iter() {
//             match line_item.as_rule() {
// 				Rule::weight => {
// 					weight = line_item.as_str().parse::<i32>()?;
// 				}
//                 Rule::line_text => {
//                     sections.push(LineItemSection::Text(line_item.as_str().to_string()))
//                 },
//                 Rule::dice => sections.push(LineItemSection::Dice(line_item.as_str().to_string())),
//                 Rule::section_link => {
//                     sections.push(LineItemSection::Link(line_item.as_str().to_string()))
//                 },
//                 _ => {},
//             }
//         }
//         line_items.push(LineItem { weight, sections });
//     }
//     Ok(
// 		line_items
// 	)
// }

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

// impl Table {
// 	fn roll(&self) -> Result<String> {
// 		let chosen = self.line_items.choose_weighted(&mut thread_rng(), |item| item.weight)?;
// 		todo!()
// 	}
// }

#[derive(Debug)]
pub struct Tables {
    map: HashMap<String, Table>,
}

impl Tables {
	const MAX_DEPTH: usize = 100;

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
		self.map.keys().next()
	}

    fn parse_tables(pairs: Pairs<Rule>) -> Result<Tables> {
        let mut map: HashMap<String, Table> = HashMap::new();
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::table_1
                | Rule::table_2
                | Rule::table_3
                | Rule::table_4
                | Rule::table_5
                | Rule::table_6 => {
                    let table = Self::parse_table(&mut pair.into_inner())?;
                    if map.contains_key(&table.name) {
                        return Err(anyhow!("Duplicate table detected: \"{}\"", table.name));
                    } else {
                        map.insert(table.name.clone(), table);
                    }
                }
                _ => {}
            }
        }
        Ok(Tables { map })
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
        // for line_item in line_items.iter() {
        //     if line_item.references.contains(&name) {
        //         return Err(anyhow!("Table \"{}\" contains self reference", name));
        //     }
        // }
        let sub_tables = if let Some(pair) = pairs.next() {
            Self::parse_tables(pair.into_inner())?
        } else {
            Tables {
                map: HashMap::new(),
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

    // fn find(&self, table_name: &String) -> Option<&Table> {
    //     if let Some(table) = self.map.get(table_name) {
    //         return Some(table);
    //     } else {
    //         let mut found = None;
    //         for (_, table) in self.map.iter() {
    //             found = table.sub_tables.find(table_name);
    //             if found.is_some() {
    //                 break;
    //             }
    //         }
    //         found
    //     }
    // }
}
