use crate::colors::legend_cols;
use std::cmp::{min, max};
use std::str::FromStr;
use std::net::Ipv4Addr;
use cidr::Ipv4Cidr;
use std::io::{Write};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn output_legend<P, S>(filename: P, name: S, invert: bool) where P: AsRef<Path>, S: Into<String>, {

	let mut cols = legend_cols(name);
  if invert { cols.reverse() };

 let res = format!(r#"
	<svg class="hilbert-legend" width="340" height="70" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
			<defs>
					<style>
					svg.hilbert-legend {{ padding-top: 10pt; }}
					.hilbert-legend-domain, .hilbert-legend-tick, line {{ stroke: black; opacity: 1; }} 
					.hilbert-legend-axis {{ fill: none; font-size: 8pt; font-family: sans-serif; text-anchor: middle; }}
					.hilbert-legend-axis-text {{ fill: black; font-family: sans-serif; font-size: 8pt; font-weight: 300; }}
					.hilbert-legend-title {{font-family: sans-serif; text-anchor: start; font-size: 10pt; fill: black; font-weight: 700; }}
					</style>
					<linearGradient id="hilbert-legend-bar">
							<stop offset="0" stop-color="{}" />
							<stop offset="0.125" stop-color="{}" />
							<stop offset="0.25" stop-color="{}" />
							<stop offset="0.375" stop-color="{}" />
							<stop offset="0.5" stop-color="{}" />
							<stop offset="0.625" stop-color="{}" />
							<stop offset="0.75" stop-color="{}" />
							<stop offset="0.875" stop-color="{}" />
							<stop offset="1" stop-color="{}" />
					</linearGradient>
			</defs>
			<g><text class="hilbert-legend-title" x="10" y="10" style="fill: black">Addresses per-pixel</text></g>
			<g>
					<rect width="300" height="20" transform="translate(10,16)" style="fill: url(&quot;#hilbert-legend-bar&quot;);" />
			</g>
			<g class="hilbert-legend-axis" transform="translate(10,40)">
					<path class="hilbert-legend-domain"  d="M0,6V0H300V6" />
					<g class="hilbert-legend-tick" transform="translate(0,0)">
							<line y2="6" /><text class="hilbert-legend-axis-text" y="9" dy="0.71em">1</text></g>
					<g class="hilbert-legend-tick" opacity="1" transform="translate(150.58823529411765,0)">
							<line y2="6" /><text class="hilbert-legend-axis-text" y="9" dy="0.71em">128</text></g>
					<g class="hilbert-legend-tick" opacity="1" transform="translate(300,0)">
							<line y2="6" /><text class="hilbert-legend-axis-text" y="9" dy="0.71em">255</text></g>
			</g>
	</svg>
	"#, cols[0], cols[1], cols[2], cols[3], cols[4], cols[5], cols[6], cols[7], cols[8]);

	let mut output = File::create(filename).expect("Error opening legend file for writing.");
  write!(output, "{}", res).expect("Error writing legend file.");

}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

pub fn ip_to_numeric<S>(ip: S) -> u32 where S: Into<String>, {
	let addr = Ipv4Addr::from_str(&ip.into()).expect("Invalid IPv4");
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

pub fn bbox_from_cidr<S>(cidr: S) -> BoundingBox where S: Into<String>, {
	
	if let Ok(parsed_cidr) = Ipv4Cidr::from_str(&cidr.into()) {
		
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
	let result = self::ip_to_numeric("192.168.1.1");
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
	let result = self::bbox_from_cidr("218.0.0.0/7");
	assert_eq!(result, BoundingBox { xmin: 2048, xmax: 2559, ymin: 1024, ymax: 1279 });
	let result = self::bbox_from_cidr("217.0.0.0/8");
	assert_eq!(result, BoundingBox { xmin: 2048, xmax: 2303, ymin: 1280, ymax: 1535 });
}