use std::env;
use std::path::PathBuf;

/// Asynchronously invokes the local `gowall` Go sidecar binary using spawn_blocking to avoid tokio runtime panics.
/// `args` are the command-line arguments to pass.
/// Returns the PathBuf parsed from stdout on success, or the stderr String on failure.
pub async fn run_gowall_command(args: Vec<String>) -> Result<PathBuf, String> {
    crate::backend::runtime::spawn_blocking(move || {
        // Locate the gowall binary which is placed in the same directory as the wallmod executable
        // by our build.rs script.
        let mut exe_path = env::current_exe().map_err(|e| e.to_string())?;
        exe_path.pop(); // Remove "wallmod" executable name from path to get the directory
        let gowall_bin = exe_path.join("gowall");

        if !gowall_bin.exists() {
            return Err(format!("Gowall sidecar binary not found at {:?}", gowall_bin));
        }

        let output = std::process::Command::new(&gowall_bin)
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to execute process: {}", e))?;

        if output.status.success() {
            // Many commands output the saved file path or extracted text on stdout
            let stdout = String::from_utf8_lossy(&output.stdout);
            let cleaned = stdout.trim();
            Ok(PathBuf::from(cleaned))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(stderr.into_owned())
        }
    })
    .await
    .unwrap_or_else(|e| Err(format!("Join error: {}", e)))
}
