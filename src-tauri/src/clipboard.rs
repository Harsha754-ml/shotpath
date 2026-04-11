use arboard::{Clipboard, ImageData};
use std::path::Path;
use image::GenericImageView;
use std::borrow::Cow;

pub fn copy_path_to_clipboard(path: &Path) {
    if let Ok(mut clipboard) = Clipboard::new() {
        let path_str = path.to_string_lossy().to_string();
        let _ = clipboard.set_text(path_str);
    }
}

pub fn copy_image_to_clipboard(path: &Path) {
    if let Ok(mut clipboard) = Clipboard::new() {
        if let Ok(img) = image::open(path) {
            let (width, height) = img.dimensions();
            let bytes = img.to_rgba8().into_raw();
            
            let img_data = ImageData {
                width: width as usize,
                height: height as usize,
                bytes: Cow::from(bytes),
            };
            
            let _ = clipboard.set_image(img_data);
        }
    }
}
