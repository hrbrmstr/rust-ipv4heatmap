use std::cmp::{min, max};
use std::str::FromStr;
use std::net::Ipv4Addr;
use cidr::Ipv4Cidr;

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
	
	let s = ip_as_int >> 8; // we're only supporting full maps
	
	while i >= 0 {
		
		row = 4 * state | ((s >> i) & 3);
		x = (x << 1) | ((0x936C >> row) & 1);
		y = (y << 1) | ((0x39C6 >> row) & 1);
		state = (0x3E6B94C1 >> 2 * row) & 3;
		
		i = i - 2;
		
	}
	
	(x, y)
	
}

#[derive(Debug, PartialEq)]
pub struct BoundingBox {
	pub xmin: u32,
	pub xmax: u32,
	pub ymin: u32,
	pub ymax: u32,
}

impl BoundingBox {
	
	pub fn x(&self) -> i32 { self.xmin as i32 }
	pub fn y(&self) -> i32 { self.ymin as i32 }
	pub fn width(&self) -> u32 { self.xmax - self.xmin }
	pub fn height(&self) -> u32 { self.ymax - self.ymin }
	
}

fn bbox(first: u32, slash: u8) -> BoundingBox {
	
	let mut diag: u32 = 0xAAAAAAAA;
	
	if slash > 31 { // special case of one point
		
		let (xmin, ymin) = hil_xy_from_s(first, 12);
		
		BoundingBox{ xmin, xmax: xmin, ymin, ymax: ymin }
		
	} else if 0 == (slash & 1) { // square
		
		diag >>= slash;
		
		let (x1, y1) = hil_xy_from_s(first, 12);
		let (x2, y2) = hil_xy_from_s(first + diag, 12);
		
		BoundingBox{ 
			xmin: min(x1, x2),
			xmax: max(x1, x2), 
			ymin: min(y1, y2), 
			ymax: max(y1, y2) 
		}
		
	} else { // rect (split into squares)
		
		let bbox1 = bbox(first, slash + 1);
		let bbox2 = bbox(first + (1 << (32 - (slash + 1))), slash + 1);
		
		BoundingBox{ 
			xmin: min(bbox1.xmin, bbox2.xmin), 
			xmax: max(bbox1.xmax, bbox2.xmax), 
			ymin: min(bbox1.ymin, bbox2.ymin), 
			ymax: max(bbox1.ymax, bbox2.ymax), 
		}
		
	}
	
}

pub fn bbox_from_cidr(cidr: String) -> BoundingBox {
	
	if let Ok(parsed_cidr) = Ipv4Cidr::from_str(cidr.as_str()) {
		
		let first: u32 = parsed_cidr.first_address().into();
		let slash: u8 = parsed_cidr.network_length();
		
		return bbox(first, slash)
		
	}
	
	BoundingBox{ xmin:0, xmax:0, ymin:0, ymax:0 }
	
}

#[cfg(test)]

#[test]
fn test_hil_xy_from_s() {
	let result = self::hil_xy_from_s(3232235777, 12);
	assert_eq!(result, (3871, 1822));
}

#[test]
fn test_ip_to_numeric() {
	let result = self::ip_to_numeric(String::from("192.168.1.1"));
	assert_eq!(result, 3232235777);
}

#[test]
fn test_read_lines() {
	if let Ok(_result) = self::read_lines("Cargo.toml") {
		assert!(true);
	} else {
		assert!(false);
	}
}

#[test]
fn test_bbox_from_cidr() {
	let result = self::bbox_from_cidr(String::from("218.0.0.0/7"));
	assert_eq!(result, BoundingBox { xmin: 2048, xmax: 2559, ymin: 1024, ymax: 1279 });
	let result = self::bbox_from_cidr(String::from("217.0.0.0/8"));
	assert_eq!(result, BoundingBox { xmin: 2048, xmax: 2303, ymin: 1280, ymax: 1535 });
}