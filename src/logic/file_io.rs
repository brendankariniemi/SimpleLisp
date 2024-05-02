use std::path::{Path, PathBuf};
use std::{fs, io};
use std::fs::File;
use std::io::Write;

pub fn read_file(path: &Path) -> Result<String, crate::error::AppError> {
    fs::read_to_string(path).map_err(|e| crate::error::AppError::Io(e))
}

pub fn write_file(path: &Path, content: &str) -> Result<(), crate::error::AppError> {
    let mut file = File::create(path).map_err(crate::error::AppError::Io)?;

    file.write_all(content.as_bytes())
        .map_err(crate::error::AppError::Io)?;

    Ok(())
}

pub fn create_directory<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path_buf = path.as_ref().to_path_buf();
    fs::create_dir_all(&path_buf)?;
    Ok(path_buf)
}

pub fn create_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut file = File::create(path)?;
    Ok(())
}