#![allow(dead_code)]

use std::cmp::{min, max};
use std::str::FromStr;
use std::net::Ipv4Addr;
use cidr::Ipv4Cidr;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use hex_color::HexColor;

use image::{ImageBuffer, GenericImage, Pixel};
use image::Rgba;

use imageproc::rect::Rect;
use imageproc::drawing::draw_hollow_rect_mut;

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
		
		BoundingBox{ xmin, xmax:xmin, ymin, ymax:ymin }
		
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
			ymax: max(bbox1.ymax, bbox2.xmax), 
		}
		
	}
	
}

pub fn bbox_from_cidr(cidr: String) -> BoundingBox {
	
	if let Ok(parsed_cidr) = Ipv4Cidr::from_str(cidr.as_str()) {
		
		let first: u32 = parsed_cidr.first_address().into();
		let _last: u32 = parsed_cidr.last_address().into();
		let slash: u8 = parsed_cidr.network_length();
		
		return bbox(first, slash)
		
	}
	
	BoundingBox{ xmin:0, xmax:0, ymin:0, ymax:0 }
	
}

pub fn draw_blended_rect_mut<I>(image: &mut I, rect: Rect, color: I::Pixel) 
  where I: GenericImage, I::Pixel: 'static, {

	let image_bounds = Rect::at(0, 0).of_size(image.width(), image.height());

	if let Some(intersection) = image_bounds.intersect(rect) {
		for dy in 0..intersection.height() {
			for dx in 0..intersection.width() {
				let x = intersection.left() as u32 + dx;
				let y = intersection.top() as u32 + dy;
				let mut pixel = image.get_pixel(x, y); 
				pixel.blend(&color);
				unsafe {
					image.unsafe_put_pixel(x, y, pixel);
				}
			}
		}
	}

}

pub fn outline_cidrs(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, outlines_file: &str) {
	
	if let Ok(lines) = read_lines(outlines_file) {
		for line in lines {
			
			if let Ok(record) = line {
				
				let fields: Vec<&str> = record.split("\t").collect();
				
				if fields.len() == 2 {
					
					let bbox = bbox_from_cidr(fields[0].to_string());
					let fill = HexColor::parse_rgba(fields[1]).expect("Invalid hex color in shade file.");
					
					draw_hollow_rect_mut(
						img, 
						Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height()),
						Rgba([fill.r, fill.g, fill.b, fill.a])
					);
					
				}
				
			}
			
		}
		
	}
	
}

pub fn shade_cidrs(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, shades_file: &str) {
	
	if let Ok(lines) = read_lines(shades_file) {
		for line in lines {
			
			if let Ok(record) = line {
				
				let fields: Vec<&str> = record.split("\t").collect();
				
				if fields.len() == 2 {
					
					let bbox = bbox_from_cidr(fields[0].to_string());
					let fill = HexColor::parse_rgba(fields[1]).expect("Invalid hex color in shade file.");
					
					draw_blended_rect_mut(
						img, 
						Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height()),
						Rgba([fill.r, fill.g, fill.b, fill.a])
					);
					
				}
				
			}
			
		}
		
	}
	
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