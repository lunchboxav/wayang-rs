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
  event_text: &'a str,
  link_texts: &'a Vec<Link>
}

#[derive(Debug)]
struct WygStory<'a> {
  title: &'a str,
  scenes: Vec<&'a str>,
  events: Vec<&'a str>,
  choices: Vec<Link>,
}

impl<'a> WygStory<'a> {
  pub fn new() -> Self {
    WygStory {
      title: &"",
      scenes: Vec::new(),
      events: Vec::new(),
      choices: Vec::new(),
    }
  }

  pub fn add_choices_from_vec_str(&mut self, v: Vec<&str>) {
    for links_pair in v.chunks(2) {
      self.choices.push(Link {
        text: links_pair[0].to_string(),
        anchor: links_pair[1].to_string(),
      });
    }
  }
}

fn main() -> std::io::Result<()>{
  let unparsed_file = fs::read_to_string("story/test.wyg")
    .expect("Unable to open file");
  
  let file = WYGParser::parse(Rule::file, &unparsed_file)
    .expect("unsuccessful parse")
    .next().unwrap();

  // let mut title = "";
  // let mut scene_vec = Vec::new();  
  // let mut event_vec = Vec::new();
  let mut temp_choice_vec = Vec::new();

  let mut story = WygStory::new();

  for record in file.into_inner() {
    match record.as_rule() {
      Rule::record => {
        for f in record.into_inner() {
          match f.as_rule() {
            Rule::meta_record => {
              // title = f.into_inner().as_str();
              story.title = f.into_inner().as_str();
            }
            Rule::scene_record => {
              for i in f.into_inner() {
                story.scenes.push(i.as_str());
              }
            }
            Rule::event_record => {
              for i in f.into_inner() {
                story.events.push(i.as_str());
              }
            }
            Rule::choice_record => {
              for i in f.into_inner() {
                temp_choice_vec.push(i.as_str());
              }
            }
            _ => unreachable!()
          }
        }
      }
      Rule::EOI => (),
      _ => unreachable!()
    }
  }

  // let links_vec = create_links_vector(temp_choice_vec);
  // story.choices = create_links_vector(temp_choice_vec);
  story.add_choices_from_vec_str(temp_choice_vec);

  let test_template = PageTemplate {
    title_text: story.title,
    scene_text: &story.scenes[0],
    event_text: &story.events[0],
    link_texts: &story.choices,
  };

  println!("For testing purpose");
  println!("title: {}", story.title);
  
  println!("{:?}", story.scenes);
  println!("there are {} scene records", story.scenes.len());

  println!("{:?}", story.events);
  println!("there are {} event records", story.events.len());

  println!("{:?}", story.choices);
  println!("there are {} links records", story.choices.len());

  let root_path = "result";
  let path = format!("{}/{}.html", root_path, story.title);
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

// fn write_html() {
//   let root_path = "result";
//   let path = format!("{}/{}.html",root_path,title);
//   let mut buffer = File::create(path)?;

//   buffer.write(test_template.call().unwrap().to_string().as_bytes())?;
//   Ok(())
// }
