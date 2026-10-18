import { defineConfig } from "@playwright/test";

// Target the locally-served production build (docker compose `web` service,
// or any static server on BASE_URL). No webServer here: the app must be
// built (`make build`) and served before the suite runs.
const BASE_URL = process.env.BASE_URL ?? "http://localhost:8080";

export default defineConfig({
  testDir: "./tests",
  timeout: 30_000,
  retries: 0,
  workers: 4,
  reporter: process.env.CI ? "list" : "list",
  use: {
    baseURL: BASE_URL,
    // WASM hydration takes a moment; expect() auto-retries until this cap.
    actionTimeout: 15_000,
    navigationTimeout: 15_000,
  },
});
