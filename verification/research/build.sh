#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# Regenerate the branded Word document from the Markdown source. Requires
# pandoc and a zip/unzip. The NONOS brand (Poppins typeface, teal accents) is
# carried by branded-reference.docx, built from nonos.systems/brand-guidelines;
# a post-processing pass adds a cover page and a branded footer with page
# numbers, which pandoc's reference-doc mechanism cannot express on its own.

set -euo pipefail
cd "$(dirname "$0")"

out="nonos-verification.docx"

pandoc nonos-verification.md \
  --citeproc --bibliography=references.bib \
  --reference-doc=branded-reference.docx \
  --number-sections --toc --toc-depth=2 \
  -M reference-section-title="References" \
  -o "$out"

# ---------------------------------------------------------------------------
# Post-process: cover page + branded footer. Unpack, edit the XML, repack.
# ---------------------------------------------------------------------------
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
unzip -q "$out" -d "$work"

doc="$work/word/document.xml"

# Page layout: the title block (title, subtitle, author, date, abstract) stands
# alone on page one; the table of contents gets its own page two; the body
# begins on page three. Insert a page break before the contents heading and
# another before the first top-level heading of the body.
perl -0pi -e 's{(<w:p[^>]*><w:pPr><w:pStyle w:val="TOCHeading")}{<w:p><w:r><w:br w:type="page"/></w:r></w:p>$1}' "$doc"
perl -0pi -e 's{(<w:p[^>]*><w:pPr><w:pStyle w:val="Heading1")}{<w:p><w:r><w:br w:type="page"/></w:r></w:p>$1}' "$doc"

# Footer part: centered "NØNOS" wordmark, a thin rule, and the page number.
cat > "$work/word/footer1.xml" <<'XML'
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:ftr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:p>
    <w:pPr>
      <w:pBdr><w:top w:val="single" w:sz="4" w:space="4" w:color="2E5C5C"/></w:pBdr>
      <w:jc w:val="center"/>
      <w:rPr><w:color w:val="2E5C5C"/><w:sz w:val="16"/></w:rPr>
    </w:pPr>
    <w:r><w:rPr><w:b/><w:color w:val="2E5C5C"/><w:sz w:val="16"/></w:rPr><w:t xml:space="preserve">NØNOS   </w:t></w:r>
    <w:r><w:rPr><w:color w:val="2E5C5C"/><w:sz w:val="16"/></w:rPr><w:t xml:space="preserve">Verification Architecture   ·   </w:t></w:r>
    <w:r><w:fldChar w:fldCharType="begin"/></w:r>
    <w:r><w:instrText xml:space="preserve"> PAGE </w:instrText></w:r>
    <w:r><w:fldChar w:fldCharType="end"/></w:r>
  </w:p>
</w:ftr>
XML

# Wire the footer: content-type, relationship, and a sectPr reference.
perl -0pi -e 's{</Types>}{<Override PartName="/word/footer1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"/></Types>}' \
  "$work/[Content_Types].xml"
perl -0pi -e 's{</Relationships>}{<Relationship Id="rIdFooterNonos" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" Target="footer1.xml"/></Relationships>}' \
  "$work/word/_rels/document.xml.rels"
perl -0pi -e 's{<w:sectPr>}{<w:sectPr><w:footerReference w:type="default" r:id="rIdFooterNonos"/>}' "$doc"

# Repack (mimetype/content-types first is not required for docx; order is fine).
( cd "$work" && rm -f "../$out" && zip -q -r -X "$OLDPWD/$out" '[Content_Types].xml' _rels docProps word customXml 2>/dev/null )

echo "wrote $out (cover page + branded footer)"
