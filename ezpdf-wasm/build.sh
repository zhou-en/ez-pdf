#!/usr/bin/env bash
# Builds the two wasm packages the web app consumes.
#
#   ops       — merge/split/remove/rotate/info. ~90 KB gzipped, loaded eagerly.
#   markdown  — adds pdf-inspector. ~2.3 MB gzipped, loaded only when the user
#               picks Markdown. It embeds glyph tables and CMaps, which is the
#               entire 26x difference between the two.
#
# Output goes straight into the Next.js app so the artifacts are committed and
# Vercel never needs a Rust toolchain.
set -euo pipefail

cd "$(dirname "$0")"
OUT="../ezpdf-web/src/wasm"

echo "==> ops (no default features)"
wasm-pack build --target web --release --out-dir "$OUT/ops" \
  --out-name ezpdf_ops -- --no-default-features

echo "==> markdown (with pdf-inspector)"
wasm-pack build --target web --release --out-dir "$OUT/markdown" \
  --out-name ezpdf_markdown

# wasm-pack writes a .gitignore into each out-dir; these artifacts are
# deliberately committed, so remove it.
rm -f "$OUT/ops/.gitignore" "$OUT/markdown/.gitignore"

echo
printf '%-12s %10s %10s\n' PACKAGE RAW GZIP
for d in ops markdown; do
  f=$(ls "$OUT/$d"/*.wasm)
  printf '%-12s %9.2fM %9.2fM\n' "$d" \
    "$(echo "$(stat -c%s "$f")/1048576" | bc -l)" \
    "$(echo "$(gzip -9 -c "$f" | wc -c)/1048576" | bc -l)"
done
