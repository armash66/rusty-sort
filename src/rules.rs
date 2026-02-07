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

const IMAGES: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp", "svg", "heic", "heif", "ico",
    "raw", "nef", "cr2", "arw", "dng",
];
const DOCUMENTS: &[&str] = &[
    "pdf", "doc", "docx", "docm", "xls", "xlsx", "xlsm", "ppt", "pptx", "pptm", "txt", "md",
    "rtf", "csv", "tsv", "json", "yaml", "yml", "xml", "log", "ini", "cfg",
];
const VIDEOS: &[&str] = &[
    "mp4", "mkv", "mov", "avi", "wmv", "flv", "webm", "m4v", "3gp", "3g2", "mpg", "mpeg",
    "ts", "mts",
];
const AUDIO: &[&str] = &[
    "mp3", "wav", "flac", "aac", "ogg", "m4a", "wma", "aiff", "aif", "amr", "opus",
];
const ARCHIVES: &[&str] = &[
    "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "tgz", "tbz2", "lz", "lzma", "zst",
    "iso",
];

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
