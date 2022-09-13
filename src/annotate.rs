use crate::utils::{ read_lines, bbox_from_cidr };

use hex_color::HexColor;

use image::{ImageBuffer, Rgba};

use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub fn annotate_cidrs(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, annotations_file: &str, _font: Option<String>) {

	let font = Vec::from(include_bytes!("Inconsolata-CondensedRegular.ttf") as &[u8]);
  let font = Font::try_from_vec(font).unwrap();

	if let Ok(lines) = read_lines(annotations_file) {
		for line in lines {
			
			if let Ok(record) = line {
				
				let fields: Vec<&str> = record.split("\t").collect();
				
				if fields.len() == 4 {
					
					let bbox = bbox_from_cidr(fields[0].to_string());
					let color = HexColor::parse_rgba(fields[1]).expect("Invalid hex color in shade file.");
					let fsize = fields[2].parse::<f32>().expect("Error parsing font size");
					let text = fields[3];

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
		}
	}


}