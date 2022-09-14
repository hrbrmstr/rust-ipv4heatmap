use crate::utils::bbox_from_cidr;
use crate::annotations::Outline;

use hex_color::HexColor;

use image::{ImageBuffer, Rgba};

use imageproc::rect::Rect;
use imageproc::drawing::draw_hollow_rect_mut;

pub fn outline_cidrs(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, outlines: Vec<Outline>) {
	
	for outline in outlines {

		let bbox = bbox_from_cidr(outline.cidr);
		let fill = HexColor::parse_rgba(&outline.color.as_str()).expect("Invalid hex color in shade file.");
					
		// println!("cidr: {:?} bbox: {:?}", fields[0], bbox);

		draw_hollow_rect_mut(
			img, 
			Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height()),
			Rgba([fill.r, fill.g, fill.b, fill.a])
		);
		
	}
	
}
