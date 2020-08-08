extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

use pest::Parser;

#[derive(Parser)]
#[grammar = "wyg.pest"]
pub struct WYGParser;

fn main() {
  let unparsed_file = fs::read_to_string("story/test.wyg").expect("Unable to open file");
  
  let file = WYGParser::parse(Rule::file, &unparsed_file)
    .expect("unsuccessful parse")
    .next().unwrap();
  
  let mut record_count: u64 = 0;

  for record in file.into_inner() {
    match record.as_rule() {
      Rule::record => {
        record_count += 1;

        for field in record.into_inner() {
          println!("{}", field.as_str().parse::<String>().unwrap());
        }
      }
      Rule::EOI => (),
      _ => unreachable!()
    }
  }

  println!("Number of records: {}", record_count);
}
