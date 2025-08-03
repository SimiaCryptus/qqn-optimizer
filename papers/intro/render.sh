#!/bin/bash

# Clean up any existing files
rm -f content.tex appendix.tex paper.aux paper.bbl paper.blg paper.log paper.pdf
rm -rf results/
mkdir -p results/latex results/plots

# Copy referenced files from results directory
RESULTS_DIR="../../results/full_all_optimizers_20250803_142238"

# Copy LaTeX files
if [ -f "$RESULTS_DIR/latex/comparison_matrix.tex" ]; then
  cp "$RESULTS_DIR/latex/comparison_matrix.tex" results/latex/
fi
if [ -f "$RESULTS_DIR/latex/family_vs_family_matrix.tex" ]; then
  cp "$RESULTS_DIR/latex/family_vs_family_matrix.tex" results/latex/
fi
if [ -f "$RESULTS_DIR/latex/Rosenbrock_5D_performance.tex" ]; then
  cp "$RESULTS_DIR/latex/Rosenbrock_5D_performance.tex" results/latex/
fi

# Copy plot files
if [ -f "$RESULTS_DIR/plots/Rosenbrock_5D/log_convergence.png" ]; then
  mkdir -p results/plots/Rosenbrock_5D/
  cp "$RESULTS_DIR/plots/Rosenbrock_5D/log_convergence.png" results/plots/Rosenbrock_5D/
fi

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
# First LaTeX pass

pdflatex -interaction=nonstopmode paper.tex

# Process bibliography
bibtex paper

# Second LaTeX pass (for cross-references)
pdflatex -interaction=nonstopmode paper.tex

# Third LaTeX pass (to ensure everything is resolved)
pdflatex -interaction=nonstopmode paper.tex

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
    echo -e "# Rendered Paper\n\nPDF conversion not available. Please install pdftotext or PyMuPDF." > paper_rendered.md
  fi
else
  echo "Warning: paper.pdf not found, skipping markdown conversion"
fi
# Prepare arXiv submission zip
if [ -f paper.pdf ]; then
  echo "Preparing arXiv submission package..."
  # Create temporary directory for arXiv files
  ARXIV_DIR="arxiv_submission"
  rm -rf "$ARXIV_DIR"
  mkdir -p "$ARXIV_DIR"
  # Copy main LaTeX files
  cp paper.tex "$ARXIV_DIR/"
  cp content.tex "$ARXIV_DIR/"
  cp appendix.tex "$ARXIV_DIR/"
  cp references.bib "$ARXIV_DIR/"
  # Copy any style files if they exist
  if [ -f "*.cls" ]; then cp *.cls "$ARXIV_DIR/"; fi
  if [ -f "*.sty" ]; then cp *.sty "$ARXIV_DIR/"; fi
  # Copy results directory (figures and tables)
  if [ -d "results" ]; then
    cp -r results "$ARXIV_DIR/"
  fi
  # Copy any additional image files
  if [ -d "figures" ]; then cp -r figures "$ARXIV_DIR/"; fi
  if [ -d "images" ]; then cp -r images "$ARXIV_DIR/"; fi
  # Create the zip file
  cd "$ARXIV_DIR"
  zip -r "../arxiv_submission.zip" .
  cd ..
  # Clean up temporary directory
  rm -rf "$ARXIV_DIR"
  echo "arXiv submission package created: arxiv_submission.zip"
  echo "Contents:"
  unzip -l arxiv_submission.zip
else
  echo "Warning: paper.pdf not found, skipping arXiv package creation"
fi