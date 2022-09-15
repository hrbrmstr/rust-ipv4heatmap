#!/usr/local/bin/Rscript --vanilla

l <- readLines("graph.svg")
css  <- readLines("extras/dot-style.css")

svg <- which(grepl("^<svg", l))

pre <- l[1:(svg + 1)]

post <- l[(svg + 2):length(l)]

post <- gsub('fill="[^"]+"', "", post)
post <- gsub('stroke="[^"]+"', "", post)
post <- gsub('font-family="[^"]+"', "", post)

writeLines(c(pre, "<defs>", "<style>", css, "</style>", "</defs>", post), "graph.svg")
