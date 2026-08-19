import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { App } from "./App";

describe("App", () => {
  it("identifies the pinned compatibility baseline", () => {
    render(<App />);

    expect(
      screen.getByRole("heading", { name: "Unciv Rust/Web" }),
    ).toBeVisible();
    expect(
      screen.getByText("Compatibility target: Unciv 4.21.10"),
    ).toBeVisible();
  });
});
