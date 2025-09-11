use std::path::Path;
use std::fs::File;
 use std::io::Write;
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
  let mut file = OpenOptions::new().append(true).read(true)
            .write(true)
            .create(true)
            .open(path);
   let _ =file.write(content.as_bytes());

}