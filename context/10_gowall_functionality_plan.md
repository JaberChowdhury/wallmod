# Gowall Functionality Implementation Plan (Go Sidecar Architecture)

Since we are keeping the Rust application focused purely on the **GPUI frontend** and utilizing the cloned `gowall` Go source code for the actual **functionality**, this plan details how we will bridge the two languages.

## 1. Architectural Strategy: The Sidecar Pattern
Instead of rewriting all of Gowall's features in Rust, we will compile the Go source code into a standalone binary during our build process, and our Rust app will invoke it as a subprocess.

### A. Build Process Integration (`build.rs`)
We will create a Cargo `build.rs` script that automatically runs `go build -o ../target/release/gowall main.go` inside the `gowall_src` directory whenever the Rust project is compiled. This ensures the Go binary is always in sync with our app.

### B. Execution Wrapper (`src/backend/gowall_cli.rs`)
We will write an asynchronous Rust wrapper using `tokio::process::Command` that constructs the CLI arguments required by Gowall, executes the binary, and captures the output.

```rust
pub async fn run_gowall_command(args: Vec<&str>) -> Result<PathBuf, String> {
    let output = tokio::process::Command::new("./gowall")
        .args(&args)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        // Parse stdout for the output file path
        Ok(PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}
```

## 2. Implementation by Feature

### Theme Conversion (`gowall color`)
- **Rust UI:** User selects a theme from a dropdown.
- **Execution:** `run_gowall_command(vec!["color", "-t", "catppuccin-mocha", input_path])`
- **Result:** Read the modified image from `gowall`'s output path and update the GPUI `Image` view.

### Basic Effects & Invert (`gowall invert`, `gowall effects`)
- **Rust UI:** User clicks "Invert" or "Grayscale".
- **Execution:** 
  - Invert: `run_gowall_command(vec!["invert", input_path])`
  - Grayscale: `run_gowall_command(vec!["effects", "--grayscale", input_path])`

### Advanced AI (Background Removal & Upscale)
- **Rust UI:** User clicks "Remove Background". A GPUI loading spinner appears.
- **Execution:** `run_gowall_command(vec!["bg", "remove", input_path])`
- Note: Go's ONNX bindings handle the heavy lifting. The Rust thread is completely unblocked due to `tokio`.

### OCR Text Extraction (`gowall ocr`)
- **Rust UI:** User selects a bounding box or clicks "Extract Text".
- **Execution:** `run_gowall_command(vec!["ocr", input_path])`
- **Result:** Parse `stdout` from the Go binary directly into a Rust `String` and display it in a GPUI text area.

## 3. Benefits of this Approach
1. **Zero Reinvention:** We leverage 100% of Gowall's existing battle-tested logic (ONNX models, format conversions, etc.).
2. **Safe & Isolated:** If Gowall panics or crashes on a malformed image, the Rust GPUI application remains alive and can simply catch the non-zero exit code and display an error toast.
3. **Async Friendly:** Rust's `tokio` effortlessly spawns OS processes without blocking the UI rendering thread.
