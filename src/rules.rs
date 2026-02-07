use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Images,
    Documents,
    Videos,
    Audio,
    Archives,
    Others,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Category::Images => "Images",
            Category::Documents => "Documents",
            Category::Videos => "Videos",
            Category::Audio => "Audio",
            Category::Archives => "Archives",
            Category::Others => "Others",
        };
        write!(f, "{}", name)
    }
}

const IMAGES: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "svg", "heic"];
const DOCUMENTS: &[&str] = &[
    "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "md", "rtf", "csv", "json",
    "yaml", "yml",
];
const VIDEOS: &[&str] = &["mp4", "mkv", "mov", "avi", "wmv", "flv", "webm", "m4v"];
const AUDIO: &[&str] = &["mp3", "wav", "flac", "aac", "ogg", "m4a", "wma"];
const ARCHIVES: &[&str] = &["zip", "rar", "7z", "tar", "gz", "bz2", "xz"];

pub fn classify(path: &Path) -> Category {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase());

    let Some(ext) = ext else {
        return Category::Others;
    };

    if IMAGES.contains(&ext.as_str()) {
        Category::Images
    } else if DOCUMENTS.contains(&ext.as_str()) {
        Category::Documents
    } else if VIDEOS.contains(&ext.as_str()) {
        Category::Videos
    } else if AUDIO.contains(&ext.as_str()) {
        Category::Audio
    } else if ARCHIVES.contains(&ext.as_str()) {
        Category::Archives
    } else {
        Category::Others
    }
}
