use crate::utils::bbox_from_cidr;
use crate::annotations::Shade;

use hex_color::HexColor;

use image::{ImageBuffer, Rgba, GenericImage, Pixel};

use imageproc::rect::Rect;

pub fn draw_blended_rect_mut<I>(image: &mut I, rect: Rect, color: I::Pixel) 
  where I: GenericImage, I::Pixel: 'static, {

	let image_bounds = Rect::at(0, 0).of_size(image.width(), image.height());

	if let Some(intersection) = image_bounds.intersect(rect) {
		for dy in 0..intersection.height() {
			for dx in 0..intersection.width() {
				let x = intersection.left() as u32 + dx;
				let y = intersection.top() as u32 + dy;
				let mut pixel = image.get_pixel(x, y); 
				pixel.blend(&color);
				unsafe {
					image.unsafe_put_pixel(x, y, pixel);
				}
			}
		}
	}

}

pub fn shade_cidrs(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, shades: Vec<Shade>) {
	
		for shade in shades {

			let bbox = bbox_from_cidr(shade.cidr);
			let fill = HexColor::parse_rgba(shade.fill.as_str()).expect("Invalid hex color in shade file.");
					
			draw_blended_rect_mut(
				img, 
				Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height()),
				Rgba([fill.r, fill.g, fill.b, fill.a])
			);
								
		}
	
}