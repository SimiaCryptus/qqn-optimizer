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

# Convert the final PDF back to markdown for agent analysis
if [ -f paper.pdf ]; then
  # Try pdftotext first (part of poppler-utils)
  if command -v pdftotext >/dev/null 2>&1; then
    pdftotext -layout paper.pdf paper_rendered.txt
    # Convert txt to md (just rename with .md extension for markdown formatting)
    mv paper_rendered.txt paper_rendered.md
    echo "PDF converted to paper_rendered.md using pdftotext"
  # Fallback to pymupdf if available
  elif command -v python3 >/dev/null 2>&1 && python3 -c "import fitz" >/dev/null 2>&1; then
    python3 -c "
import fitz
doc = fitz.open('paper.pdf')
text = ''
for page in doc:
    text += page.get_text()
with open('paper_rendered.md', 'w') as f:
    f.write(text)
doc.close()
"
    echo "PDF converted to paper_rendered.md using PyMuPDF"
  else
    echo "Warning: No PDF-to-text converter found. Install poppler-utils (pdftotext) or PyMuPDF"
    echo "Creating placeholder paper_rendered.md"
    echo "# Rendered Paper\n\nPDF conversion not available. Please install pdftotext or PyMuPDF." > paper_rendered.md
  fi
else
  echo "Warning: paper.pdf not found, skipping markdown conversion"
fi