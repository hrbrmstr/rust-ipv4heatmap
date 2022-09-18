//! # Crop final heatmap output to the bounding box surrounding a list of CIDRs

use ril::{Rgba, Image };

use crate::utils::{BoundingBox, bbox_from_cidr, find_min, find_max};

/// Crop heatmap to the given CIDR list PRESENTLY BROKEN
pub fn crop_cidrs<S>(img: &mut Image<Rgba>, crops: S) where S: Into<String>, {

  let crops = crops.into();
  let crop_list: Vec<&str>  = crops.split(",").collect();
  let cidrs: Vec<BoundingBox> = crop_list.into_iter().map(|x| bbox_from_cidr(x)).collect();

  let xmins: Vec<u32> = cidrs.to_owned().into_iter().map(|x| x.xmin).collect();
  let ymins: Vec<u32> = cidrs.to_owned().into_iter().map(|x| x.ymin).collect();

	let xmaxs: Vec<u32> = cidrs.to_owned().into_iter().map(|x| x.xmax).collect();
  let ymaxs: Vec<u32> = cidrs.to_owned().into_iter().map(|x| x.ymax).collect();

  let x1 = find_min(xmins.iter()).unwrap().clone();
  let y1 = find_min(ymins.iter()).unwrap().clone();
  let x2 = find_max(xmaxs.iter()).unwrap().clone(); 
  let y2 = find_max(ymaxs.iter()).unwrap().clone();

  img.crop(x1, y1, x2, y2);

}