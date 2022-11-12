use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::env::var;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// If provided, data will be written to this file.
    output_data: Option<PathBuf>,

    /// If provided, Rust source code with embedded data will be generated and written to this file.
    output_code: Option<PathBuf>,
}

fn main() {
  let cli = Cli::parse();

  let res = get("https://static.wilsonl.in/wiki/enwiktionary-20221101-all-titles-in-ns0.gz")
    .unwrap()
    .error_for_status()
    .unwrap()
    .bytes()
    .unwrap()
    .to_vec();
  eprintln!("Downloaded compressed file");

  let mut d = GzDecoder::new(&res[..]);
  let mut lines = String::new();
  d.read_to_string(&mut lines).unwrap();
  eprintln!("Decompressed file");
  
  let mut builder = fastrie::FastrieBuilderNode::new(fastrie::IndexWidth(4));
  for (i, line) in lines.split('\n').enumerate() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    };
    // TODO Maybe exclude non-English words?
    // TODO Remove typo/misspelling entries, stubs, and redirects.
    if line.chars().any(|c| match c {
      '-' | '?' | '!' | '_' | '"' | ' ' | '\\' | '^' => true,
      _ => false,
    }) {
      continue;
    };
    builder.add(line.as_bytes(), true);
    if i % 100000 == 0 {
      eprintln!("Processed entry {i}");
    };
  };
  eprintln!("Building trie");
  let build = builder.prebuild();

  if let Some(path) = cli.output_data {
    eprintln!("Writing data with {} bytes", build.data.len());
    File::create(path).unwrap().write_all(&build.data).unwrap();
  }

  if let Some(path) = cli.output_code {
    eprintln!("Generating source code with {} bytes", build.data.len());
    // Either generate code in memory or use a buffered writer on the file, do not write and flush each character to the file individually.
    // A single byte string literal is easier to parse than millions of individual u8 literal elements.
    let mut code = b"const DATA: &[u8] = \"".to_vec();
    for b in build.data {
      match b {
        b'\0' => code.extend_from_slice(b"\\0"),
        b'\n' => code.extend_from_slice(b"\\n"),
        b'\r' => code.extend_from_slice(b"\\r"),
        b'\t' => code.extend_from_slice(b"\\t"),
        b'"' => code.extend_from_slice(b"\\\""),
        b'\\' => code.extend_from_slice(b"\\\\"),
        b if b >= 32 && b <= 126 => code.push(b),
        b => {
          code.extend_from_slice(b"\\x");
          write!(code, "{:02x}", b).unwrap();
        }
      };
    };
    code.extend_from_slice(b"];\n\n");
    code.extend_from_slice(b"pub const ENTRIES: fastrie::Fastrie<'static, 'static, ()> = fastrie::from_prebuilt_without_values(fastrie::IndexWidth(4), DATA);");
    eprintln!("Generated source code");
    let mut out = File::create(path).unwrap();
    out.write_all(&code).unwrap();
    out.flush().unwrap();
    eprintln!("Wrote source code");
  }
}
