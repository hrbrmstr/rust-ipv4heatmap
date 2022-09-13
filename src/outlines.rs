use crate::utils::{ read_lines, bbox_from_cidr };

use hex_color::HexColor;

use image::{ImageBuffer, Rgba};

use imageproc::rect::Rect;
use imageproc::drawing::draw_hollow_rect_mut;

pub fn outline_cidrs(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, outlines_file: &str) {
	
	if let Ok(lines) = read_lines(outlines_file) {
		for line in lines {
			
			if let Ok(record) = line {
				
				let fields: Vec<&str> = record.split("\t").collect();
				
				if fields.len() == 2 {
					
					let bbox = bbox_from_cidr(fields[0].to_string());
					let fill = HexColor::parse_rgba(fields[1]).expect("Invalid hex color in shade file.");
					
					// println!("cidr: {:?} bbox: {:?}", fields[0], bbox);

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
