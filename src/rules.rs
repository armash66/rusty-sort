use std::collections::HashSet;
use std::fmt;
use std::io;
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

pub struct Rules {
    images: HashSet<String>,
    documents: HashSet<String>,
    videos: HashSet<String>,
    audio: HashSet<String>,
    archives: HashSet<String>,
}

impl Rules {
    pub fn default() -> Self {
        Self {
            images: IMAGES.iter().map(|s| s.to_string()).collect(),
            documents: DOCUMENTS.iter().map(|s| s.to_string()).collect(),
            videos: VIDEOS.iter().map(|s| s.to_string()).collect(),
            audio: AUDIO.iter().map(|s| s.to_string()).collect(),
            archives: ARCHIVES.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn from_config(path: &Path) -> io::Result<Self> {
        let mut rules = Self::default();
        let content = std::fs::read_to_string(path)?;
        for (idx, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (name, rest) = line.split_once('=').ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid config line {}: {}", idx + 1, raw_line),
                )
            })?;
            let category = parse_category(name.trim()).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unknown category on line {}: {}", idx + 1, raw_line),
                )
            })?;

            for ext in rest.split(',') {
                let ext = ext.trim().trim_start_matches('.').to_ascii_lowercase();
                if ext.is_empty() {
                    continue;
                }
                rules.insert(category, ext);
            }
        }

        Ok(rules)
    }

    pub fn classify(&self, path: &Path) -> Category {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase());

        let Some(ext) = ext else {
            return Category::Others;
        };

        if self.images.contains(&ext) {
            Category::Images
        } else if self.documents.contains(&ext) {
            Category::Documents
        } else if self.videos.contains(&ext) {
            Category::Videos
        } else if self.audio.contains(&ext) {
            Category::Audio
        } else if self.archives.contains(&ext) {
            Category::Archives
        } else {
            Category::Others
        }
    }

    fn insert(&mut self, category: Category, ext: String) {
        match category {
            Category::Images => {
                self.images.insert(ext);
            }
            Category::Documents => {
                self.documents.insert(ext);
            }
            Category::Videos => {
                self.videos.insert(ext);
            }
            Category::Audio => {
                self.audio.insert(ext);
            }
            Category::Archives => {
                self.archives.insert(ext);
            }
            Category::Others => {}
        }
    }
}

fn parse_category(name: &str) -> Option<Category> {
    match name.trim().to_ascii_lowercase().as_str() {
        "images" => Some(Category::Images),
        "documents" => Some(Category::Documents),
        "videos" => Some(Category::Videos),
        "audio" => Some(Category::Audio),
        "archives" => Some(Category::Archives),
        "others" => Some(Category::Others),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{Category, Rules};
    use std::path::Path;

    #[test]
    fn classify_images() {
        let rules = Rules::default();
        assert_eq!(rules.classify(Path::new("photo.JPG")), Category::Images);
        assert_eq!(rules.classify(Path::new("icon.png")), Category::Images);
    }

    #[test]
    fn classify_documents() {
        let rules = Rules::default();
        assert_eq!(rules.classify(Path::new("report.pdf")), Category::Documents);
        assert_eq!(rules.classify(Path::new("notes.md")), Category::Documents);
    }

    #[test]
    fn classify_videos() {
        let rules = Rules::default();
        assert_eq!(rules.classify(Path::new("movie.mkv")), Category::Videos);
        assert_eq!(rules.classify(Path::new("clip.MP4")), Category::Videos);
    }

    #[test]
    fn classify_audio() {
        let rules = Rules::default();
        assert_eq!(rules.classify(Path::new("song.mp3")), Category::Audio);
        assert_eq!(rules.classify(Path::new("voice.WAV")), Category::Audio);
    }

    #[test]
    fn classify_archives() {
        let rules = Rules::default();
        assert_eq!(rules.classify(Path::new("backup.zip")), Category::Archives);
        assert_eq!(rules.classify(Path::new("bundle.tar")), Category::Archives);
    }

    #[test]
    fn classify_others() {
        let rules = Rules::default();
        assert_eq!(rules.classify(Path::new("file")), Category::Others);
        assert_eq!(rules.classify(Path::new("weird.ext")), Category::Others);
    }
}
