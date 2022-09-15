#!/usr/bin/env Rscript --vanilla
suppressWarnings(suppressPackageStartupMessages({
  library(RColorBrewer)
  library(viridis)
  library(dplyr)
  library(purrr)
}))

# generate colors.rs

color_fil <- "colors.rs"

cat(
  "//! # Color palettes and color utilities used by annotations\n",
  "#![allow(non_upper_case_globals)]",
  "#![allow(dead_code)]\n",
  sprintf(
    "/* GENERATED AUTOMATICALLY [%s] DO NOT EDIT BY HAND */\n/* SEE extras/colorgen.r for details */\n",
    Sys.time()
  ),
  "use image::Rgba;",
  sep = "\n",
  file = color_fil
)

viridis_maps <- c("viridis", "magma", "inferno", "plasma", "cividis", "rocket", "mako", "turbo")

walk(viridis_maps, \(x) {

  # get 256 viridis colors

  v <- col2rgb(gsub("FF$", "", viridis(256, option = x)))

  # but that creates duplicate entries which is a problem since that will
  # cause the gd code in ipv4heatmap.c to not increment past them; so,
  # we locate the dups and tweak them a bit

  v <- data.frame(t(v))
  dups <- which(duplicated(v))
  v[dups, ] <- v[dups, ] - 1
  v$red[v$red < 0] <- 0
  v$green[v$blue < 0] <- 0
  v$blue[v$blue < 0] <- 0

  cat(
    sprintf(
      "\npub const %s_red: [u8; 256] = [ %s ];\npub const %s_green: [u8; 256] = [ %s ];\npub const %s_blue: [u8; 256] = [ %s ];\n",
      x, 
      paste0(v$red, collapse = ", "),
      x,
      paste0(v$green, collapse = ", "),
      x,
      paste0(v$blue, collapse = ", ")
    ),
    file = color_fil,
    append = TRUE
  )

  cat(
    "pub const ", x, "_legend: [&str; 9] = [ ",
    paste0(sprintf('"%s"', gsub("FF$", "", viridis(9))), collapse = ", "),
    " ];\n\n",
    sep = "",
    file = color_fil,
    append = TRUE
  )

})


# give some more choices

diverg <- c("BrBG", "PuOr", "RdBu", "RdGy", "RdYlBu", "Spectral")

walk(diverg, \(x) {

  b <- data.frame(t(col2rgb(colorRampPalette(brewer.pal(11, x))(256))))

  if (sum(duplicated(b)) > 0) {
    print(x)
    print(sum(duplicated(b)))
    print(b[duplicated(b), ])
  }

  cat(
    sprintf(
      "\npub const %s_red: [u8; 256] = [ %s ];\npub const %s_green: [u8; 256] = [ %s ];\npub const %s_blue: [u8; 256] = [ %s ];\n",
      tolower(x),
      paste0(b$red, collapse = ", "),
      tolower(x),
      paste0(b$green, collapse = ", "),
      tolower(x),
      paste0(b$blue, collapse = ", ")
    ),
    file = color_fil,
    append = TRUE
  )

  cat(
    "pub const ", tolower(x), "_legend: [&str; 9] = [ ",
    paste0(sprintf('"%s"', colorRampPalette(brewer.pal(11, x))(9)), collapse = ", "),
    " ];\n\n",
    sep = "",
    file = color_fil,
    append = TRUE
  )

})

sequent <- c("BuPu", "Reds", "YlGnBu", "YlOrBr", "YlOrRd")

walk(sequent, \(x) {

  b <- data.frame(t(col2rgb(colorRampPalette(brewer.pal(9, x))(256))))

  if (sum(duplicated(b)) > 0) {
    print(x)
    print(sum(duplicated(b)))
    print(b[duplicated(b),])
    print("================")
  }

  cat(
    sprintf(
      "\npub const %s_red: [u8; 256] = [ %s ];\npub const %s_green: [u8; 256] = [ %s ];\npub const %s_blue: [u8; 256] = [ %s ];\n",
      tolower(x),
      paste0(b$red, collapse = ", "),
      tolower(x),
      paste0(b$green, collapse = ", "),
      tolower(x),
      paste0(b$blue, collapse = ", "))
    ,
    file = color_fil,
    append = TRUE
  )

  cat(
    "pub const ", tolower(x), "_legend: [&str; 9] = [ ",
    paste0(sprintf('"%s"', colorRampPalette(brewer.pal(9, x))(9)), collapse = ", "),
    " ];\n\n",
    sep = "",
    file = color_fil,
    append = TRUE
  )
  
})

cat(
  r"(
pub const white:Rgba<u8> = Rgba([255, 255, 255, 255]);
pub const black:Rgba<u8> = Rgba([0, 0, 0, 255]);

#[derive(Debug, PartialEq)]
pub enum ColorChannel {
  Red,
  Green,
  Blue,
}

use self::ColorChannel::{Red, Green, Blue};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
 enum ColorPalette {
  viridis,
	magma,
	inferno,
	plasma,
	cividis,
	rocket,
	mako,
	turbo,
  brbg,
  puor,
  rdbu,
  rdgy,
  rdylbu,
  spectral,
  bupu,
  reds,
  ylgnbu,
  ylorbr,
  ylorrd
}

use self::ColorPalette::{
  viridis,
	magma,
	inferno,
	plasma,
	cividis,
	rocket,
	mako,
	turbo,
  brbg,
  puor,
  rdbu,
  rdgy,
  rdylbu,
  spectral,
  bupu,
  reds,
  ylgnbu,
  ylorbr,
  ylorrd
};

fn palette<S>(name: S) -> ColorPalette where S: Into<String>, {
  match name.into().as_str() {
    "viridis" => viridis,
    "magma" => magma,
    "inferno" => inferno,
    "plasma" => plasma,
    "cividis" => cividis,
    "rocket" => rocket,
    "mako" => mako,
    "turbo" => turbo,
    "brbg" => brbg,
    "puor" => puor,
    "rdbu" => rdbu,
    "rdgy" => rdgy,
    "rdylbu" => rdylbu,
    "spectral" => spectral,
    "bupu" => bupu,
    "reds" => reds,
    "ylgnbu" => ylgnbu,
    "ylorbr" => ylorbr,
    "ylorrd" => ylorrd,
    _ => viridis
  }
}

fn legend_cols<S>(name: S) -> [&'static str; 9] where S: Into<String>, {

  let chosen_palette = palette(name);
  
  match chosen_palette {
    viridis => viridis_legend,
    magma => magma_legend,
    inferno => inferno_legend,
    plasma => plasma_legend,
    cividis => cividis_legend,
    rocket => rocket_legend,
    mako => mako_legend,
    turbo => turbo_legend,
    brbg => brbg_legend,
    puor => puor_legend,
    rdbu => rdbu_legend,
    rdgy => rdgy_legend,
    rdylbu => rdylbu_legend,
    spectral => spectral_legend,
    bupu => bupu_legend,
    reds => reds_legend,
    ylgnbu => ylgnbu_legend,
    ylorbr => ylorbr_legend,
    ylorrd => ylorrd_legend,
  }

}

fn set_palette(palette: &ColorPalette, channel: ColorChannel) -> [u8; 256] {
  
  match palette {
    viridis => {
      match channel {
        Red => viridis_red,
        Green => viridis_green,
        Blue => viridis_blue,
      }	
    },

    magma => {
      match channel {
        Red => magma_red,
        Green => magma_green,
        Blue => magma_blue,
      }
    },

    inferno => {
      match channel {
        Red => inferno_red,
        Green => inferno_green,
        Blue => inferno_blue,
      }
    },

    plasma => {
      match channel {
        Red => plasma_red,
        Green => plasma_green,
        Blue => plasma_blue,
      }
    },

    cividis => {
      match channel {
        Red => cividis_red,
        Green => cividis_green,
        Blue => cividis_blue,
      }
    },

    rocket => {
      match channel {
        Red => rocket_red,
        Green => rocket_green,
        Blue => rocket_blue,
      }
    },

    mako => {
      match channel {
        Red => mako_red,
        Green => mako_green,
        Blue => mako_blue,
      }
    },

    turbo => {
      match channel {
        Red => turbo_red,
        Green => turbo_green,
        Blue => turbo_blue,
      }
    },

    
    brbg => {
      match channel {
        Red => brbg_red,
        Green => brbg_green,
        Blue => brbg_blue,
      }	
    },
    
    puor => {
      match channel {
        Red => puor_red,
        Green => puor_green,
        Blue => puor_blue,
      }	
    },
    
    rdbu => {
      match channel {
        Red => rdbu_red,
        Green => rdbu_green,
        Blue => rdbu_blue,
      }	
    },
    
    rdgy => {
      match channel {
        Red => rdgy_red,
        Green => rdgy_green,
        Blue => rdgy_blue,
      }	
    },
    
    rdylbu => {
      match channel {
        Red => rdylbu_red,
        Green => rdylbu_green,
        Blue => rdylbu_blue,
      }	
    },
    
    spectral => {
      match channel {
        Red => spectral_red,
        Green => spectral_green,
        Blue => spectral_blue,
      }	
    },
    
    bupu => {
      match channel {
        Red => bupu_red,
        Green => bupu_green,
        Blue => bupu_blue,
      }	
    },
    
    reds => {
      match channel {
        Red => reds_red,
        Green => reds_green,
        Blue => reds_blue,
      }	
    },
    
    ylgnbu => {
      match channel {
        Red => ylgnbu_red,
        Green => ylgnbu_green,
        Blue => ylgnbu_blue,
      }	
    },
    
    ylorbr => {
      match channel {
        Red => ylorbr_red,
        Green => ylorbr_green,
        Blue => ylorbr_blue,
      }	
    },
    
    ylorrd => {
      match channel {
        Red => ylorrd_red,
        Green => ylorrd_green,
        Blue => ylorrd_blue,
      }	
    },
        
  }
  
}

pub fn select_palette<S>(name: S, invert: bool) -> Vec<image::Rgba<u8>> where S: Into<String>, {

	let chosen_palette = palette(name);

  let mut red = set_palette(&chosen_palette, Red);
  let mut green = set_palette(&chosen_palette, Green);
  let mut blue = set_palette(&chosen_palette, Blue);

	if invert {
	  red.reverse();
	  green.reverse();
	  blue.reverse();
	}
	
	(0..red.len()).into_iter().map(|i| image::Rgba([red[i], green[i], blue[i], 255])).collect()

}

#[cfg(test)]

#[test]
fn test_set_palette() {
	let result = self::select_palette("viridis", false);
	assert_eq!(result[0], image::Rgba([self::viridis_red[0], viridis_green[0], viridis_blue[0], 255]));
}

#[test]
fn test_palette() {
	let result = self::palette("ylorbr");
	assert_eq!(result, self::ColorPalette::ylorbr);
}

)",
    file = color_fil,
    append = TRUE
  )


cat(
  "Please move 'colors.rs' to the 'src' directory under the main source tree.\n"
)

