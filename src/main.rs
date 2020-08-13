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
  let mut choice_vec = Vec::new();

  for record in file.into_inner() {
    match record.as_rule() {
      Rule::record => {
        for f in record.into_inner() {
          let rule = f.as_rule();
          if rule == Rule::event_record {
            for i in f.into_inner() {
              event_vec.push(i.as_str());
            }
          } else if rule == Rule::choice_record {
            for i in f.into_inner() {
              choice_vec.push(i.as_str());
            }
          }
          
        }
      }
      Rule::EOI => (),
      _ => unreachable!()
    }
  }

  println!("{:?}", event_vec);
  println!("there are {} event records", event_vec.len());

  println!("{:?}", choice_vec);
  println!("there are {} event records", choice_vec.len());
}
