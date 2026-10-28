export interface Rect {
  position: { x: number; y: number };
  size: { width: number; height: number };
}

// Preserve a window that is at least partly visible on any active display.
export function visiblePosition(position: Rect["position"], size: Rect["size"], areas: Rect[], fallback: Rect): Rect["position"] {
  const visible = areas.some(({ position: area, size: bounds }) =>
    position.x < area.x + bounds.width && position.x + size.width > area.x &&
    position.y < area.y + bounds.height && position.y + size.height > area.y,
  );
  if (visible) return position;
  return {
    x: Math.max(fallback.position.x, Math.min(position.x, fallback.position.x + fallback.size.width - size.width)),
    y: Math.max(fallback.position.y, Math.min(position.y, fallback.position.y + fallback.size.height - size.height)),
  };
}
