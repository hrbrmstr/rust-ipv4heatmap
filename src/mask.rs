//! # Hilight only a list of CIDRs on the heatmap image

use crate::utils::{ bbox_from_cidr, BoundingBox };

use ril::{Rectangle, Image, L, Rgba};

/// Given a vector of areas to highlight (masking all other blocks), turn all non-specified
/// blocks transparent.
pub fn mask_cidrs<S>(img: &mut Image<Rgba>, masks: Option<S>) where S: Into<String>, {
	
	if let Some(masks) = masks {
		
		let masks = masks.into();
		
		let cidrs: Vec<BoundingBox> = masks.split(',').map(bbox_from_cidr).collect();
		
		let mut img_mask = Image::new(4096, 4096, L(255));
		
		for cidr in cidrs {
			
			let rect  = Rectangle::new()
			.with_position(cidr.x() as u32, cidr.y() as u32)
			.with_size(cidr.width() as u32, cidr.height() as u32)
			.with_fill(L(0));
			
			img_mask.draw(&rect);
			img.mask_alpha(&img_mask);
			
		}
		
		img_mask.invert();
		img.mask_alpha(&img_mask);
		
	}
	
}
