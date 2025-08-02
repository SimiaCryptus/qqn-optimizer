#!/bin/bash

# Clean up any existing files
rm -f content.tex appendix.tex paper.aux paper.bbl paper.blg paper.log paper.pdf

if [ ! -f content.tex ]; then
  pandoc content.md -o content.tex \
    --bibliography=references.bib \
    --citeproc \
    --wrap=preserve \
    --top-level-division=section \
   --resource-path=.. \
    --natbib
else
  echo "content.tex already exists, skipping pandoc generation"
fi

if [ ! -f appendix.tex ]; then
  pandoc appendix.md -o appendix.tex \
    --bibliography=references.bib \
    --citeproc \
    --wrap=preserve \
    --top-level-division=section \
    --resource-path=.. \
    --natbib
else
  echo "appendix.tex already exists, skipping pandoc generation"
fi

pdflatex paper.tex

# Process bibliography
bibtex paper

# Second LaTeX pass (for cross-references)
pdflatex paper.tex

# Third LaTeX pass (to ensure everything is resolved)
pdflatex paper.tex