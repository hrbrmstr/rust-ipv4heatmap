mod colors;
use colors::ColorChannel::{Red, Green, Blue};

use std::str::FromStr;
use std::net::Ipv4Addr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;

use image::{ImageBuffer};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn hil_xy_from_s(ip_as_int: u32, order: i16) -> (u32, u32) {
	
	let mut i: i16;
	let mut state: u32 = 0;
	let mut x: u32 = 0;
	let mut y: u32 = 0;
	let mut row: u32;
	
	i = 2 * order - 2;
	
	let s = ip_as_int >> 8;
	
	while i >= 0 {
		
		row = 4 * state | ((s >> i) & 3);
		x = (x << 1) | ((0x936C >> row) & 1);
		y = (y << 1) | ((0x39C6 >> row) & 1);
		state = (0x3E6B94C1 >> 2 * row) & 3;
		
		i = i - 2;
		
	}
	
	(x, y)
	
}

fn ip_to_numeric(ip: String) -> u32 {
	let addr = Ipv4Addr::from_str(&ip).expect("Invalid IPv4");
  let addr_u32: u32 = addr.into();
	addr_u32
}

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

	let chosen_palette = colors::palette(args.palette.as_str());

  let mut red = colors::set_palette(&chosen_palette, Red);
  let mut green = colors::set_palette(&chosen_palette, Green);
  let mut blue = colors::set_palette(&chosen_palette, Blue);

	if args.invert {
	  red.reverse();
	  green.reverse();
	  blue.reverse();
	}
	
	let colors: Vec<image::Rgb<u8>> = (0..red.len()).into_iter().map(|i| image::Rgb([red[i], green[i], blue[i]])).collect();
	
	let mut img = ImageBuffer::from_fn(4096, 4096, |_x, _y| {
    if args.reverse {
      image::Rgb([255, 255, 255])
    } else {
      image::Rgb([0, 0, 0])
    }
  });
	
	if let Ok(lines) = read_lines(args.filename) {
		
		for line in lines {
			
			if let Ok(ip) = line {
				
				let (x, y) = hil_xy_from_s(ip_to_numeric(ip), 12);
				
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
