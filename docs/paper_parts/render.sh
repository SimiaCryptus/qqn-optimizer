#!/bin/bash

if [ ! -f content.tex ]; then
  pandoc combined.md -o content.tex \
    --bibliography=references.bib \
    --citeproc \
    --wrap=preserve \
    --top-level-division=section
else
  echo "content.tex already exists, skipping pandoc generation"
fi

pdflatex paper.tex

bibtex paper || echo "No bibliography to process"

pdflatex paper.tex