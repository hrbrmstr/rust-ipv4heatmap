mod utils;

mod colors;
use colors::{white, black};

use clap::Parser;

use image::{ImageBuffer};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

	/// color palette; one of (viridis brbg puor rdbu rdgy rdylbu spectral bupu reds ylgnbu ylorbr ylorrd)
  #[clap(short, long, default_value_t = String::from("viridis"))]
	palette: String,

	/// invert color palette
	#[clap(short, long)]
	invert: bool,

	/// reverse; white background, black text
	#[clap(short, long)]
	reverse: bool,

	/// filename of ips
	#[clap(short, long, default_value_t = String::from("ips.txt"))]
  filename: String,

	/// map output filename
	#[clap(short, long, default_value_t = String::from("map.png"))]
  output: String,

}

fn main() {
	
	let args = Args::parse();

	let colors: Vec<image::Rgb<u8>> = colors::select_palette(args.palette.as_str(), args.invert);
	
	let mut img = ImageBuffer::from_fn(4096, 4096, |_x, _y| {
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
	
	img.save(args.output).expect("Error saving file.");
	
}
