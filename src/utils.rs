use std::str::FromStr;
use std::net::Ipv4Addr;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

pub fn ip_to_numeric(ip: String) -> u32 {
	let addr = Ipv4Addr::from_str(&ip).expect("Invalid IPv4");
  let addr_u32: u32 = addr.into();
	addr_u32
}

pub fn hil_xy_from_s(ip_as_int: u32, order: i16) -> (u32, u32) {
	
	let mut i: i16;
	let mut state: u32 = 0;
	let mut x: u32 = 0;
	let mut y: u32 = 0;
	let mut row: u32;
	
	i = 2 * order - 2;
	
	let s = ip_as_int >> 8;
	
	while i >= 0 {
		
		row = 4 * state | ((s >> i) & 3);
		x = (x << 1) | ((0x936C >> row) & 1);
		y = (y << 1) | ((0x39C6 >> row) & 1);
		state = (0x3E6B94C1 >> 2 * row) & 3;
		
		i = i - 2;
		
	}
	
	(x, y)
	
}