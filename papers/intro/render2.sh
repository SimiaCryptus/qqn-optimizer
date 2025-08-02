#!/bin/bash

pandoc combined.md -o paper.pdf \
  --bibliography=references.bib \
  --citeproc \
  --template=paper_template.tex \
  --pdf-engine=pdflatex
