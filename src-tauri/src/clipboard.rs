use std::path::Path;
use clipboard_win::{Clipboard, formats, Setter};
use clipboard_win::formats::CF_DIB;

pub fn copy_to_clipboard(path: &Path) {
    let path_str = path.to_string_lossy().to_string();
    
    // Open the clipboard once.
    if let Ok(_clip) = Clipboard::new_attempts(10) {
        // Set all formats in a single block.
        // If write_clipboard fails, it returns an error.
        
        if let Err(e) = formats::Unicode.write_clipboard(&path_str) {
            println!("Failed to set Unicode: {:?}", e);
        } else {
            println!("Unicode (Path) set successfully.");
        }
        
        if let Err(e) = formats::FileList.write_clipboard(&vec![path_str.clone()]) {
            println!("Failed to set FileList: {:?}", e);
        } else {
            println!("FileList set successfully.");
        }

        if let Ok(img) = image::open(path) {
            let mut bmp_data: Vec<u8> = Vec::new();
            if let Ok(_) = img.write_to(&mut std::io::Cursor::new(&mut bmp_data), image::ImageFormat::Bmp) {
                if bmp_data.len() > 14 {
                    let dib_data = &bmp_data[14..];
                    if let Err(e) = formats::RawData(CF_DIB).write_clipboard(&dib_data) {
                        println!("Failed to set DIB Image: {:?}", e);
                    } else {
                        println!("DIB Image set successfully.");
                    }
                }
            }
        }
    } else {
        println!("Failed to open clipboard guard.");
    }
}
