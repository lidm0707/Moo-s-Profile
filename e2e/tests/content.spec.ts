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

function publishedSlugs(min: number): string[] {
  const snapshot: Snapshot = JSON.parse(readFileSync(SNAPSHOT_PATH, "utf8"));
  const slugs = snapshot.content
    .filter((c) => c.status === "published" && c.slug)
    .map((c) => c.slug);
  if (slugs.length < min)
    throw new Error(`Need ${min} published content in snapshot — run \`make snapshot\``);
  return slugs;
}

// Index of the first tag chip whose tag has ≥ 2 published contents, so the
// filtered topics list allows switching between at least two articles.
type FullSnapshot = Snapshot & {
  tags: { id: number }[];
  content_tags: { content_id: number | null; tag_id: number }[];
};

function firstTagChipWithTwoTopics(): number {
  const snapshot: FullSnapshot = JSON.parse(readFileSync(SNAPSHOT_PATH, "utf8"));
  const publishedIds = new Set(
    snapshot.content
      .filter((c) => c.status === "published")
      .map((c) => c.id),
  );
  const countPerTag = new Map<number, number>();
  for (const ct of snapshot.content_tags) {
    if (ct.content_id !== null && publishedIds.has(ct.content_id))
      countPerTag.set(ct.tag_id, (countPerTag.get(ct.tag_id) ?? 0) + 1);
  }
  const index = snapshot.tags.findIndex((t) => (countPerTag.get(t.id) ?? 0) >= 2);
  if (index < 0) throw new Error("No tag with ≥2 published contents in snapshot");
  return index;
}

function firstPublishedSlug(): string {
  return publishedSlugs(1)[0];
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

  test("switching topics from the sidebar updates the viewer on a slug route", async ({
    page,
  }) => {
    // Regression: navigating between slugs reused the mounted component, so
    // the deep-link effect was skipped and the center viewer kept the old
    // article.
    const chip = page.locator(".tag-chip").nth(firstTagChipWithTwoTopics());
    await page.goto("/content");
    await chip.click();

    const topics = page.locator("a.content-topics-item");
    await expect(topics.first()).toBeVisible();
    expect(await topics.count()).toBeGreaterThanOrEqual(2);

    const openFirst = topics.nth(0);
    await openFirst.click();
    await expect(page.locator(".content-viewer-title")).toBeVisible();
    const firstTitle = await page.locator(".content-viewer-title").innerText();

    const target = topics.nth(1);
    const secondTitle = await target.locator(".content-topics-item-title").innerText();
    expect(secondTitle).not.toBe(firstTitle);

    await target.click();

    await expect(page.locator(".content-viewer-title")).toHaveText(secondTitle);
    await expect(page).toHaveURL(/\/content\/[^/?]+$/);
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
