use std::fs::File;
use std::io::prelude::*;

fn u8_to_u32(x: [u8; 4]) -> u32 {
  let mut y: u32 = 0;
  y = (y | (x[0] as u32)) << 24;
  y = y | ((0 | (x[1] as u32)) << 16);
  y = y | ((0 | (x[2] as u32)) << 8);
  y = y | ((0 | (x[3] as u32)));
  y
}

fn read_magic(mut file: File) -> u32 {
  let mut magic_bytes = [0u8; 4];
  file.read(&mut magic_bytes)
      .expect("something went wrong reading the file");
  u8_to_u32(magic_bytes)
}

fn main() {
  let filename = "hello-world";
  let f = File::open(filename).expect("file not found"); // TODO: WHAT IS expect?
  let magic = read_magic(f);
  println!("{}", magic);
}
