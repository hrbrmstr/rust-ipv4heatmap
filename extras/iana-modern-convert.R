library(tidyverse)

xdf <- read_csv("extras/iana-modern.csv", col_types = "cc")

sort(unique(xdf$label))

reg_cols <- c("ARIN" = "#5a84c088", "RIPE" = "#e2d25b88", "APNIC" = "#7e6aaa88", "LACNIC" = "#8cbf5588", "AFRINIC" = "#e68d4288")

xdf |>
  filter(
    label %in% c("ARIN", "RIPE", "APNIC", "LACNIC", "AFRINIC")
  ) |>
  mutate(
		`fill-color` = reg_cols[label],
    `display-prefix` = TRUE
	) -> registries

xdf |>
  filter(
    label %in% c("Amazon", "Apple", "AT&T", "Cogent", "Comcast", "Orange", "Ford", "Mercedes", "Prudential", "SOFTBANK")
  ) |> 
  mutate(
    `label-color` = "#00FF0055",
    `border-color` = "#FFFFFFFF",
    `display-prefix` = TRUE
  ) -> orgs

xdf |>
  filter(
    label %in% c("USPS", "DoD", "UK MoD")
  ) |> 
   mutate(
     `label-color` = "#FF000055",
     `border-color` = "#FFFFFFFF",
     `display-prefix` = TRUE
   ) -> gov

xdf |>
  filter(
    label %in% c("6to4", "Carrier NAT", "Multicast", "RFC 1918", "RFC 5737", "RFC 6890", "Private", "Reserved")
  ) |> 
  mutate(
    `label-color` = "#0000FF55",
    `border-color` = "#FFFFFFFF",
    `display-prefix` = TRUE
  ) -> infra

c(
  sub("\\]$", ",", jsonlite::toJSON(registries)),
  sub("^\\[", "", sub("\\]$", ",", jsonlite::toJSON(orgs))),
  sub("^\\[", "", sub("\\]$", ",", jsonlite::toJSON(gov))),
  sub("^\\[", "", sub("\\]$", "]", jsonlite::toJSON(infra)))
) |> 
	writeLines("extras/iana-modern.json")


