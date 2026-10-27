// @ts-check
const { test, expect } = require('@playwright/test');
const path = require('path');
const fs = require('fs');

const GUI_PIXEL_SOURCE = `func main() {
    let win = __syx_gui_winit_window_create("tier2s")
    __syx_gui_winit_canvas_draw(win, "red")
    print(__syx_gui_pixel_read(win))
}`;

const GUI_CLICK_SOURCE = `func main() {
    let win = __syx_gui_winit_window_create("tier2s-click")
    let btn = __syx_gui_button_create(win, "Go")
    print(__syx_gui_on_click(win, btn, "handler"))
    print(__syx_gui_pixel_read(win))
}`;

function requireArtifacts(test) {
  const indexPath = path.join(__dirname, 'index.html');
  const wasmPath = path.join(__dirname, 'compiler.wasm');
  test.skip(
    !fs.existsSync(indexPath) || !fs.existsSync(wasmPath),
    'index.html or compiler.wasm missing (build syx-compiler-wasm first)'
  );
  return { indexPath, wasmPath };
}

test('GUI wasm red pixel numeric (tier2s)', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(GUI_PIXEL_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('4278190335', { timeout: 30000 });
});

test('GUI wasm onClick + pixel numeric (tier2s)', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(GUI_CLICK_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('true', { timeout: 30000 });
  await expect(out).toContainText('4278190335', { timeout: 30000 });
});
