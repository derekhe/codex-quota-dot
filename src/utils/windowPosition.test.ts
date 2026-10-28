import { describe, expect, it } from "vitest";
import { visiblePosition } from "./windowPosition";

const primary = { position: { x: 0, y: 0 }, size: { width: 1920, height: 1040 } };
const left = { position: { x: -1600, y: 0 }, size: { width: 1600, height: 900 } };
const size = { width: 64, height: 64 };

describe("visiblePosition", () => {
  it("keeps positions on connected displays, including negative coordinates", () => {
    expect(visiblePosition({ x: -1000, y: 200 }, size, [primary, left], primary)).toEqual({ x: -1000, y: 200 });
    expect(visiblePosition({ x: -30, y: 100 }, size, [primary], primary)).toEqual({ x: -30, y: 100 });
  });
  it("moves a disconnected display position into the primary work area", () => {
    expect(visiblePosition({ x: -1000, y: 200 }, size, [primary], primary)).toEqual({ x: 0, y: 200 });
    expect(visiblePosition({ x: 3000, y: 1500 }, size, [primary], primary)).toEqual({ x: 1856, y: 976 });
  });
});
