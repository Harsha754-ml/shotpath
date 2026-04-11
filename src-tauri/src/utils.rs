use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub fn is_image(path: &Path) -> bool {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp")
}

pub fn wait_for_file_ready(path: &Path) {
    // Basic retry loop to wait for OS to finish writing the file
    let mut retries = 0;
    while retries < 5 {
        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.len() > 0 {
                return;
            }
        }
        sleep(Duration::from_millis(100));
        retries += 1;
    }
}
