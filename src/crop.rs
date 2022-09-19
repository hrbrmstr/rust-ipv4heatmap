//! # Crop final heatmap output to the bounding box surrounding a list of CIDRs

use ril::{Rgba, Image };

use crate::utils::{BoundingBox, bbox_from_cidr, find_min, find_max};

/// Crop heatmap to the given CIDR list
pub fn crop_cidrs<S>(img: &mut Image<Rgba>, crops: Option<S>) where S: Into<String>, {
	
	if let Some(crops) = crops {
		
		let crops = crops.into();
		
		let cidrs: Vec<BoundingBox> = crops.split(',').map(bbox_from_cidr).collect();
		
		let xmins: Vec<u32> = cidrs.iter().copied().map(|x| x.xmin).collect();
		let ymins: Vec<u32> = cidrs.iter().copied().map(|x| x.ymin).collect();
		
		let xmaxs: Vec<u32> = cidrs.iter().copied().map(|x| x.xmax).collect();
		let ymaxs: Vec<u32> = cidrs.iter().copied().map(|x| x.ymax).collect();
		
		let x1 = *find_min(xmins.iter()).unwrap();
		let y1 = *find_min(ymins.iter()).unwrap();
		let x2 = *find_max(xmaxs.iter()).unwrap(); 
		let y2 = *find_max(ymaxs.iter()).unwrap();
		
		img.crop(x1, y1, x2, y2);
		
	}
	
}