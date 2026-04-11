use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub fn is_image(path: &Path) -> bool {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp")
}

pub fn wait_for_file_ready(path: &Path) {
    let mut retries = 0;
    while retries < 10 {
        if let Ok(file) = std::fs::OpenOptions::new().read(true).write(true).open(path) {
            if let Ok(metadata) = file.metadata() {
                if metadata.len() > 0 {
                    return;
                }
            }
        }
        sleep(Duration::from_millis(200));
        retries += 1;
    }
}
