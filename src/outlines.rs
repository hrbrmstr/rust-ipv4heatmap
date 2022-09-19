//! # CIDR block outlne annotation

use crate::utils::bbox_from_cidr;
use crate::annotations::Outline;
use anyhow::{Context, Result};

use hex_color::HexColor;

use ril::{Border, Rectangle, Rgba, Image};

/// Given a vector of CIDRs, draw a border around them.
pub fn outline_cidrs(img: &mut Image<Rgba>, outlines: Option<Vec<Outline>>) -> Result<()> {
	
	if let Some(outlines) = outlines {
		for outline in outlines {
			
			let bbox = bbox_from_cidr(&outline.cidr);
			
			if bbox.width() > 1 && bbox.height() > 1 {
				
				let stroke = HexColor::parse_rgba(outline.color.as_str())
				.context("Invalid outline hex color in annotations file.")?;
				
				let border = Border::new(
					Rgba{r:stroke.r, g:stroke.g, b:stroke.b, a:stroke.a}, 
					1
				).with_position(
					if bbox.ymin == 0 || bbox.xmin == 0 {
						ril::BorderPosition::Inset
					} else {
						ril::BorderPosition::Outset
					}
				);
				
				let rect  = Rectangle::new()
				.with_position(bbox.x() as u32, bbox.y() as u32)
				.with_size(bbox.width() as u32, bbox.height() as u32)
				.with_border(border);
				
				img.draw(&rect);
			}
			
		}
		
	}

	Ok(())
	
}
