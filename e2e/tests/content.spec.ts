import { readFileSync } from "node:fs";
import path from "node:path";
import { expect, test } from "@playwright/test";

// Slugs come from the same snapshot embedded in the build, so tests always
// exercise real published content.
const SNAPSHOT_PATH =
  process.env.SNAPSHOT_PATH ?? path.join(process.cwd(), "..", "assets", "content_snapshot.json");

type Snapshot = {
  content: { id: number | null; slug: string; title: string; status: string }[];
};

function firstPublishedSlug(): string {
  const snapshot: Snapshot = JSON.parse(readFileSync(SNAPSHOT_PATH, "utf8"));
  const found = snapshot.content.find((c) => c.status === "published" && c.slug);
  if (!found) throw new Error("No published content in snapshot — run `make snapshot`");
  return found.slug;
}

const NOT_FOUND_TEXT = "Content not found";

test.describe("Content page", () => {
  test("home redirects to /content and shows the tag bar", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveURL(/\/content\/?$/);
    await expect(page.locator(".tag-chip").first()).toBeVisible();
  });

  test("default view lists published articles as crawlable links", async ({ page }) => {
    await page.goto("/content");
    const topics = page.locator("a.content-topics-item");
    await expect(topics.first()).toBeVisible();
    expect(await topics.count()).toBeGreaterThan(0);
    for (let i = 0; i < Math.min(await topics.count(), 5); i++) {
      await expect(topics.nth(i)).toHaveAttribute("href", /\/content\/.+/);
    }
  });

  test("clicking a topic opens the article at its slug URL", async ({ page }) => {
    await page.goto("/content");
    await page.locator("a.content-topics-item").first().click();
    await expect(page).toHaveURL(/\/content\/[^/?]+$/);
    await expect(page.locator(".content-viewer-title")).toBeVisible();
    await expect(page.locator(".content-viewer-body")).not.toBeEmpty();
  });

  test("slug URL renders the article directly (no redirect)", async ({ page }) => {
    const slug = firstPublishedSlug();
    await page.goto(`/content/${slug}`);
    await expect(page).toHaveURL(new RegExp(`/content/${slug}/?$`));
    await expect(page.locator(".content-viewer-title")).toBeVisible();
  });

  test("selecting a tag filters the topics list", async ({ page }) => {
    await page.goto("/content");
    await page.locator(".tag-chip").first().click();
    const topics = page.locator("a.content-topics-item");
    await expect(topics.first()).toBeVisible();
    expect(await topics.count()).toBeGreaterThan(0);
  });

  test("unknown slug shows the not-found state", async ({ page }) => {
    await page.goto("/content/no-such-article-xyz");
    await expect(page.locator(".empty-state")).toHaveText(NOT_FOUND_TEXT);
  });

  test("page has a main landmark and html lang", async ({ page }) => {
    await page.goto("/content");
    await expect(page.locator("main.content-layout")).toBeVisible();
    const lang = await page.locator("html").getAttribute("lang");
    expect(lang).not.toBeNull();
  });
});
