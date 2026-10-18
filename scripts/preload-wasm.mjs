// Inject <link rel="preload"> for the hashed wasm bundle into dist/public/index.html.
// Without it the browser only discovers the 1.6MB wasm after the loader JS
// downloads and executes, serializing the critical path on mobile.
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const DIST = "dist/public";
const MARKER = "<!-- wasm-preload -->";
const index_path = join(DIST, "index.html");

const wasm = readdirSync(join(DIST, "assets")).find((f) => f.endsWith(".wasm"));
if (!wasm) {
    console.error("preload-wasm: no .wasm found in dist/public/assets");
    process.exit(1);
}

let html = readFileSync(index_path, "utf8");
html = html.replace(MARKER, "").replace(/\s*<link rel="preload" as="fetch"[^>]*my_profile[^>]*>\n?/g, "");

const tag = `    ${MARKER}\n    <link rel="preload" as="fetch" href="/assets/${wasm}" crossorigin>\n`;
html = html.replace("</head>", `${tag}</head>`);

writeFileSync(index_path, html);
console.log(`preload-wasm: injected preload for /assets/${wasm}`);
