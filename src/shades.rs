//! # CIDR block fill annotation

use crate::utils::bbox_from_cidr;
use crate::annotations::Shade;

use hex_color::HexColor;

use ril::{Rectangle, Rgba, OverlayMode, Image};

/// Given a vector of CIDRs, shade them in.
pub fn shade_cidrs(img: &mut Image<Rgba>, shades: Vec<Shade>) {
  
  for shade in shades {
    
    let bbox = bbox_from_cidr(shade.cidr);
    let fill = HexColor::parse_rgba(shade.fill.as_str()).expect("Invalid hex color in shade file.");
    
    let shade = Rgba{r:fill.r, g:fill.g, b:fill.b, a:fill.a};
    let rect  = Rectangle::new()
    .with_position(bbox.x() as u32, bbox.y() as u32)
    .with_size(bbox.width() as u32, bbox.height() as u32)
    .with_fill(shade)
    .with_overlay_mode(OverlayMode::Merge);
    
    img.draw(&rect);
    
  }
  
}