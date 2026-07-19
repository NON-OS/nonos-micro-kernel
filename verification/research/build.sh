#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# Regenerate the branded Word document from the Markdown source. Requires
# pandoc. The NONOS brand (Poppins typeface, teal headings) is carried by
# branded-reference.docx, built from nonos.systems/brand-guidelines.

set -euo pipefail
cd "$(dirname "$0")"

pandoc nonos-verification.md \
  --citeproc --bibliography=references.bib \
  --reference-doc=branded-reference.docx \
  --number-sections --toc --toc-depth=2 \
  -M reference-section-title="References" \
  -o nonos-verification.docx

echo "wrote nonos-verification.docx"
