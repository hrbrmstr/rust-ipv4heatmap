//! # Utility functions used by across the crate

use crate::colors::legend_cols;

use std::cmp::{min, max};
use std::str::FromStr;
use std::net::Ipv4Addr;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};

use cidr::Ipv4Cidr;

/// Given a filename or path and palette name (+ whether the palette should be inverted) 
/// write an SVG legend out to the specified file.
pub fn output_legend<P>(filename: Option<P>, name: &str, invert: bool) -> Result<()> where P: AsRef<Path>, {
	
	if let Some(filename) = filename {
		
		let cols = legend_cols(name, invert)?;
		
		let res = format!(r#"
		<svg class="hilbert-legend" width="340" height="70" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
		<defs>
		<style>
		svg.hilbert-legend {{ padding-top: 10pt; }}
		.hilbert-legend-domain, .hilbert-legend-tick, line {{ stroke: black; opacity: 1; }}
		.hilbert-legend-axis {{ fill: none; font-size: 8pt; font-family: sans-serif; text-anchor: middle; }}
		.hilbert-legend-axis-text {{ fill: black; font-family: sans-serif; font-size: 8pt; font-weight: 300; }}
		.hilbert-legend-title {{ font-family: sans-serif; text-anchor: start; font-size: 10pt; fill: black; font-weight: 700; }}
		@media (prefers-color-scheme: dark) {{
			svg {{ background-color: black; }}
			.hilbert-legend-domain, .hilbert-legend-tick, line {{ stroke: white; opacity: 1; }}
			.hilbert-legend-axis {{ fill: none; font-size: 8pt; font-family: sans-serif; text-anchor: middle; }}
			.hilbert-legend-axis-text {{ fill: white; font-family: sans-serif; font-size: 8pt; font-weight: 300; }}
			.hilbert-legend-title {{ font-family: sans-serif; text-anchor: start; font-size: 10pt; fill: white; font-weight: 700; }}
		}}
		</style>
		<linearGradient id="hilbert-legend-bar">
		<stop offset="0" stop-color="{}" />
		<stop offset="0.125" stop-color="{}" />
		<stop offset="0.25" stop-color="{}" />
		<stop offset="0.375" stop-color="{}" />
		<stop offset="0.5" stop-color="{}" />
		<stop offset="0.625" stop-color="{}" />
		<stop offset="0.75" stop-color="{}" />
		<stop offset="0.875" stop-color="{}" />
		<stop offset="1" stop-color="{}" />
		</linearGradient>
		</defs>
		<g><text class="hilbert-legend-title" x="20" y="10">Addresses per-pixel</text></g>
		<g>
		<rect width="300" height="20" transform="translate(20,16)" style="fill: url(&quot;#hilbert-legend-bar&quot;);" />
		</g>
		<g class="hilbert-legend-axis" transform="translate(20,40)">
		<path class="hilbert-legend-domain"  d="M0,6V0H300V6" />
		<g class="hilbert-legend-tick" transform="translate(0,0)">
		<line y2="6" /><text class="hilbert-legend-axis-text" y="9" dy="0.71em">1</text></g>
		<g class="hilbert-legend-tick" opacity="1" transform="translate(150.58823529411765,0)">
		<line y2="6" /><text class="hilbert-legend-axis-text" y="9" dy="0.71em">128</text></g>
		<g class="hilbert-legend-tick" opacity="1" transform="translate(300,0)">
		<line y2="6" /><text class="hilbert-legend-axis-text" y="9" dy="0.71em">255</text></g>
		</g>
		</svg>
		"#, cols[0], cols[1], cols[2], cols[3], cols[4], cols[5], cols[6], cols[7], cols[8]);
		
		let mut output = File::create(filename).context("Error opening legend file for writing.")?;
		write!(output, "{}", res).context("Error writing legend file.")?;
		
	}
	
	Ok(())
	
}


/// Find the maximum value in iterable
pub fn find_max<I>(iter: I) -> Option<I::Item> where I: Iterator, I::Item: Ord, {
	iter.reduce(|accum, item| {
		if accum >= item { accum } else { item }
	})
}

/// Find the minimum value in iterable
pub fn find_min<I>(iter: I) -> Option<I::Item> where I: Iterator, I::Item: Ord, {
	iter.reduce(|accum, item| {
		if accum >= item { item } else { accum }
	})
}

/// Convert an characrter IPv4 address into an integer.
/// 
/// Panics on invalid address since it's in a CLI.
/// 
/// ```rust
/// let res = ip_to_numeric("192.168.1.1");
/// ## 3232235777
/// ```
pub fn ip_to_numeric<S>(ip: S) -> Result<u32> where S: Into<String>, {
	let addr = Ipv4Addr::from_str(&ip.into()).context("Invalid IPv4")?;
	let addr_u32: u32 = addr.into();
	Ok(addr_u32)
}

/// Convert an IPv4 address (in integer form) to a 12th order Hilbert x/y point
/// 
/// This is a funky state-transition table made (in)famous? in 
/// "[The Hacker's Delight](https://en.wikipedia.org/wiki/Hacker's_Delight)".
/// 
/// The [license](https://web.archive.org/web/20060108180340/http://www.hackersdelight.org/permissions.htm)
/// is quite generous, if not adorable by today's standards. Do not try to visit the
/// site today as spammers nabbed the domain.
/// 
/// In any Hilbert curve, only four of eight possible U-shapes occur:
/// 
/// - (A) left-to-right arrow downward facing
/// - (B) bottom-to-top arrow leftward facing
/// - (C) right-to-left arrow upward facing
/// - (D) top-to-bottom arrow rightward facing
/// 
/// In this program, the current `state` is represented by an integer from 0 to 3 for 
/// the above states A through D, respectively. In the assignment to `row`, the current 
/// state is concatenated with the next two bits of `s`, giving an integer from 0 to 15, 
/// which is the applicable row number in the state table (below). `row` is used to 
/// access integers (expressed in hexadecimal) that are used as bit strings to represent 
/// the rightmost two columns of the state table (that is, these accesses are in-register 
	/// table lookups). Left-to-right in the hexadecimal values corresponds to bottom-to-top in
	/// the state table.
	/// 
	/// |If the<br/>current<br/>state is|and the<br/>next (to right)<br/>2 bits of s are|then<br/>append<br/>to (x,y)|and<br/>enter<br/>state|
	/// |:----:|:---:|:-------:|:---:|
	/// | `A` | `00` | `(0,0)` | `B` |
	/// | `A` | `01` | `(0,1)` | `A` |
	/// | `A` | `10` | `(1,1)` | `A` |
	/// | `A` | `11` | `(1,0)` | `D` |
	/// | `B` | `00` | `(0,0)` | `A` |
	/// | `B` | `01` | `(0,1)` | `B` |
	/// | `B` | `10` | `(1,1)` | `B` |
	/// | `B` | `11` | `(1,0)` | `C` |
	/// | `C` | `00` | `(0,0)` | `D` |
	/// | `C` | `01` | `(0,1)` | `C` |
	/// | `C` | `10` | `(1,1)` | `C` |
	/// | `C` | `11` | `(1,0)` | `B` |
	/// | `D` | `00` | `(0,0)` | `C` |
	/// | `D` | `01` | `(0,1)` | `D` |
	/// | `D` | `10` | `(1,1)` | `D` |
	/// | `D` | `11` | `(1,0)` | `A` |
	/// 
	/// Original C code:
	/// 
	/// ```c
	/// void hil_xy_from_s(unsigned s, int n, unsigned *xp, unsigned *yp) {
		///   int i;
		///   unsigned state, x, y, row;
		///   state = 0; // Initialize state
		///   x = y = 0;
		/// 
		///   for (i = 2*n - 2; i >= 0; i -= 2) {  // Do n times.
			///     row = 4*state | (s>>i) & 3;        // Row in table. 
			///     x = (x << 1) | (0x936C >> row) & 1;
			///     y = (y << 1) | (0x39C6 >> row) & 1;
			///     state = (0x3E6B94C1 >> 2*row) & 3; // New state.
			///   }
			/// 
			///   *xp = x; // pass results back
			///   *yp = y;
			/// }
			/// ```
			/// 
			/// Grab the book for a few more alternative implementations.
			/// 
			pub fn hil_xy_from_s(ip_as_int: u32, order: i16) -> (u32, u32) {
				
				let mut i: i16;
				let mut state: u32 = 0;
				let mut x: u32 = 0;
				let mut y: u32 = 0;
				let mut row: u32;
				
				i = 2 * order - 2;
				
				let s = ip_as_int >> 8; // we're only supporting full internet maps
				
				while i >= 0 {
					
					row = (4 * state) | ((s >> i) & 3);
					x = (x << 1) | ((0x936C >> row) & 1);
					y = (y << 1) | ((0x39C6 >> row) & 1);
					state = (0x3E6B94C1 >> (2 * row)) & 3;
					
					i -= 2;
					
				}
				
				(x, y)
				
			}
			
			/// CIDRs in Hilbert space can represent a bounding box
			#[derive(Debug, PartialEq, Eq, Copy, Clone)]
			pub struct BoundingBox {
				pub xmin: u32,
				pub xmax: u32,
				pub ymin: u32,
				pub ymax: u32,
			}
			
			impl BoundingBox {
				
				pub fn x(&self) -> i32 { self.xmin as i32 }
				pub fn y(&self) -> i32 { self.ymin as i32 }
				pub fn width(&self) -> u32 { self.xmax - self.xmin }
				pub fn height(&self) -> u32 { self.ymax - self.ymin }
				
			}
			
			/// Given the first (numeric) IP address in a CIDR block and the size of
			/// the CIDR block, return the bounding box. This handles the single point,
			/// square, and rectangle cases.
			fn bbox(first: u32, slash: u8) -> BoundingBox {
				
				let mut diag: u32 = 0xAAAAAAAA;
				
				if slash > 31 { // special case of one point
					
					let (xmin, ymin) = hil_xy_from_s(first, 12);
					
					BoundingBox{ xmin, xmax: xmin, ymin, ymax: ymin }
					
				} else if 0 == (slash & 1) { // square
					
					diag >>= slash;
					
					let (x1, y1) = hil_xy_from_s(first, 12);
					let (x2, y2) = hil_xy_from_s(first + diag, 12);
					
					BoundingBox{ 
						xmin: min(x1, x2),
						xmax: max(x1, x2), 
						ymin: min(y1, y2), 
						ymax: max(y1, y2) 
					}
					
				} else { // rect (split into squares)
					
					let bbox1 = bbox(first, slash + 1);
					let bbox2 = bbox(first + (1 << (32 - (slash + 1))), slash + 1);
					
					BoundingBox{ 
						xmin: min(bbox1.xmin, bbox2.xmin), 
						xmax: max(bbox1.xmax, bbox2.xmax), 
						ymin: min(bbox1.ymin, bbox2.ymin), 
						ymax: max(bbox1.ymax, bbox2.ymax), 
					}
					
				}
				
			}
			
			/// Given a CIDR in `IP/##` form, return the bounding box.
			/// 
			/// # Examples
			/// 
			/// ```rust
			/// let result = bbox_from_cidr("218.0.0.0/7");
			/// // BoundingBox { xmin: 2048, xmax: 2559, ymin: 1024, ymax: 1279 }
			/// ```
			pub fn bbox_from_cidr<S>(cidr: S) -> BoundingBox where S: Into<String>, {
				
				if let Ok(parsed_cidr) = Ipv4Cidr::from_str(&cidr.into()) {
					
					let first: u32 = parsed_cidr.first_address().into();
					let slash: u8 = parsed_cidr.network_length();
					
					return bbox(first, slash)
					
				}
				
				BoundingBox{ xmin:0, xmax:0, ymin:0, ymax:0 }
				
			}
			
			#[cfg(test)]
			
			#[test]
			fn test_hil_xy_from_s() {
				let result = self::hil_xy_from_s(3232235777, 12);
				assert_eq!(result, (3871, 1822));
			}
			
			#[test]
			fn test_ip_to_numeric() {
				let result = self::ip_to_numeric("192.168.1.1").expect("IP conversion error");
				assert_eq!(result, 3232235777);
			}
			
			#[test]
			fn test_bbox_from_cidr() {
				let result = self::bbox_from_cidr("218.0.0.0/7");
				assert_eq!(result, BoundingBox { xmin: 2048, xmax: 2559, ymin: 1024, ymax: 1279 });
				let result = self::bbox_from_cidr("217.0.0.0/8");
				assert_eq!(result, BoundingBox { xmin: 2048, xmax: 2303, ymin: 1280, ymax: 1535 });
			}