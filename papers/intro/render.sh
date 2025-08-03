#!/bin/bash

# Clean up any existing files
rm -f content.tex appendix.tex paper.aux paper.bbl paper.blg paper.log paper.pdf
rm -rf results/
mkdir -p results/latex results/plots

# Copy referenced files from results directory
RESULTS_DIR="../../results/full_all_optimizers_20250803_144146"

# Function to strip LaTeX document wrapper
strip_latex_wrapper() {
  local input_file="$1"
  local output_file="$2"

  perl -0777 -ne 'print $1 if /.*\\begin\{document\}(.*)\\end\{document\}.*/s' "$input_file" > "$output_file"

}

# Copy LaTeX files and strip document wrappers
if [ -f "$RESULTS_DIR/latex/comparison_matrix.tex" ]; then
  strip_latex_wrapper "$RESULTS_DIR/latex/comparison_matrix.tex" "results/latex/comparison_matrix.tex"
fi

if [ -f "$RESULTS_DIR/latex/family_vs_family_matrix.tex" ]; then
  strip_latex_wrapper "$RESULTS_DIR/latex/family_vs_family_matrix.tex" "results/latex/family_vs_family_matrix.tex"
fi

if [ -f "$RESULTS_DIR/latex/Rosenbrock_5D_performance.tex" ]; then
  strip_latex_wrapper "$RESULTS_DIR/latex/Rosenbrock_5D_performance.tex" "results/latex/Rosenbrock_5D_performance.tex"
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

# Preflight checks for arXiv submission
if [ -f paper.pdf ]; then
  echo "Running preflight checks..."

  # Check file sizes (arXiv has limits)
  PDF_SIZE=$(stat -f%z paper.pdf 2>/dev/null || stat -c%s paper.pdf 2>/dev/null)
  if [ "$PDF_SIZE" -gt 52428800 ]; then  # 50MB limit
    echo "WARNING: PDF is larger than 50MB ($((PDF_SIZE/1048576))MB). arXiv may reject it."
  fi

  # Check for common LaTeX issues
  if grep -q "LaTeX Warning" paper.log; then
    echo "WARNING: LaTeX warnings found in paper.log:"
    grep "LaTeX Warning" paper.log
  fi

  if grep -q "LaTeX Error" paper.log; then
    echo "ERROR: LaTeX errors found in paper.log:"
    grep "LaTeX Error" paper.log
    echo "Fix these errors before submitting to arXiv"
  fi

  # Check for undefined references
  if grep -q "undefined" paper.log; then
    echo "WARNING: Undefined references found:"
    grep "undefined" paper.log
  fi

  # Check bibliography compilation
  if [ -f paper.blg ]; then
    if grep -q "error" paper.blg; then
      echo "WARNING: BibTeX errors found in paper.blg:"
      grep -i "error" paper.blg
    fi
  fi

  # Verify all referenced files exist
  echo "Checking file dependencies..."
  MISSING_FILES=0

  # Check for missing figures
  if grep -q "includegraphics" paper.tex content.tex appendix.tex 2>/dev/null; then
    for file in $(grep -h "includegraphics" paper.tex content.tex appendix.tex 2>/dev/null | sed 's/.*{\([^}]*\)}.*/\1/' | sort -u); do
      # Try common extensions if no extension provided
      if [[ ! "$file" =~ \. ]]; then
        found=false
        for ext in .png .pdf .jpg .jpeg .eps; do
          if [ -f "${file}${ext}" ]; then
            found=true
            break
          fi
        done
        if [ "$found" = false ]; then
          echo "WARNING: Figure file not found: $file (tried common extensions)"
          MISSING_FILES=$((MISSING_FILES + 1))
        fi
      elif [ ! -f "$file" ]; then
        echo "WARNING: Figure file not found: $file"
        MISSING_FILES=$((MISSING_FILES + 1))
      fi
    done
  fi

  # Check for missing input/include files
  for file in $(grep -h "\\\\input{\\|\\\\include{" paper.tex 2>/dev/null | sed 's/.*{\([^}]*\)}.*/\1/' | sort -u); do
    if [ ! -f "$file" ] && [ ! -f "$file.tex" ]; then
      echo "WARNING: Input file not found: $file"
      MISSING_FILES=$((MISSING_FILES + 1))
    fi
  done

  # Check arXiv package contents
  if [ -f arxiv_submission.zip ]; then
    echo "Verifying arXiv package contents..."

    # Check that main files are included
    if ! unzip -l arxiv_submission.zip | grep -q "paper.tex"; then
      echo "ERROR: paper.tex not found in arXiv package"
    fi

    if ! unzip -l arxiv_submission.zip | grep -q "references.bib"; then
      echo "ERROR: references.bib not found in arXiv package"
    fi

    # Check package size
    ZIP_SIZE=$(stat -f%z arxiv_submission.zip 2>/dev/null || stat -c%s arxiv_submission.zip 2>/dev/null)
    if [ "$ZIP_SIZE" -gt 52428800 ]; then  # 50MB limit
      echo "WARNING: arXiv package is larger than 50MB ($((ZIP_SIZE/1048576))MB)"
    fi
  fi

  # Summary
  echo "Preflight check complete."
  if [ "$MISSING_FILES" -gt 0 ]; then
    echo "WARNING: $MISSING_FILES missing file(s) detected. Review before submission."
  else
    echo "✓ All file dependencies appear to be satisfied."
  fi

  # Final recommendations
  echo ""
  echo "Before submitting to arXiv:"
  echo "1. Review the generated PDF for formatting issues"
  echo "2. Verify all figures and tables display correctly"
  echo "3. Check that all citations are properly formatted"
  echo "4. Ensure the abstract and title are finalized"
  echo "5. Consider running a spell check on the source files"

else
  echo "Skipping preflight checks - no PDF generated"
fi