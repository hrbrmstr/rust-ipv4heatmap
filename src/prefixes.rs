//! # CIDR block prefix annotation

use crate::utils::bbox_from_cidr;
use crate::annotations::Prefix;

use ril::{Font, Image, Rgba, TextSegment, TextLayout, OverlayMode};

/// Given a vector of label annotations, draw the labels.
pub fn annotate_prefixes(img: &mut Image<Rgba>, prefixes: Vec<Prefix>) {
	
	let builtin_font = include_bytes!("Inconsolata-CondensedRegular.ttf") as &[u8];
	let builtin_font = Font::from_bytes(builtin_font, 128.0).expect("Error reading in default font");
	
	for prefix in prefixes {			
		
		let bbox = bbox_from_cidr(prefix.cidr.to_owned());	
		
		if bbox.width() >= 127 && bbox.height() >= 60 {
		
			let text = prefix.cidr.as_str().to_owned();
			let font = builtin_font.to_owned();

			let font_color = Rgba{r: 255, g:255, b:255, a:127};

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
