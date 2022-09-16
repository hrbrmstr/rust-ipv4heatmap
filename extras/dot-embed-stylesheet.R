#!/usr/local/bin/Rscript --vanilla

l <- readLines("assets/graph.svg")
css  <- readLines("extras/dot-style.css")

svg <- which(grepl("^<svg", l))

# pre <- l[1:(svg + 1)]

post <- l[(svg + 2):length(l)]

post <- gsub('fill="[^"]+"', "", post)
post <- gsub('stroke="[^"]+"', "", post)
post <- gsub('font-family="[^"]+"', "", post)

writeLines(c(l[svg:(svg + 1)], "<defs>", "<style>", css, "</style>", "</defs>", post), "assets/graph.svg")
