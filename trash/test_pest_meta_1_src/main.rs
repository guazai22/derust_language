extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "DeRust.pest"]
pub struct DeRustParser;

// use std::collections::HashMap;
use std::fs;

fn main() {
	let unparsed_file =
		fs::read_to_string("code_of_input.drs").expect("cannot read file");

	let file = DeRustParser::parse(Rule::file, &unparsed_file)
		.expect("unsuccessful parse") // unwrap the parse result
		.next()
		.unwrap(); // get and unwrap the `file` rule; never fails
		   // println!("{:#?}", file.as_str());

	// let mut output: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

	for line in file.into_inner() {
		println!("{:#?}: {:#?}", line.as_rule(), line.as_str());
		// match line.as_rule() {
		// 	Rule::fn_def => {
		// 		// let mut inner_rules = line.into_inner();
		// 		println!("{:#?}: {:#?}", line.as_rule(), line.as_str());
		// 		// println!("{:#?}", line);
		// 	}
		// 	_ => {}
		// }
	}
}
