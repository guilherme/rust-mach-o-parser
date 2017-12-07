use std::fs::File;
use std::io::prelude::*;

const MH_CIGAM: u32 = 0xcdfaedfe;
const MH_CIGAM_64: u32 = 0xcffaedfe;
const MH_MAGIC: u32 = 0xfeedface;
const MH_MAGIC_64: u32 = 0xfeedfacf;

fn u8_to_u32(x: [u8; 4]) -> u32 {
  let mut y: u32 = 0;
  y = y | ((0 | x[0] as u32) << 24);
  y = y | ((0 | (x[1] as u32)) << 16);
  y = y | ((0 | (x[2] as u32)) << 8);
  y = y | ((0 | (x[3] as u32)));
  y
}

fn should_swap(magic: u32) -> bool {
  return magic == MH_CIGAM || magic == MH_CIGAM_64;
}

fn is_64(magic: u32) -> bool {
  return magic == MH_CIGAM_64 || magic == MH_MAGIC_64;
}

fn read_magic(file: &mut File) -> u32 {
  let mut magic_bytes = [0u8; 4];
  file.read(&mut magic_bytes)
      .expect("something went wrong reading the file");
  u8_to_u32(magic_bytes)
}

fn main() {
  let filename = "hello-world";
  let f = File::open(filename).expect("file not found");
  dump_segments(f);
}

fn dump_mach_header(f: &mut File, is_64: bool, should_swap: bool) {
  // TODO: dump header

}

fn dump_segments(mut f: File) {
  let magic = read_magic(&mut f);
  let is_64 = is_64(magic);
  let should_swap = should_swap(magic);
  dump_mach_header(&mut f, is_64, should_swap);
}
