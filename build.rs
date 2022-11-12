use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::env::var;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
  let res = get("https://dumps.wikimedia.org/enwiktionary/20221101/enwiktionary-20221101-all-titles-in-ns0.gz")
    .unwrap()
    .error_for_status()
    .unwrap()
    .bytes()
    .unwrap()
    .to_vec();
  let mut d = GzDecoder::new(&res[..]);
  let mut lines = String::new();
  d.read_to_string(&mut lines).unwrap();
  let mut out = File::create(format!("{}/entries.rs", var("OUT_DIR").unwrap())).unwrap();
  out.write_all(b"pub const ENTRIES: &[&str] = &[").unwrap();
  for line in lines.split('\n') {
    let line = line.trim();
    if line.is_empty() {
      continue;
    };
    // TODO Maybe exclude non-English words?
    // TODO Remove typo entries.
    if line.chars().any(|c| match c {
      '-' | '?' | '!' | '_' | '"' | ' ' | '\\' | '^' => true,
      _ => false,
    }) {
      continue;
    };
    write!(out, "  \"{}\",\n", line).unwrap();
  };
  out.write_all(b"];\n").unwrap();
}
