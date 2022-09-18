//! # CIDR block annotations
//! 
//! There are four types of annotations supported:
//! 
//! - shade: fill a CIDR block with a specified color
//! - border: outline a CIDR block with a specified color
//! - label: fit a text label into a CIDR block
//! - prefix: add CIDR prefix text to a CIDR block
//! 
//! Annotations are stored in a single JSON file that is
//! an array of annotation objects with the following 
//! structure:
//! 
//! ```json
//! {
//!   "cidr": "4.0.0.0/8",
//!   "label-color": "#FFFFFFFF",
//!   "label": "Level3",
//!   "label-font": "extras/Lato-Black.ttf",
//!   "border-color": "#FFFFFFFF",
//!   "fill-color": "#FF00FF22",
//!   "display-prefix": true
//! }
//! ```
//! Not all fields are required, but if present:
//! 
//! - `fill-color` will overlay the specified color on the CIDR region
//! - `border-color` will draw a border around the specified color on the CIDR region
//! - `label`, `label-color`, and `label-font` (which is optional) will draw the specified label text to fit the CIDR region
//! - `display-prefix` will display the CIDR in Inconsolata at the CIDR bottom center w/alpha'd white (if present and `true`).
//! 
//! in that order.

use serde_derive::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Deserialization structure for the annotation JSON object
#[derive(Deserialize)]
pub struct Annotation {

	#[serde(rename = "cidr")]
	cidr: String,
	
	#[serde(rename = "border-color")]
	border_color: Option<String>,
	
	#[serde(rename = "fill-color")]
	fill_color: Option<String>,
	
	#[serde(rename = "label")]
	label: Option<String>,
	
	#[serde(rename = "label-color")]
	label_color: Option<String>,
	
	#[serde(rename = "label-font")]
	label_font: Option<String>,
	
	#[serde(rename = "display-prefix")]
	display_prefix: Option<bool>,

	#[serde(rename = "prefix-color")]
	prefix_color: Option<String>,

}

/// An annotation describing the CIDR outline style
#[derive(Debug, PartialEq, Eq)]
pub struct Outline {
	pub cidr: String,
	pub color: String,
}

/// An annotation describing the CIDR fill style
#[derive(Debug, PartialEq, Eq)]
pub struct Shade {
	pub cidr: String,
	pub fill: String,
}

/// An annotation describing the CIDR label text & style
#[derive(Debug, PartialEq, Eq)]
pub struct Label {
	pub cidr: String,
	pub label: String,
	pub color: String,
	pub font: Option<String>
}

/// An annotation that says to tag each CIDR block with the CIDR text
#[derive(Debug, PartialEq, Eq)]
pub struct Prefix {
	pub cidr: String,
	pub color: Option<String>
}

#[derive(Debug, PartialEq, Eq)]
/// Annotations on top of the heatmap can be outlines, shades, labels, or the CIDR text.
/// This structure holds all specified annotations.
pub struct AnnotationCollection {
	pub outlines: Option<Vec<Outline>>,
	pub shades: Option<Vec<Shade>>,
	pub labels: Option<Vec<Label>>,
	pub prefixes: Option<Vec<Prefix>>,
}

/// Open and read the spefified annotations JSON file.
pub fn load_config<P: AsRef<Path>>(path: P) -> AnnotationCollection {

	let file = File::open(path).expect("Error opening annotations file.");
	let reader = BufReader::new(file);
	
	let ann: Vec<Annotation> = serde_json::from_reader(reader).expect("Error loading annotations.");

	let outlines: Vec<Outline> = ann.iter()
	  .filter(|x| x.border_color.is_some())
    .map(|x| 
		Outline { 
			cidr: x.cidr.to_owned(), 
			color: x.border_color.to_owned().unwrap() 
		})
    .collect();

	let shades: Vec<Shade> = ann.iter()
	  .filter(|x| x.fill_color.is_some())
    .map(|x| 
		Shade { 
			cidr: x.cidr.to_owned(), 
			fill: x.fill_color.to_owned().unwrap() 
		})
    .collect();

	let labels: Vec<Label> = ann.iter()
	  .filter(|x| x.label.is_some() && x.label_color.is_some())
    .map(|x| 
		Label { 
			cidr: x.cidr.to_owned(), 
			label: x.label.to_owned().unwrap(),
			color: x.label_color.to_owned().unwrap(),
			font: x.label_font.to_owned(),
		})
    .collect();

	let prefixes: Vec<Prefix> = ann.iter()
	  .filter(|x| x.display_prefix.is_some() && x.display_prefix.unwrap())
    .map(|x| 
		Prefix { 
			cidr: x.cidr.to_owned(),
			color: x.prefix_color.to_owned()
		})
    .collect();

  

  AnnotationCollection {
		outlines: Some(outlines),
		shades: Some(shades),
		labels: Some(labels),
		prefixes: Some(prefixes)
	}

}