use oxipng::{optimize_from_memory, Options};

/// Compresses a PNG image losslessly in memory using oxipng.
/// Returns (Optimized Bytes, Original Size, Optimized Size).
pub fn compress_png(raw_bytes: &[u8]) -> Result<(Vec<u8>, usize, usize), String> {
    let original_size = raw_bytes.len();
    
    let options = Options::max_compression();
    
    match optimize_from_memory(raw_bytes, &options) {
        Ok(optimized_bytes) => {
            let opt_size = optimized_bytes.len();
            Ok((optimized_bytes, original_size, opt_size))
        }
        Err(e) => Err(format!("Compression failed: {}", e)),
    }
}
