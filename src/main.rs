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

  let mut event_vec = Vec::new();

  for record in file.into_inner() {
    match record.as_rule() {
      Rule::event_record => {
        for field in record.into_inner() {
          event_vec.push(field.as_str().parse::<String>().unwrap());
          //println!("{}", field.as_str().parse::<String>().unwrap());
        }
      }
      Rule::EOI => (),
      _ => unreachable!()
    }
  }

  println!("{:?}", event_vec);
}
