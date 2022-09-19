//! # CIDR block prefix annotation

use crate::utils::bbox_from_cidr;
use crate::annotations::Prefix;
use anyhow::{Context, Result};

use hex_color::HexColor;

use ril::{Font, Image, Rgba, TextSegment, TextLayout, OverlayMode};

const PREFIX_DEFAULT_COLOR: Rgba = Rgba{r: 255, g:255, b:255, a:127};

/// Given a vector of label annotations, draw the labels.
pub fn annotate_prefixes(img: &mut Image<Rgba>, prefixes: Option<Vec<Prefix>>) -> Result<()> {
	
	if let Some(prefixes) = prefixes {
		
		let builtin_font: ril::Font = Font::from_bytes(
			include_bytes!("Inconsolata-CondensedRegular.ttf") as &[u8], 
			128.0
		)
		.expect("Error loading builtin font.");
		
		for prefix in prefixes {			
			
			let bbox = bbox_from_cidr(prefix.cidr.to_owned());	
			
			if bbox.width() >= 127 && bbox.height() >= 60 {
				
				let text = prefix.cidr.as_str().to_owned();
				let font = builtin_font.to_owned();
				
				let font_color = if let Some(color) = prefix.color {
					let stroke = HexColor::parse_rgba(color.as_str()).context("Invalid prefix hex color in annotations file.")?;
					Rgba{r:stroke.r, g:stroke.g, b:stroke.b, a:stroke.a}
				} else {
					PREFIX_DEFAULT_COLOR
				};
				
				let size = 24.0;
				
				let cx = bbox.xmin + (bbox.width()/2);
				let by = bbox.ymax - (size as u32) - 1;
				
				let segment = 
				TextSegment::new(&font, text, font_color.to_owned())
				.with_size(size)
				.with_overlay_mode(OverlayMode::Merge);
				
				let layout = TextLayout::new()
				.centered()
				.with_position(cx, by)
				.with_segment(&segment);
				
				img.draw(&layout);
				
			}
			
		}
		
	}
	
	Ok(())
	
}
