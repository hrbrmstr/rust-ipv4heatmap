//! # CIDR block text wlabel annotation

use crate::utils::bbox_from_cidr;
use crate::annotations::Label;
use crate::fonts::{ load_font, FontCache, BUILTIN_FONT};

use anyhow::{Context, Result};

use hex_color::HexColor;

use ril::{Image, Rgba, TextSegment, TextLayout, OverlayMode};

/// Given a vector of label annotations, draw the labels.
pub fn annotate_cidrs(img: &mut Image<Rgba>, labels: Option<Vec<Label>>) -> Result<()> {
	
	if let Some(labels) = labels {

		// We allow the specification of fonts to use for each label; it's likely folks will end up 
		// using the same font multiple times and it's silly to load it each time, so we cache them as
		// we encounter them to save time at the expense of some memory.
		let mut cache: FontCache = FontCache::new();
		
		for label in labels {
			
			let bbox = bbox_from_cidr(label.cidr);
			let color = HexColor::parse_rgba(label.color.as_str())
			.context("Invalid hex color in shade file.")?;
			
			let text = label.label.as_str();
			let font: ril::Font = match label.font.to_owned() {
				Some(label_font) => cache
																			.entry(label_font.to_owned())
																			.or_insert_with(|| load_font(Some(label_font))).to_owned(),
				None => BUILTIN_FONT.font.to_owned()
			};
			
			let font_color = Rgba{r: color.r, g:color.g, b:color.b, a:color.a};
			
			let cx = bbox.xmin + (bbox.width()/2);
			let cy = bbox.ymin + (bbox.height()/2);
			
			let bw = bbox.width();
			let bh = bbox.height();
			
			let mut size = 256.0;
			
			let mut segment = 
			TextSegment::new(&font, text, font_color.to_owned())
			.with_size(size)
			.with_overlay_mode(OverlayMode::Merge);
			
			let mut layout = TextLayout::new()
			.centered()
			.with_position(cx, cy)
			.with_segment(&segment);
			
			let (mut w, mut h) = layout.dimensions();
			
			while (w > bw) || (h > bh) {
				
				size *= 0.95;
				segment.size = size;
				
				layout = TextLayout::new()
				.centered()
				.with_position(cx, cy)
				.with_segment(&segment);
				
				(w, h) = layout.dimensions();
				
			}
			
			img.draw(&layout);
			
		}
		
	}
	
	Ok(())
	
}
