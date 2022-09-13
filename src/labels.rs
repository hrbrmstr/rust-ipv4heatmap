use crate::utils::bbox_from_cidr;
use crate::annotations::Label;

use hex_color::HexColor;

use image::{ImageBuffer, Rgba};

use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub fn annotate_cidrs(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, labels: Vec<Label>) {

	let builtin_font = Vec::from(include_bytes!("Inconsolata-CondensedRegular.ttf") as &[u8]);
  let builtin_font = Font::try_from_vec(builtin_font).unwrap();

	for label in labels {					

		let bbox = bbox_from_cidr(label.cidr);
		let color = HexColor::parse_rgba(label.color.as_str()).expect("Invalid hex color in shade file.");

		let text = label.label.as_str();
		let font = match label.font {
      Some(f) => {
	      let user_font = std::fs::read(f).expect("Error reading user-specified font.");
        let user_font = Font::try_from_vec(user_font).expect("Error parsing user-specified font.");
				user_font
			},
			None => builtin_font.to_owned()
		};

		let mut scale = Scale {
			x: 256.0,
			y: 256.0,
		};

		let (mut w, mut h) = text_size(scale, &font, text);

		let padding:u32 = 50;

    let bw = bbox.width() - padding;
		let bh = bbox.height() - padding;

		while w > ((bw as i32)) || h > ((bh as i32)) {
			scale = Scale { x: scale.x*0.95, y: scale.y*0.95 };
			(w, h) = text_size(scale, &font, text);
		}

		draw_text_mut(
			img, 
			Rgba([color.r, color.g, color.b, color.a]),
			((bbox.xmin + padding/2 + (bw/2))-((w as f32/2.0) as u32)) as i32, 
			((bbox.ymin + padding/2 + (bh/2))-((h as f32/1.65) as u32)) as i32, 
			scale, 
			&font, 
			text
		);

	}

}
