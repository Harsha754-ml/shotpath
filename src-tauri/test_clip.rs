use clipboard_win::{Clipboard, formats, Setter};

fn main() {
    let path = "C:\\Windows\\System32\\notepad.exe".to_string();
    let _clip = Clipboard::new_attempts(10).unwrap();
    
    // Set text format
    formats::Unicode.write_clipboard(&path).unwrap();
    
    // Set file drop format
    formats::FileList.write_clipboard(&vec![path]).unwrap();
    
    println!("Clipboard set!");
}
