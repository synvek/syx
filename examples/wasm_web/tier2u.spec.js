// @ts-check
const { test, expect } = require('@playwright/test');
const path = require('path');
const fs = require('fs');

const GUI_GUIDEMO_SUBSET = `func main() {
    let win = __syx_gui_winit_window_create("tier2u")
    print("window: " + win)
    __syx_gui_winit_canvas_draw(win, "blue")
    let label = __syx_gui_label_create(win, "Hello SYX")
    print("label: " + label)
    let btn = __syx_gui_button_create(win, "Click me")
    print(__syx_gui_on_click(win, btn, "handler"))
    print(__syx_gui_winit_event_pump(win))
    let px = __syx_gui_pixel_read(win)
    print(px)
}`;

function requireArtifacts() {
  const indexPath = path.join(__dirname, 'index.html');
  const wasmPath = path.join(__dirname, 'compiler.wasm');
  if (!fs.existsSync(indexPath) || !fs.existsSync(wasmPath)) {
    throw new Error(
      'Phase 99 hard gate: index.html or compiler.wasm missing (build syx-compiler-wasm first)'
    );
  }
  return { indexPath, wasmPath };
}

test('GUI wasm guidemo subset window draw label click pump pixel (tier2u)', async ({ page }) => {
  const { indexPath } = requireArtifacts();
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(GUI_GUIDEMO_SUBSET);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('window:', { timeout: 30000 });
  await expect(out).toContainText('label:', { timeout: 30000 });
  await expect(out).toContainText('true', { timeout: 30000 });
  await expect(out).toContainText('4278190335', { timeout: 30000 });
});
