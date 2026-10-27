// @ts-check
const { test, expect } = require('@playwright/test');
const path = require('path');
const fs = require('fs');

const GUI_MULTISTEP_SOURCE = `func main() {
    let win = __syx_gui_winit_window_create("tier2t")
    __syx_gui_winit_canvas_draw(win, "red")
    let px1 = __syx_gui_pixel_read(win)
    print(px1)
    let btn = __syx_gui_button_create(win, "Go")
    print(__syx_gui_on_click(win, btn, "handler"))
    print(__syx_gui_winit_event_pump(win))
    __syx_gui_winit_canvas_draw(win, "blue")
    let px2 = __syx_gui_pixel_read(win)
    print(px2)
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

test('GUI wasm multistep create draw click pump pixel (tier2t)', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(GUI_MULTISTEP_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('4278190335', { timeout: 30000 });
  await expect(out).toContainText('true', { timeout: 30000 });
  await expect(out).toContainText('4294901760', { timeout: 30000 });
});
