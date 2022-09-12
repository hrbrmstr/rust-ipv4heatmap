#!/usr/bin/env Rscript --vanilla
suppressWarnings(suppressPackageStartupMessages({
  library(RColorBrewer)
  library(viridis)
  library(dplyr)
  library(purrr)
}))

# get 256 viridis colors

v <- col2rgb(gsub("FF$", "", viridis(256)))

# but that creates duplicate entries which is a problem since that will
# cause the gd code in ipv4heatmap.c to not increment past them; so,
# we locate the dups and tweak them a bit

v <- data.frame(t(v))
dups <- which(duplicated(v))
v[dups, ] <- v[dups, ] - 1

# and generate colors.rs

color_fil <- "colors.rs"

cat(
  "#![allow(non_upper_case_globals)]",
  "#![allow(dead_code)]\n",
  "/* GENERATED AUTOMATICALLY DO NOT EDIT BY HAND */\n/* SEE extra/colorgen.r for details */",
  sep = "\n",
  file = color_fil
)

cat(
  sprintf(
    "\npub const viridis_red: [u8; 256] = [ %s ];\npub const viridis_green: [u8; 256] = [ %s ];\npub const viridis_blue: [u8; 256] = [ %s ];\n",
    paste0(v$red, collapse = ", "),
    paste0(v$green, collapse = ", "),
    paste0(v$blue, collapse = ", ")
  ),
  file = color_fil,
  append = TRUE
)

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
})

cat("Please move 'colors.rs' to the 'src' directory under the main source tree.\n")

