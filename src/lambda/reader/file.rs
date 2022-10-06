use std::{env::current_dir, path::Path, fs};

use crate::lambda::utils::sys::is_windows;

#[inline]
pub fn read_file(file: File) -> String {
    fs::read_to_string(file.full_path).unwrap()
}

#[inline]
pub fn configure_file(file: String, home: Option<String>) -> File {
    if is_full_path(file.clone()) {
        create_file(
            file.clone(),
            None)
    } else {
        create_file(
            file.clone(),
            home)
    }
}

#[inline]
fn create_file(path: String, dir: Option<String>) -> File {
    let full_path = smallest(full(path.clone(), dir.clone())).replace("//", "/");
    let dir = get_dir(full_path.clone());

    File {
        file_type: if path.ends_with(".ld") {
            FileType::SourceCode
        } else if path.ends_with(".ldb") {
            FileType::ByteCode
        } else {
            FileType::UnkownCode
        },
        full_path,
        raw_path: path,
        current_dir: dir
    }
}

#[inline]
fn is_full_path(file: String) -> bool {
    if is_windows() {
        !(file.starts_with("./") || file.starts_with("../") || file.starts_with(".\\") || file.starts_with("..\\"))
    } else {
        !(file.starts_with("./") || file.starts_with("../"))
    }
}

#[inline]
fn full(file: String, home: Option<String>) -> String {
    let mut dir = current_dir().unwrap();
    if let Some(i) = home {
        dir.push(Path::new(i.as_str()).to_path_buf());
    }

    dir.push(Path::new(file.as_str()).to_path_buf());
    dir.into_os_string().into_string().unwrap()
}

#[inline]
fn get_dir(path: String) -> String {
    smallest(Path::new(path.as_str()).parent().unwrap().as_os_str().to_os_string().into_string().unwrap())
}

fn smallest(mut path: String) -> String {
    if is_windows() { path = path.replace("\\", "/"); }
    let mut tokens = path.split("/").collect::<Vec<&str>>();
    let iter = tokens.iter_mut();
    let mut prev = false;
    let replace_str = "";
    for it in iter {
        match it {
            &mut "." => {
                *it = replace_str; 
            },
            &mut ".." => {
                *it = replace_str;
                prev = true;
            },
            _ => {
                if prev {
                    *it = replace_str;
                }
            }
        }
    }

    tokens.join("/")
}

#[derive(Debug, Clone)]
pub struct File {
    pub raw_path: String,
    pub current_dir: String,
    pub full_path: String,
    pub file_type: FileType
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    ByteCode,
    SourceCode,
    UnkownCode
}