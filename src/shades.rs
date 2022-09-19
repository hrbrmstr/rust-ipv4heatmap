//! # CIDR block fill annotation

use crate::utils::bbox_from_cidr;
use crate::annotations::Shade;

use anyhow::{Context, Result};

use hex_color::HexColor;

use ril::{Rectangle, Rgba, OverlayMode, Image};

/// Given a vector of CIDRs, shade them in.
pub fn shade_cidrs(img: &mut Image<Rgba>, shades: Option<Vec<Shade>>) -> Result<()> {
	
	if let Some(shades) = shades {
		
		for shade in shades {
			
			let bbox = bbox_from_cidr(&shade.cidr);
			let fill = HexColor::parse_rgba(shade.fill.as_str()).context("Invalid hex color in shade file.")?;
			
			let fill = Rgba{r:fill.r, g:fill.g, b:fill.b, a:fill.a};
			
			if bbox.width() <= 1 || bbox.height() <= 1 {
				img.set_pixel(bbox.x() as u32, bbox.y() as u32, fill)
			} else {
				let rect  = Rectangle::new()
				.with_position(bbox.x() as u32, bbox.y() as u32)
				.with_size(bbox.width() as u32, bbox.height() as u32)
				.with_fill(fill)
				.with_overlay_mode(OverlayMode::Merge);
				
				img.draw(&rect);
			}
			
		}
		
	}
	
	Ok(())
	
}