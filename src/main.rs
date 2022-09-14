mod annotations;
mod utils;
mod outlines;
mod shades;
mod labels;
mod colors;
use annotations::AnnotationCollection;
use colors::{white, black};

use clap::Parser;

use ril::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

	/// color palette to use; one of (viridis magma inferno plasma cividis rocket mako turbo brbg puor rdbu rdgy rdylbu spectral bupu reds ylgnbu ylorbr ylorrd)
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

}

fn main() {
	
	let args = Args::parse();

	let colors: Vec<ril::Rgba> = colors::select_palette(args.palette.to_owned(), args.invert);

	let mut img = Image::new(4096, 4096, if args.reverse { white } else { black });
	// draw /24 pixels

	if let Ok(lines) = utils::read_lines(args.filename) {
		
		for line in lines {
			
			if let Ok(ip) = line {
				
				let (x, y) = utils::hil_xy_from_s(utils::ip_to_numeric(ip), 12);

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
		}
	}

  if let Some(annotations) = args.annotations {

    let ann: AnnotationCollection = annotations::load_config(annotations);

		if let Some(shades) = ann.shades {
		  shades::shade_cidrs(&mut img, shades);
		}

		if let Some(outlines) = ann.outlines {
		  outlines::outline_cidrs(&mut img, outlines);
		}

		if let Some(labels) = ann.labels {
		  labels::annotate_cidrs(&mut img, labels);		
		}

	}

 	img.save_inferred(args.output).expect("Error saving file.");

  if let Some(f) = args.legend_file {
    utils::output_legend(f, args.palette, args.invert)
	}

}
