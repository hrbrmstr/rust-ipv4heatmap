//! # Heatmap renderer.

use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{Context, Result};

use crate::colors;
use crate::utils;

use ril::{Rgba, Image};

pub fn render_heatmap(img: &mut Image<Rgba>, filename: String, palette: String, invert: bool) -> Result<()> {

	let file = File::open(filename).context("Cannot find or access file containing IPv4s.")?;
	let lines = io::BufReader::new(file).lines();

  let colors: Vec<ril::Rgba> = colors::select_palette(&palette, invert)?;
	
	for ip in lines.flatten() {
		
		let (x, y) = utils::hil_xy_from_s(utils::ip_to_numeric(ip)?, 12);
		
		let pixel = img.get_pixel(x, y).unwrap();
		
		let k = colors.iter().position(|x| x == pixel);
		
		match k {
			Some(pos) => 
			if pos < colors.len()-1 { 
				img.set_pixel(x, y, colors[pos+1])
			}
			None => img.set_pixel(x, y, colors[0])
		}
		
	}

	Ok(())

}