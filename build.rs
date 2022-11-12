use std::{fs::File, io::{BufReader, Write, BufRead}, env::var};

fn main() {
  let file = File::open("entries.txt").unwrap();
  let reader = BufReader::new(file);
  let mut out = File::create(format!("{}/entries.rs", var("OUT_DIR").unwrap())).unwrap();
  out.write_all(b"pub const ENTRIES: &[&str] = &[").unwrap();
  for line in reader.lines() {
    let line = line.unwrap();
    // TODO Maybe exclude non-English words?
    // TODO Remove typo entries.
    if line.chars().any(|c| match c {
      '-' | '?' | '!' | '_' | '"' | ' ' | '\\' | '^' => true,
      _ => false
    }) {
      continue;
    }
    write!(out, "  \"{}\",\n", line).unwrap();
  }
  out.write_all(b"];\n").unwrap();
}
