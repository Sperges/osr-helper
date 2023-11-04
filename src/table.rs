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
