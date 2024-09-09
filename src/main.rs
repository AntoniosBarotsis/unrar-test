#![allow(clippy::unwrap_used)]

use std::path::Path;

use unrar::Archive;

fn main() {
  let output_folder = Path::new("test");
  let mut archive = Archive::new("test.rar").open_for_processing().unwrap();
  let mut unpacked = 0;

  while let Some(header) = archive.read_header().unwrap() {
    let entry = header.entry();
    archive = if entry.is_file() {
      unpacked += 1;
      header.extract_with_base(output_folder).unwrap()
    } else {
      header.skip().unwrap()
    }
  }

  println!("{unpacked}");
}
