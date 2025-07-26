#!/bin/bash

# Clean up any existing files
rm -f content.tex paper.aux paper.bbl paper.blg paper.log paper.pdf

if [ ! -f content.tex ]; then
  pandoc combined.md -o content.tex \
    --bibliography=references.bib \
    --citeproc \
    --wrap=preserve \
    --top-level-division=section \
    --natbib
else
  echo "content.tex already exists, skipping pandoc generation"
fi

pdflatex paper.tex

# Process bibliography
bibtex paper

# Second LaTeX pass (for cross-references)
pdflatex paper.tex

# Third LaTeX pass (to ensure everything is resolved)
pdflatex paper.tex