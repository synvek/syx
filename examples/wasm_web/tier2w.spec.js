// @ts-check
const { test, expect } = require('@playwright/test');
const path = require('path');
const fs = require('fs');

const GUI_CALLBACK_SOURCE = `func onBtnClick() {
    print("TIER2W_HANDLER_RAN")
}
func main() {
    let win = __syx_gui_winit_window_create("tier2w")
    __syx_gui_winit_canvas_draw(win, "blue")
    let field = __syx_gui_text_field_create(win, "Type here...")
    let btn = __syx_gui_button_create(win, "Click me")
    __syx_gui_on_click(win, btn, "onBtnClick")
    __syx_gui_click_inject(win, btn)
    print(__syx_gui_run_loop_once(win))
    __syx_gui_key_inject(win, "x")
    __syx_gui_key_inject(win, "y")
    print(__syx_gui_widget_get_text(win, field))
}`;

function requireArtifacts() {
  const indexPath = path.join(__dirname, 'index.html');
  const wasmPath = path.join(__dirname, 'compiler.wasm');
  if (!fs.existsSync(indexPath) || !fs.existsSync(wasmPath)) {
    throw new Error(
      'Phase 102 hard gate: index.html or compiler.wasm missing (build syx-compiler-wasm first)'
    );
  }
  return { indexPath, wasmPath };
}

test('GUI wasm click invoke + field text (tier2w)', async ({ page }) => {
  const { indexPath } = requireArtifacts();
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(GUI_CALLBACK_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('TIER2W_HANDLER_RAN', { timeout: 30000 });
  await expect(out).toContainText('invoked:onBtnClick', { timeout: 30000 });
  await expect(out).toContainText('xy', { timeout: 30000 });
});
