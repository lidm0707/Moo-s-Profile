import { expect, test } from "@playwright/test";

// Ad slot on the content page renders an in-app dismiss button so mobile
// users can close dark ads that cover the text.
test.describe("Ad close button", () => {
  test("content page shows the ad wrap with a close button", async ({ page }) => {
    await page.goto("/content");
    const adWrap = page.locator(".adsense-wrap").first();
    await expect(adWrap).toBeVisible();
    await expect(adWrap.locator(".adsense-close")).toBeVisible();
    await expect(adWrap.locator("ins.adsbygoogle")).toHaveAttribute(
      "data-ad-slot",
      /[^_\s]/,
    );
  });

  test("clicking the close button removes the ad wrap", async ({ page }) => {
    await page.goto("/content");
    const close = page.locator(".adsense-close").first();
    await expect(close).toBeVisible();
    await close.click();
    await expect(page.locator(".adsense-wrap")).toHaveCount(0);
  });

  // Mobile: the rail is a fixed bottom banner (z-index 400) that covers the
  // viewport — it must be dismissible or it blocks the page content.
  test("clicking the rail close button hides the fixed ad rail", async ({ page }) => {
    await page.goto("/content");
    const rail = page.locator(".ad-rail");
    await expect(rail).toBeVisible();
    await rail.locator(".ad-rail-close").click();
    await expect(rail).not.toBeVisible();
  });
});
