use crate::utils::bbox_from_cidr;
use crate::annotations::Label;

use hex_color::HexColor;

use image::{ImageBuffer, Rgba};

use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub fn annotate_cidrs(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, labels: Vec<Label>) {

	let font = Vec::from(include_bytes!("Inconsolata-CondensedRegular.ttf") as &[u8]);
  let font = Font::try_from_vec(font).unwrap();

	for label in labels {					

		let bbox = bbox_from_cidr(label.cidr);
		let color = HexColor::parse_rgba(label.color.as_str()).expect("Invalid hex color in shade file.");
		let fsize = if let Some(size) = label.size {
			size
		} else {
			24.0
		};
		let text = label.label.as_str();

		let scale = Scale {
			x: fsize,
			y: fsize,
		};

		let (w, h) = text_size(scale, &font, text);
		// println!("Text size: {}x{}", w, h);

		draw_text_mut(
			img, 
			Rgba([color.r, color.g, color.b, color.a]),
			((bbox.xmin + (bbox.width()/2))-((w/2) as u32)) as i32, 
			((bbox.ymin + (bbox.height()/2))-((h/2) as u32)) as i32, 
			scale, 
			&font, 
			text
		);

	}

}
