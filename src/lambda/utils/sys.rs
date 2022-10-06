pub fn is_windows() -> bool {
    cfg!(windows)
}

pub fn is_linux() -> bool {
    cfg!(linux)
}

pub fn is_macos() -> bool {
    cfg!(macos)
}