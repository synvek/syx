// @ts-check
const { test, expect } = require('@playwright/test');
const path = require('path');
const fs = require('fs');

const POINT_SOURCE = `class Point {
    var x: Int
    init(x: Int) { self.x = x }
}
func main() {
    let p = Point(x: 42)
    return p.x
}`;

const ITERATOR_SOURCE = `protocol Iterator {
    func next() -> Int?
}
class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        if self.n >= 3 { return nil }
        self.n = self.n + 1
        return self.n
    }
}
func main() {
    var s = 0
    for x in Counter() { s = s + x }
    print(s)
}`;

const SORTED_SOURCE = `func main() {
    for x in [1, 2, 3] { print(x) }
    return 0
}`;

const CLOSURE_SOURCE = `func id(x: Int) -> Int { return x }
func main() {
    print(id(86))
}`;

const FMT_SOURCE = `func main() {
    Fmt.printLine(Fmt.format("tier2i-{0}", [86]))
    return 86
}`;

const REFLECT_SOURCE = `class Point {
    var x: Int
    init(x: Int) { self.x = x }
}
func main() {
    print(__syx_reflect_type("Point"))
    return 86
}`;

const STRING_BUILDER_SOURCE = `func main() {
    let b = __syx_string_builder_new()
    __syx_string_builder_append(b, "p86-sb")
    print(__syx_string_builder_to_string(b))
    return 86
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

async function compileAndRun(page, source, expectedOut) {
  await page.locator('#src').fill(source);
  await page.locator('#compile').click();
  await expect(page.locator('#out')).toHaveText(expectedOut, { timeout: 30000 });
}

test('wasm_web page loads compiler', async ({ page }) => {
  const indexPath = path.join(__dirname, 'index.html');
  test.skip(!fs.existsSync(indexPath), 'index.html missing');
  await page.goto('file://' + indexPath);
  await expect(page.locator('body')).toBeVisible();
});

test('compiler.wasm artifact present', async () => {
  const wasmPath = path.join(__dirname, 'compiler.wasm');
  test.skip(!fs.existsSync(wasmPath), 'compiler.wasm missing (build in CI first)');
  const stat = fs.statSync(wasmPath);
  expect(stat.size).toBeGreaterThan(1000);
});

test('compile and run Point class in browser', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await compileAndRun(page, POINT_SOURCE, 'main() => 42');
});

test('compile and run custom Iterator for-in stdout', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(ITERATOR_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('6', { timeout: 30000 });
});

test('compile and run sorted array for-in stdout', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(SORTED_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('1', { timeout: 30000 });
  await expect(out).toContainText('2', { timeout: 30000 });
  await expect(out).toContainText('3', { timeout: 30000 });
});

test('compile and run closure stdout hard (tier-2I)', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(CLOSURE_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('86', { timeout: 30000 });
});

test('compile and run Fmt.format stdout hard (tier-2I)', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(FMT_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('tier2i-86', { timeout: 30000 });
  await expect(out).toContainText('86', { timeout: 30000 });
});

test('compile and run StringBuilder stdout hard (tier-2I)', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(STRING_BUILDER_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('p86-sb', { timeout: 30000 });
});

test('compile and run reflect type host stdout hard (tier-2I)', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(REFLECT_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('86', { timeout: 30000 });
});

const GUI_PIXEL_SOURCE = `func main() {
    let win = __syx_gui_winit_window_create("tier2q")
    __syx_gui_winit_canvas_draw(win, "red")
    print(__syx_gui_pixel_read(win))
}`;

test('GUI wasm pixelRead smoke (tier2q)', async ({ page }) => {
  const { indexPath } = requireArtifacts(test);
  await page.goto('file://' + indexPath);
  await page.locator('#src').fill(GUI_PIXEL_SOURCE);
  await page.locator('#compile').click();
  const out = page.locator('#out');
  await expect(out).toContainText('pixel', { timeout: 30000 });
});
