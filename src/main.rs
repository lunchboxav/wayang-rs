use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut data = String::new();
  let mut f = File::open("story/story.wyg").expect("Unable to open file");
  f.read_to_string(&mut data).expect("Unable to read data");

  let v: Vec<&str> = data.split("  \n").collect();

  let v_scene: Vec<&str> = v[0].split("\n")
    .into_iter()
    .filter(|s| !s.contains("Scene"))
    .filter(|s| !s.is_empty())
    .collect();
  let v_event: Vec<&str> = v[1].split("\n")
    .into_iter()
    .filter(|s| !s.contains("Event"))
    .filter(|s| !s.is_empty())
    .collect();
  let v_branch: Vec<&str> = v[2].split("\n")
    .into_iter()
    .filter(|s| !s.contains("Branch"))
    .filter(|s| !s.is_empty())
    .collect();

  println!("{:?}", v_scene);
  println!("{:?}", v_event);
  println!("{:?}", v_branch);



}
