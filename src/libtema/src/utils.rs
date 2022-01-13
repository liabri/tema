use std::fs::{ read_dir, copy, create_dir_all };
use std::path::Path;
use std::io::Result;

pub(crate) fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    let dst = dst.as_ref();
    create_dir_all(dst)?;
    for entry in read_dir(src)? {
        let entry = entry?;
        let filename = entry.file_name();
        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), dst.join(filename))?;
        } else {
            copy(entry.path(), dst.join(filename))?;
        }
    }
    Ok(())
}