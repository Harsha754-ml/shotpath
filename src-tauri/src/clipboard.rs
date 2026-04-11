use std::path::Path;
use clipboard_win::{Clipboard, formats, Setter};
use image::GenericImageView;

pub fn copy_to_clipboard(path: &Path) {
    if let Ok(_clip) = Clipboard::new_attempts(10) {
        let path_str = path.to_string_lossy().to_string();
        
        // 1. Set File Path as Unicode Text (for text editors, CLI, etc.)
        let _ = formats::Unicode.write_clipboard(&path_str);
        
        // 2. Set as File List (for File Explorer or attaching files)
        let _ = formats::FileList.write_clipboard(&vec![path_str]);

        // 3. Set as Image Data (for Discord, Slack, etc.)
        // We open the image and convert it to a BMP format that the Windows clipboard recognizes
        if let Ok(img) = image::open(path) {
            let mut bmp_data: Vec<u8> = Vec::new();
            // Using the image crate to encode as BMP directly into a buffer
            if let Ok(_) = img.write_to(&mut std::io::Cursor::new(&mut bmp_data), image::ImageFormat::Bmp) {
                // Windows clipboard expects the DIB (Device Independent Bitmap) 
                // which is the BMP data MINUS the 14-byte File Header.
                if bmp_data.len() > 14 {
                    let dib_data = &bmp_data[14..];
                    let _ = formats::Dib.write_clipboard(&dib_data);
                }
            }
        }
    }
}
