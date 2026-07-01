# Code-to-Image Rendering: Implementation Plan

## Root Cause Analysis

### Why It Currently Fails

#### Problem 1: `cx.update()` Returns a `Result`, Not a String
Inside `cx.spawn(async move |this, cx| { ... })`, the call:
```rust
let code_text = cx.update(|cx| code_input.read(cx).text().to_string());
```
Returns `Result<String, Closed>`, not `String`. If the window context is briefly
unavailable, this is `Err(...)`. The code then passes this to silicon with garbage input.

#### Problem 2: Font List Causes Empty FontCollection -> Panic
`FontCollection::new()` tries each font and silently skips failures.
If all fonts fail (fontconfig not working on executor thread), the collection is empty.
Then `get_font_height()` calls `.max().unwrap()` on an empty iterator -> **panic**.
This panic is caught by... nothing, since we removed `catch_unwind`.

#### Problem 3: `Hack` Font IS Embedded But We're Not Using It Correctly
Silicon has embedded `Hack-Regular.ttf` bytes. When you pass `.font(vec![("Hack", ...)])`,
it calls `SystemSource::new().select_family_by_name("Hack")` — a SYSTEM fontconfig lookup,
not the embedded font. The embedded font is only used when `ImageFont::new("Hack", ...)` is
called, which short-circuits to `Default::default()` (embedded bytes).

The key insight: **pass NO font list to the builder** -> it uses `FontCollection::default()`
which loads from embedded TTF bytes. Zero fontconfig dependency.

---

## Correct Implementation

### Strategy
1. Read `code_text` **in `on_click`** (synchronous, before any spawning)
2. Do the rendering in **`std::thread::spawn`** (blocking CPU work, not async)
3. Use **`cx.spawn`** only for polling the result channel back to the UI

### New Module: `src/render.rs`
```rust
use silicon::assets::HighlightingAssets;
use silicon::formatter::ImageFormatterBuilder;
use silicon::utils::ShadowAdder;
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

pub fn render_code_to_image(
    code: &str,
    lang: &str,
    theme_name: &str,
) -> Result<std::path::PathBuf, String> {
    let ha = HighlightingAssets::new();
    let ps = &ha.syntax_set;
    let ts = &ha.theme_set;

    let syntax = ps
        .find_syntax_by_name(lang)
        .or_else(|| ps.find_syntax_by_extension(lang))
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let theme = ts
        .themes
        .get(theme_name)
        .or_else(|| ts.themes.get("Dracula"))
        .or_else(|| ts.themes.values().next())
        .ok_or_else(|| "No themes available".to_string())?;

    let mut h = HighlightLines::new(syntax, theme);
    let highlight = LinesWithEndings::from(code)
        .map(|line| h.highlight_line(line, ps))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Highlight error: {e}"))?;

    // KEY: no .font(...) call -> uses FontCollection::default() -> embedded Hack bytes
    // This bypasses fontconfig/SystemSource entirely
    let mut formatter = ImageFormatterBuilder::<String>::new()
        .shadow_adder(ShadowAdder::default())
        .build()
        .map_err(|e| format!("Formatter build error: {e}"))?;

    let image = formatter.format(&highlight, theme);

    let out_path = std::env::temp_dir().join(format!(
        "wallmod_render_{}.png",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    ));

    image.save(&out_path).map_err(|e| format!("Save error: {e}"))?;
    Ok(out_path)
}
```

### Updated `on_click` in `sidebar.rs`
```rust
.on_click(cx.listener(|this, _, _, cx| {
    // STEP 1: Read code text SYNCHRONOUSLY in on_click, not inside spawn
    let code_text = this.code_render_input.read(cx).text().to_string();
    if code_text.trim().is_empty() {
        this.app.state = AppState::Notice("Code is empty.".to_string());
        cx.notify();
        return;
    }

    let lang = this.app.code_render_language.clone();
    let theme = this.app.code_render_theme.clone();
    this.app.state = AppState::Loading(0.0, "Rendering...".to_string());
    cx.notify();

    // STEP 2: Use mpsc channel + std::thread for blocking work
    let (tx, rx) = std::sync::mpsc::channel::<Result<std::path::PathBuf, String>>();
    std::thread::spawn(move || {
        let _ = tx.send(crate::render::render_code_to_image(&code_text, &lang, &theme));
    });

    // STEP 3: cx.spawn only for polling result back to UI
    let task = cx.spawn(async move |this, mut cx| {
        loop {
            cx.background_executor().timer(std::time::Duration::from_millis(50)).await;
            match rx.try_recv() {
                Ok(Ok(path)) => {
                    let _ = this.update(&mut cx, |view, cx| {
                        view.app.code_render_preview = Some(path);
                        view.app.state = crate::app::AppState::Idle;
                        cx.notify();
                    });
                    break;
                }
                Ok(Err(msg)) => {
                    let _ = this.update(&mut cx, |view, cx| {
                        view.app.state = crate::app::AppState::Notice(msg);
                        cx.notify();
                    });
                    break;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => continue,
                Err(_) => break,
            }
        }
    });
    task.detach();
}))
```

---

## Summary of Changes

| File | Action |
|------|--------|
| `src/render.rs` | **CREATE**: `render_code_to_image()` with embedded Hack font |
| `src/main.rs` | Add `mod render;` |
| `src/ui/sidebar.rs` | Fix `on_click`: read code in handler, use `std::thread`, poll via `cx.spawn` |

## Why This Will Work
- **No fontconfig**: No `.font(vec![...])` = embedded Hack TTF bytes only
- **No GPUI async context issues**: Code text read synchronously in `on_click`  
- **Proper threading**: `std::thread::spawn` for blocking CPU work
- **Clean error reporting**: All errors propagate as `Err(String)` to UI
