use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(path).unwrap();
    let _ = file.write(content.as_bytes());
}
