//! # Font cache handler

use lazy_static::lazy_static;

use std::collections::HashMap;

pub type FontCache = HashMap<String, ril::Font>;

#[derive(Clone)]
pub struct CrateFont {
	pub font: ril::Font
}

lazy_static! {
	pub static ref BUILTIN_FONT:CrateFont = CrateFont { font: ril::Font::from_bytes(
		include_bytes!("Inconsolata-CondensedRegular.ttf") as &[u8],  128.0
	).expect("Builtin font error")
};
}

/// Load the specified font from the annotations file. The builtin one will be used if the font fails to load.
pub fn load_font(filename: Option<String>) -> ril::Font {

	if let Some(filename) = filename {
		match ril::Font::open(&filename, 64.0) {
			Ok(font) => font,
			Err(e) => {
				eprintln!("Error [{}] loading {}. Substituting internal font.", e, filename);
				BUILTIN_FONT.font.to_owned()
			}
		}
	} else {
		BUILTIN_FONT.font.to_owned()
	}

}