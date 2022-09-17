//! # CIDR block outlne annotation

use crate::utils::bbox_from_cidr;
use crate::annotations::Outline;

use hex_color::HexColor;

use ril::{Border, Rectangle, Rgba, Image};

/// Given a vector of CIDRs, draw a border around them.
pub fn outline_cidrs(img: &mut Image<Rgba>, outlines: Vec<Outline>) {
	
	for outline in outlines {

		let bbox = bbox_from_cidr(outline.cidr);
					
		if bbox.width() > 1 && bbox.height() > 1 {
  		let stroke = HexColor::parse_rgba(&outline.color.as_str()).expect("Invalid hex color in shade file.");
			let border = Border::new(Rgba{r:stroke.r, g:stroke.g, b:stroke.b, a:stroke.a}, 1);
			let rect  = Rectangle::new()
					.with_position(bbox.x() as u32, bbox.y() as u32)
					.with_size(bbox.width() as u32, bbox.height() as u32)
					.with_border(border);
			
			img.draw(&rect);
		}
		
	}
	
}
