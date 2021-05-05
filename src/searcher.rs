use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct SoundFile {
    name: String,
    path: String,
    full_path: String,
}

impl Serialize for SoundFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SoundFile", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("path", &self.path)?;
        s.serialize_field("full_path", &self.full_path)?;
        s.end()
    }
}

pub fn get_files(full_path: &Path, filter_ext: &str) -> io::Result<Vec<SoundFile>> {
    let files = search_files(full_path, filter_ext)?;
    let files = convert_sound_files(files);
    Ok(files)
}

// TODO: 拡張子複数対応
fn search_files(full_path: &Path, filter_ext: &str) -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];
    let walkdir = WalkDir::new(full_path).max_depth(1);

    for entry in walkdir {
        let entry = entry?;
        let path = entry.path();
        let path_buf = path.to_path_buf();
        if entry.file_type().is_file() {
            if filter_ext != "" {
                if let Some(e) = path_buf.extension() {
                    if e == filter_ext {
                        files.push(path_buf);
                    }
                }
            } else {
                files.push(path_buf);
            }
        }
    }

    Ok(files)
}

fn convert_sound_files(files: Vec<PathBuf>) -> Vec<SoundFile> {
    let mut sound_files = vec![];
    for path in files {
        if let Some(x) = path_buf_to_sound_file(&path) {
            sound_files.push(x);
        }
    }

    sound_files
}

fn path_buf_to_sound_file(path: &PathBuf) -> Option<SoundFile> {
    let name = path.file_name()?.to_str()?.to_string();
    let full_path = path.to_str()?.to_string();
    let path = format!("/_static/{}", name);

    Some(SoundFile {
        name,
        full_path,
        path,
    })
}
