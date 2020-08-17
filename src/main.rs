extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;

use yarte::Template;

use std::io::prelude::*;
use std::fs::File;
#[derive(Parser)]
#[grammar = "wyg.pest"]
pub struct WYGParser;

#[derive(Debug)]
struct Link {
  text: String,
  anchor: String
}

#[derive(Template)]
#[template(path="test")]
struct PageTemplate<'a> {
  title_text: &'a str,
  scene_text: &'a str,
  event_text: &'a str
}

fn main() -> std::io::Result<()>{
  let unparsed_file = fs::read_to_string("story/test.wyg")
    .expect("Unable to open file");
  
  let file = WYGParser::parse(Rule::file, &unparsed_file)
    .expect("unsuccessful parse")
    .next().unwrap();

  let mut title = "";
  let mut scene_vec = Vec::new();  
  let mut event_vec = Vec::new();
  let mut temp_choice_vec = Vec::new();

  for record in file.into_inner() {
    match record.as_rule() {
      Rule::record => {
        for f in record.into_inner() {
          let rule = f.as_rule();
          if rule == Rule::meta_record {
            title = f.into_inner().as_str();
          } else if rule == Rule::scene_record {
            for i in f.into_inner() {
              scene_vec.push(i.as_str());
            }
          } else if rule == Rule::event_record {
            for i in f.into_inner() {
              event_vec.push(i.as_str());
            }
          } else if rule == Rule::choice_record {
            for i in f.into_inner() {
              temp_choice_vec.push(i.as_str());
            }
          }
        }
      }
      Rule::EOI => (),
      _ => unreachable!()
    }
  }

  let links_vec = create_links_vector(temp_choice_vec);

  let test_template = PageTemplate {
    title_text: title,
    scene_text: &scene_vec[0],
    event_text: &event_vec[0]
  };

  // println!("{:?}",test_template.call().unwrap().to_string());

  println!("title: {}", title);
  
  println!("{:?}", scene_vec);
  println!("there are {} scene records", scene_vec.len());

  println!("{:?}", event_vec);
  println!("there are {} event records", event_vec.len());

  println!("{:?}", links_vec);
  println!("there are {} links records", links_vec.len());

  let root_path = "result";
  let path = format!("{}/{}.html",root_path,title);
  let mut buffer = File::create(path)?;

  buffer.write(test_template.call().unwrap().to_string().as_bytes())?;
  Ok(())
}

fn create_links_vector(v: Vec<&str>) -> Vec<Link> {
  let mut temp_v = Vec::new();
  let mut result_v = Vec::new();

  for links_pair in v.chunks(2) {
    temp_v.push(links_pair);
  }

  for links in temp_v {
    let link: Link = Link {
      text: links[0].to_string(),
      anchor: links[1].to_string()
    };
    result_v.push(link);
  }

  result_v
}
