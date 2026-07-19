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

out="nonos-zkolang.docx"

pandoc nonos-zkolang.md \
  --citeproc --bibliography=zkolang-references.bib \
  --reference-doc=branded-reference.docx \
  --number-sections --toc --toc-depth=2 \
  -o "$out"

# ---------------------------------------------------------------------------
# Post-process: cover page + branded footer. Unpack, edit the XML, repack.
# ---------------------------------------------------------------------------
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
unzip -q "$out" -d "$work"

doc="$work/word/document.xml"

# Cover logo: embed the symmetric teal logomark, centered, with a teal "NØNOS"
# text wordmark beneath it. The full-lockup PNG carries its wordmark as white
# glyphs on transparency (invisible on a white page) and packs the mark into the
# left third, so centering its box pushes the mark off-centre; the square mark
# centres cleanly. The image part, its content-type, and its relationship are
# added, then the lockup paragraphs are inserted at the top of the body.
# Dimensions are in EMU (914400 per inch); the mark source is 120x134.
mkdir -p "$work/word/media"
cp nonos-logomark.png "$work/word/media/logo.png"
grep -q 'Extension="png"' "$work/[Content_Types].xml" || \
  perl -0pi -e 's{(<Types[^>]*>)}{$1<Default Extension="png" ContentType="image/png"/>}' "$work/[Content_Types].xml"
perl -0pi -e 's{</Relationships>}{<Relationship Id="rIdNonosLogo" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/logo.png"/></Relationships>}' \
  "$work/word/_rels/document.xml.rels"
logo_frag="$work/logo_frag.xml"
cat > "$logo_frag" <<'XML'
<w:p><w:pPr><w:jc w:val="center"/><w:spacing w:before="2600" w:after="120"/></w:pPr><w:r><w:drawing><wp:inline distT="0" distB="0" distL="0" distR="0"><wp:extent cx="1143000" cy="1276650"/><wp:effectExtent l="0" t="0" r="0" b="0"/><wp:docPr id="101" name="NONOS logomark"/><wp:cNvGraphicFramePr/><a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic><pic:nvPicPr><pic:cNvPr id="101" name="logo.png"/><pic:cNvPicPr/></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdNonosLogo"/><a:stretch><a:fillRect/></a:stretch></pic:blipFill><pic:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1143000" cy="1276650"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing></w:r></w:p>
<w:p><w:pPr><w:jc w:val="center"/><w:spacing w:before="0" w:after="800"/></w:pPr><w:r><w:rPr><w:b/><w:color w:val="2E5C5C"/><w:sz w:val="48"/><w:spacing w:val="80"/></w:rPr><w:t xml:space="preserve">NØNOS</w:t></w:r></w:p>
XML
LOGO_FRAG="$logo_frag" perl -0pi -e 'BEGIN{local $/; open(my $f,"<",$ENV{LOGO_FRAG}); our $L=<$f>; close $f; chomp $L} s{(<w:body>)}{$1$L}' "$doc"

# Page layout: page one is a clean cover (logo, title, subtitle, author, date);
# the abstract takes page two; the table of contents gets its own page three;
# the body begins on page four. Insert a page break before the abstract, before
# the contents heading, and before the first top-level heading of the body.
perl -0pi -e 's{(<w:p\b[^>]*>(?:(?!</w:p>).)*?<w:pStyle w:val="AbstractTitle")}{<w:p><w:r><w:br w:type="page"/></w:r></w:p>$1}s' "$doc"
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
    <w:r><w:rPr><w:color w:val="2E5C5C"/><w:sz w:val="16"/></w:rPr><w:t xml:space="preserve">zKølang Language   ·   </w:t></w:r>
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
