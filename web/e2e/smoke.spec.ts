import { expect, test } from "@playwright/test";

test("loads the project shell", async ({ page }) => {
  await page.goto("/");

  await expect(
    page.getByRole("heading", { name: "Unciv Rust/Web" }),
  ).toBeVisible();
  await expect(
    page.getByText("Compatibility target: Unciv 4.21.10"),
  ).toBeVisible();
});
