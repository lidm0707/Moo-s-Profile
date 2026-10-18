#!/usr/bin/env node
// Minifies the hashed main-*.css in dist/public/assets in place.
// Safe transforms only: comments, whitespace, trailing semicolons.
// Run AFTER `dx bundle` (needs the hashed filename).

import { readdir, readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const ASSETS_DIR = path.join(path.resolve(path.dirname(__filename), ".."), "dist", "public", "assets");

function minifyCss(css) {
  return css
    .replace(/\/\*[\s\S]*?\*\//g, "")
    .replace(/\s+/g, " ")
    .replace(/\s*([{}:;,>~])\s*/g, "$1")
    .replace(/;}/g, "}")
    .trim();
}

const files = (await readdir(ASSETS_DIR)).filter((f) => /^main-.*\.css$/.test(f));
if (files.length === 0) {
  throw new Error("No main-*.css found in dist/public/assets — run `dx bundle` first");
}
for (const file of files) {
  const filePath = path.join(ASSETS_DIR, file);
  const before = await readFile(filePath, "utf8");
  const after = minifyCss(before);
  await writeFile(filePath, after, "utf8");
  console.log(`Minified ${file}: ${(before.length / 1024).toFixed(1)} KiB -> ${(after.length / 1024).toFixed(1)} KiB`);
}
