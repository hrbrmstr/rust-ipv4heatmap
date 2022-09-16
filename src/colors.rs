//! # Color palettes and color utilities used by annotations

use ril::Rgba;
use colorgrad::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
  static ref PALETTES: HashMap<&'static str, fn() -> Gradient > = {
    let mut map = HashMap::new();
    map.insert("blues", blues as fn() -> Gradient);
    map.insert("br_bg", br_bg);
    map.insert("bu_gn", bu_gn);
    map.insert("bu_pu", bu_pu);
    map.insert("cividis", cividis);
    map.insert("cool", cool);
    map.insert("gn_bu", gn_bu);
    map.insert("greens", greens);
    map.insert("greys", greys);
    map.insert("inferno", inferno);
    map.insert("magma", magma);
    map.insert("or_rd", or_rd);
    map.insert("oranges", oranges);
    map.insert("pi_yg", pi_yg);
    map.insert("plasma", plasma);
    map.insert("pr_gn", pr_gn);
    map.insert("pu_bu", pu_bu);
    map.insert("pu_bu_gn", pu_bu_gn);
    map.insert("pu_or", pu_or);
    map.insert("pu_rd", pu_rd);
    map.insert("purples", purples);
    map.insert("rainbow", rainbow);
    map.insert("rd_bu", rd_bu);
    map.insert("rd_gy", rd_gy);
    map.insert("rd_pu", rd_pu);
    map.insert("rd_yl_bu", rd_yl_bu);
    map.insert("rd_yl_gn", rd_yl_gn);
    map.insert("reds", reds);
    map.insert("sinebow", sinebow);
    map.insert("spectral", spectral);
    map.insert("turbo", turbo);
    map.insert("viridis", viridis);
    map.insert("warm", warm);
    map.insert("yl_gn", yl_gn);
    map.insert("yl_gn_bu", yl_gn_bu);
    map.insert("yl_or_br", yl_or_br);
    map.insert("yl_or_rd", yl_or_rd);
    map
  };
}

pub const WHITE:Rgba = Rgba{r:255, g:255, b:255, a:255};
pub const BLACK:Rgba = Rgba{r:0, g:0, b:0, a:255};

/// Take a 0…1 color channel domain value and upgrade it to 0:255
#[inline(always)]
fn upgrade(dom: f64) -> u8 { (dom * 255.0) as u8 }

/// Return 256 colors from the selected palette, optionally reversing it
pub fn select_palette(name: &str, invert: bool) -> Vec<Rgba> {
  
  let pal = PALETTES.get(name).expect("Unsupported palette.");
  let cols = pal().colors(256);
  
  let mut res: Vec<Rgba> = cols.into_iter().map(|c| Rgba{r:upgrade(c.r) , g:upgrade(c.g), b:upgrade(c.b), a:255}).collect();
  
  if invert {
    res.reverse();
  }
  
  res
  
}

/// Generate the 9-color palette used in the legend gradient, optionally reversing it
pub fn legend_cols(name: &str, invert: bool) -> Vec<String> {
  
  let pal = PALETTES.get(name).expect("Unsupported palette.");
  let cols = pal().colors(9);
  
  let mut res: Vec<String> = cols.into_iter().map(|c| c.to_hex_string()).collect();

  if invert {
    res.reverse()
  }
  
  res
}
