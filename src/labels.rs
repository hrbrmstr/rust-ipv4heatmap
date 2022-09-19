//! # CIDR block text wlabel annotation
use anyhow::{Context, Result};

use crate::utils::bbox_from_cidr;
use crate::annotations::Label;

use hex_color::HexColor;

use ril::{Font, Image, Rgba, TextSegment, TextLayout, OverlayMode};

/// Given a vector of label annotations, draw the labels.
pub fn annotate_cidrs(img: &mut Image<Rgba>, labels: Vec<Label>) -> Result<()> {

	let builtin_font: ril::Font = Font::from_bytes(
		include_bytes!("Inconsolata-CondensedRegular.ttf") as &[u8], 
		128.0
	)
	.expect("Error loading builtin font.");
		
	for label in labels {
		
		let bbox = bbox_from_cidr(label.cidr);
		let color = HexColor::parse_rgba(label.color.as_str())
		  .context("Invalid hex color in shade file.")?;
		
		let text = label.label.as_str();
		let font = match label.font {
			Some(f) => {
        if let Ok(font) = Font::open(&f, 64.0) {
					font
				} else {
					eprintln!("{} not found", f);
					builtin_font.to_owned()
				}
			},
			None => builtin_font.to_owned()
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
  
	Ok(())
	
}
