#![allow(dead_code)]
use serde_derive::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json;

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
}

#[derive(Debug, PartialEq)]
pub struct Outline {
	pub cidr: String,
	pub color: String,
}
#[derive(Debug, PartialEq)]
pub struct Shade {
	pub cidr: String,
	pub fill: String,
}

#[derive(Debug, PartialEq)]
pub struct Label {
	pub cidr: String,
	pub label: String,
	pub color: String,
	pub font: Option<String>
}

#[derive(Debug, PartialEq)]
/// Annotations on top of the heatmap can be outlines, shades, or labels.
/// This holds all of them (if any).
pub struct AnnotationCollection {
	pub outlines: Option<Vec<Outline>>,
	pub shades: Option<Vec<Shade>>,
	pub labels: Option<Vec<Label>>
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

  let v = AnnotationCollection {
		outlines: Some(outlines),
		shades: Some(shades),
		labels: Some(labels)
	};

	return v

}