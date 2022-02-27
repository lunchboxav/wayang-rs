extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;
use std::fs;
use std::io;

use yarte::Template;

use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
#[grammar = "wyg.pest"]
pub struct WYGParser;

#[derive(Debug)]
struct Link {
  text: String,
  anchor: String,
}

#[derive(Template)]
#[template(path = "main-template")]
struct PageTemplate<'a> {
  title_text: &'a str,
  scene_text: &'a str,
  event_text: &'a str,
  link_texts: &'a Vec<Link>,
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

  fn from_parsed_file(parsed_file: Pair<Rule>) -> WygStory {
    let mut choices = Vec::new();

    let mut story = WygStory::new();

    for record in parsed_file.into_inner() {
      match record.as_rule() {
        Rule::record => {
          for f in record.into_inner() {
            match f.as_rule() {
              Rule::meta_record => {
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
                  choices.push(i.as_str());
                }
              }
              _ => unreachable!(),
            }
          }
        }
        Rule::EOI => (),
        _ => unreachable!(),
      }
    }
    story.add_choices_from_vec_str(choices);

    story
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

fn main() -> std::io::Result<()> {
  let mut file_paths = fs::read_dir("story")?
    .map(|res| res.map(|f| f.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;
  file_paths.sort();

  for file_path in file_paths.iter() {
    // This is String version of the .wyg file.
    let raw_file = fs::read_to_string(file_path).expect("Unable to open file");

    // This is the parsed form of the String.
    let parsed_file = WYGParser::parse(Rule::file, &raw_file)
      .expect("Unsuccessful parse")
      .next()
      .unwrap();

    // This is the struct that holds values acquired from the parsed String.
    // We can use the values by referencing the field, for example story.title.
    let story = WygStory::from_parsed_file(parsed_file);
    println!("{:?}", story);

    println!("Parsing results:");
    println!("title: {}", story.title);
    println!("{} scenes", story.scenes.len());
    println!("{:?}", story.scenes);

    println!("{} events", story.events.len());
    println!("{:?}", story.events);
    
    println!("{} links", story.choices.len());
    println!("{:?}", story.choices);
    
    // Finally we use the values stored in story to generate HTML files.
    let root_path = "result";
    let path = format!("{}/{}.html", root_path, story.title);
    let buffer = File::create(path)?;

    write_html(story, buffer);
  }

  Ok(())
}

fn write_html(story: WygStory, mut buffer: std::fs::File) {
  let html_template = PageTemplate {
    title_text: story.title,
    scene_text: &story.scenes[0],
    event_text: &story.events[0],
    link_texts: &story.choices,
  };

  buffer
    .write(html_template.call().unwrap().to_string().as_bytes())
    .unwrap();
}

