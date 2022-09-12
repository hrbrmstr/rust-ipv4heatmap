mod utils;
mod colors;
use colors::{white, black};

use clap::Parser;

// use image::ImageBuffer;
use image::RgbaImage;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
// #[clap(group(
//   ArgGroup::new("annnotate")
//     .args(&["annotations", "font"])
// 	  .required(true)
//     .requires_all(&["annotations", "font"]),
// ))]
struct Args {

	/// color palette; one of (viridis brbg puor rdbu rdgy rdylbu spectral bupu reds ylgnbu ylorbr ylorrd)
  #[clap(short, long, default_value_t = String::from("viridis"))]
	palette: String,

	/// invert color palette
	#[clap(short, long)]
	invert: bool,

	/// reverse (white background, black text)
	#[clap(short, long)]
	reverse: bool,

	/// file of IPs
	#[clap(short, long, default_value_t = String::from("ips.txt"))]
  filename: String,

	/// map output file
	#[clap(short, long, default_value_t = String::from("map.png"))]
  output: String,

	/// shades file
  #[clap(short, long)]
	shades: Option<String>,

	/// outlines file
  #[clap(long)]
	outlines: Option<String>,

	/// annotations file
  #[clap(short, long, requires = "font", requires_all = &["font", "annotations"])]
	annotations: Option<String>,

		/// annotation font
  #[clap(long, requires = "annotations", requires_all = &["font", "annotations"])]
	font: Option<String>,

}

fn main() {
	
	let args = Args::parse();

	let colors: Vec<image::Rgba<u8>> = colors::select_palette(args.palette.as_str(), args.invert);

	let mut img = RgbaImage::from_fn(4096, 4096, |_x, _y| {
    if args.reverse { white } else { black }
  }); 
	
	if let Ok(lines) = utils::read_lines(args.filename) {
		
		for line in lines {
			
			if let Ok(ip) = line {
				
				let (x, y) = utils::hil_xy_from_s(utils::ip_to_numeric(ip), 12);

				let pixel = *img.get_pixel(x, y);
				
				let k = colors.iter().position(|x| x == &pixel);
				
				match k {
					Some(pos) => 
						if pos < colors.len()-1 { 
							img.put_pixel(x, y, colors[pos+1])
					  }
					None => img.put_pixel(x, y, colors[0])
				}
				
			}
		}
	}

	if let Some(shades_file) = args.shades {
    utils::shade_cidrs(&mut img, shades_file.as_str());
	}

	if let Some(outlines_file) = args.outlines {
    utils::outline_cidrs(&mut img, outlines_file.as_str());
	}

	if let Some(annotations_file) = args.annotations {
    utils::annotate_cidrs(&mut img, annotations_file.as_str(), args.font);		
	}

 	img.save(args.output).expect("Error saving file.");
	
	
}
