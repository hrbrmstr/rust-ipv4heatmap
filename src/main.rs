//! # ipv4-heatmap
//! 
//! This crate builds a binary that can be used to create 
//! [IPv4 Hilbert heatmaps](https://www.caida.org/archive/arin-heatmaps/).
//! 
//! <div>
//! <img src="https://rud.is/dl/map.png"/>
//! </div>
//! 
//! You can find design decisions and more information [in this repository](https://github.com/hrbrmstr/rust-ipv4heatmap).

mod annotations;
pub mod utils;
mod fonts;
mod colors;
mod heatmap;
mod shades;
mod outlines;
mod labels;
mod prefixes;
mod crop;
mod mask;

use crate::annotations::AnnotationCollection;
use crate::colors::{WHITE, BLACK};

use clap::Parser;

use ril::{Image};

use anyhow::{Result, anyhow};

/// Supported CLI args
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	
	/// color palette to use; one of (blues br_bg bu_gn bu_pu cividis cool gn_bu greens greys inferno magma or_rd oranges pi_yg plasma pr_gn pu_bu pu_bu_gn pu_or pu_rd purples rainbow rd_bu rd_gy rd_pu rd_yl_bu rd_yl_gn reds sinebow spectral turbo viridis warm yl_gn yl_gn_bu yl_or_br yl_or_rd)
	#[clap(short, long, default_value_t = String::from("cividis"))]
	palette: String,
	
	/// invert the chosen color palette
	#[clap(short, long)]
	invert: bool,
	
	/// reverse the heatmap base (i.e. white background, black text)
	#[clap(short, long)]
	reverse: bool,
	
	/// input file of IPs
	#[clap(short, long, default_value_t = String::from("ips.txt"))]
	filename: String,
	
	/// heatmap output file; extenstion determines format
	#[clap(short, long, default_value_t = String::from("map.png"))]
	output: String,
	
	/// file containing JSON CIDR annotations
	#[clap(short, long)]
	annotations: Option<String>,
	
	/// output an SVG colourbar legend to this file
	#[clap(short, long)]
	legend_file: Option<String>,
	
	/// crop output to area represented by these CIDRs (comma separated CIDR list)
	#[clap(short, long)]
	crop: Option<String>,
	
	/// Hilight only certain CIDRs in the heatmap image. Can be used with the "crop" argument
	/// to produce a masked and cropped heatmap image.
	#[clap(short, long)]
	mask: Option<String>,
	
}

/// main!
fn main() -> Result<()> {
	
	let args = Args::parse();
	
	let mut img = Image::new(4096, 4096, if args.reverse { WHITE } else { BLACK });
	
	heatmap::render_heatmap(&mut img, args.filename, args.palette.to_owned(), args.invert)?;
	
	if let Some(annotations) = args.annotations {
		
		let ann: AnnotationCollection = annotations::load_config(annotations)?;
		
		shades::shade_cidrs(&mut img, ann.shades)?;
  	outlines::outline_cidrs(&mut img, ann.outlines)?;
		labels::annotate_cidrs(&mut img, ann.labels)?;		
		prefixes::annotate_prefixes(&mut img, ann.prefixes)?;		
		
	}
	
	mask::mask_cidrs(&mut img, args.mask);
	crop::crop_cidrs(&mut img, args.crop);
	
	match img.save_inferred(args.output) {
		Ok(_) => utils::output_legend(args.legend_file, &args.palette, args.invert)?,
		Err(e) => return Err(anyhow!("Error saving heatmap image file: {e}."))
	}
	
	Ok(())
	
}
