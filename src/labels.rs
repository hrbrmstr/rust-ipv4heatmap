use crate::utils::bbox_from_cidr;
use crate::annotations::Label;

use hex_color::HexColor;

use image::{ImageBuffer, Rgba, Pixel};
use imageproc::definitions::{Clamp};
use imageproc::drawing::{Canvas, text_size};
use imageproc::pixelops::{weighted_sum};

use conv::ValueInto;
use std::cmp::max;
use rusttype::{point, PositionedGlyph, Rect, Font, Scale};

/// This is here for the eventual transparency solution
fn layout_glyphs(
	scale: Scale,
	font: &Font,
	text: &str,
	mut f: impl FnMut(PositionedGlyph, Rect<i32>),
) -> (i32, i32) {
	let v_metrics = font.v_metrics(scale);
	
	let (mut w, mut h) = (0, 0);
	
	for g in font.layout(text, scale, point(0.0, v_metrics.ascent)) {
		if let Some(bb) = g.pixel_bounding_box() {
			w = max(w, bb.max.x);
			h = max(h, bb.max.y);
			f(g, bb);
		}
	}
	
	(w, h)
}

/// This is here for the eventual transparency solution
pub fn draw_blended_text_mut<'a, C>(
	canvas: &'a mut C,
	color: C::Pixel,
	x: i32,
	y: i32,
	scale: Scale,
	font: &'a Font<'a>,
	text: &'a str,
) where
C: Canvas,
<C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
	let image_width = canvas.width() as i32;
	let image_height = canvas.height() as i32;
	
	layout_glyphs(scale, font, text, |g, bb| {
		g.draw(|gx, gy, gv| {
			let gx = gx as i32 + bb.min.x;
			let gy = gy as i32 + bb.min.y;
			
			let image_x = gx + x;
			let image_y = gy + y;

			// TODO figure out how to blend the text

			if (0..image_width).contains(&image_x) && (0..image_height).contains(&image_y) {
				let pixel = canvas.get_pixel(image_x as u32, image_y as u32);
				let weighted_color = weighted_sum(pixel, color, 1.0 - gv, gv);
				canvas.draw_pixel(image_x as u32, image_y as u32, weighted_color);
			}
		})
	});
}

/// Given a vector of label annotations, draw the labels.
/// 
/// NOTE: Transparency is still an issue.
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
		
		draw_blended_text_mut(
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
