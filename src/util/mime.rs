use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Mime {
    pub mime_type: String,
    pub ext: String
}
impl Mime {
    pub fn new(mime_type: &str, ext: &str) -> Mime { Mime { mime_type: mime_type.to_string(), ext: ext.to_string() }}
}
impl fmt::Display for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ \"mime\": {}, \"ext\": \"{}\" }}", self.mime_type, self.ext)
    }
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Complete_list_of_MIME_types
#[allow(dead_code)]
lazy_static! {
    pub static ref AAC_AUDIO: Mime = Mime::new("audio/aac", ".aac");
    pub static ref ABIWORD_DOCUMENT: Mime = Mime::new("application/x-abiword", ".abw");
    pub static ref ARCHIVE_DOCUMENT: Mime = Mime::new("application/x-freearc", ".arc");
    pub static ref AVI_AUDIO_VIDEO_INTERLEAVE: Mime = Mime::new("video/x-msvideo", ".avi  ");
    pub static ref AMAZON_KINDLE_EBOOK: Mime = Mime::new("application/vnd.amazon.ebook", ".azw  ");
    pub static ref BINARY: Mime = Mime::new("application/octet-stream", ".bin  ");
    pub static ref WINDOWS_BITMAP_GRAPHICS: Mime = Mime::new("image/bmp", ".bmp  ");
    pub static ref BZIP_ARCHIVE: Mime = Mime::new("application/x-bzip", ".bz  ");
    pub static ref BZIP2_ARCHIVE: Mime = Mime::new("application/x-bzip2", ".bz2  ");
    pub static ref C_SHELL_SCRIPT: Mime = Mime::new("application/x-csh", ".csh  ");
    pub static ref CSS: Mime = Mime::new("text/css", ".css");
    pub static ref CSV: Mime = Mime::new("text/csv", ".csv");
    pub static ref MICROSOFT_WORD: Mime = Mime::new("application/msword", ".doc");
    pub static ref MICROSOFT_WORD_OPEN_XML: Mime = Mime::new("application/vnd.openxmlformats-officedocument.wordprocessingml.document", ".docx");
    pub static ref MS_EMBEDDED_OPENTYPE_FONTS: Mime = Mime::new("application/vnd.ms-fontobject", ".eot");
    pub static ref EPUB: Mime = Mime::new("application/epub+zip", ".epub");
    pub static ref GZIP: Mime = Mime::new("application/gzip", ".gz");
    pub static ref GIF: Mime = Mime::new("image/gif", ".gif");
    pub static ref HTML: Mime = Mime::new("text/html", ".html");
    pub static ref HTM: Mime = Mime::new("text/html", ".htm");
    pub static ref ICON: Mime = Mime::new("image/vnd.microsoft.icon", ".ico");
    pub static ref ICALENDAR: Mime = Mime::new("text/calendar", ".ics");
    pub static ref JAVA_ARCHIVE: Mime = Mime::new("application/java-archive", ".jar");
    pub static ref JPEG: Mime = Mime::new("image/jpeg", ".jpeg");
    pub static ref JPG: Mime = Mime::new("image/jpeg", ".jpg");
    pub static ref JAVASCRIPT: Mime = Mime::new("text/javascript", ".js");
    pub static ref JSON: Mime = Mime::new("application/json", ".json");
    pub static ref JSON_LD: Mime = Mime::new("application/ld+json", ".jsonld");
    pub static ref MID: Mime = Mime::new("audio/midi", ".mid");
    pub static ref X_MID: Mime = Mime::new("audio/x-midi", ".mid");
    pub static ref MIDI: Mime = Mime::new("audio/midi", ".midi");
    pub static ref X_MIDI: Mime = Mime::new("audio/x-midi", ".midi");
    pub static ref JAVASCRIPT_MODULE: Mime = Mime::new("application/javascript", ".mjs");
    pub static ref MP3: Mime = Mime::new("audio/mpeg", ".mp3");
    pub static ref MPEG: Mime = Mime::new("video/mpeg", ".mpeg");
    pub static ref APPLE_INSTALLER_PACKAGE: Mime = Mime::new("application/vnd.apple.installer+xml", ".mpkg");
    pub static ref OPENDOCUMENT_PRESENTATION: Mime = Mime::new("application/vnd.oasis.opendocument.presentation", ".odp");
    pub static ref OPENDOCUMENT_SPREADSHEET: Mime = Mime::new("application/vnd.oasis.opendocument.spreadsheet", ".ods");
    pub static ref OPENDOCUMENT_TEXT: Mime = Mime::new("application/vnd.oasis.opendocument.text", ".odt");
    pub static ref OGG_AUDIO: Mime = Mime::new("audio/ogg", ".oga");
    pub static ref OGG_VIDEO: Mime = Mime::new("video/ogg", ".ogv");
    pub static ref OGG: Mime = Mime::new("application/ogg", ".ogx");
    pub static ref OPUS_AUDIO: Mime = Mime::new("audio/opus", ".opus");
    pub static ref OPENTYPE_FONT: Mime = Mime::new("font/otf", ".otf");
    pub static ref PNG: Mime = Mime::new("image/png", ".png");
    pub static ref PDF: Mime = Mime::new("application/pdf", ".pdf");
    pub static ref PERSONAL_HOME_PAGE: Mime = Mime::new("application/php", ".php");
    pub static ref MICROSOFT_POWERPOINT: Mime = Mime::new("application/vnd.ms-powerpoint", ".ppt");
    pub static ref MICROSOFT_POWERPOINT_OPEN_XML: Mime = Mime::new("application/vnd.openxmlformats-officedocument.presentationml.presentation", ".pptx");
    pub static ref RAR_ARCHIVE: Mime = Mime::new("application/x-rar-compressed", ".rar");
    pub static ref RTF: Mime = Mime::new("application/rtf", ".rtf");
    pub static ref BOURNE_SHELL_SCRIPT: Mime = Mime::new("application/x-sh", ".sh");
    pub static ref SVG: Mime = Mime::new("image/svg+xml", ".svg");
    pub static ref ADOBE_FLASH: Mime = Mime::new("application/x-shockwave-flash", ".swf");
    pub static ref TAPE_ARCHIVE: Mime = Mime::new("application/x-tar", ".tar");
    pub static ref TIF: Mime = Mime::new("image/tiff", ".tif");
    pub static ref TIFF: Mime = Mime::new("image/tiff", ".tiff");
    pub static ref MPEG_TRANSPORT_STREAM: Mime = Mime::new("video/mp2t", ".ts");
    pub static ref TRUETYPE_FONT: Mime = Mime::new("font/ttf", ".ttf");
    pub static ref TEXT: Mime = Mime::new("text/plain", ".txt");
    pub static ref MICROSOFT_VISIO: Mime = Mime::new("application/vnd.visio", ".vsd");
    pub static ref WAVEFORM_AUDIO_FORMAT: Mime = Mime::new("audio/wav", ".wav");
    pub static ref WEBM_AUDIO: Mime = Mime::new("audio/webm", ".weba");
    pub static ref WEBM_VIDEO: Mime = Mime::new("video/webm", ".webm");
    pub static ref WEBP_IMAGE: Mime = Mime::new("image/webp", ".webp");
    pub static ref WEB_OPEN_FONT_FORMAT: Mime = Mime::new("font/woff", ".woff");
    pub static ref WEB_OPEN_FONT_FORMAT_2: Mime = Mime::new("font/woff2", ".woff2");
    pub static ref XHTML: Mime = Mime::new("application/xhtml+xml", ".xhtml");
    pub static ref MICROSOFT_EXCEL: Mime = Mime::new("application/vnd.ms-excel", ".xls");
    pub static ref MICROSOFT_EXCEL_OPEN_XML: Mime = Mime::new("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", ".xlsx");
    pub static ref XML_MACHINE: Mime = Mime::new("application/xml", ".xml");
    pub static ref XML: Mime = Mime::new("text/xml", ".xml");
    pub static ref XUL: Mime = Mime::new("application/vnd.mozilla.xul+xml", ".xuls");
    pub static ref ZIP_ARCHIVE: Mime = Mime::new("application/zip", ".zip");
    pub static ref GPP3_AUDIO_VIDEO_CONTAINER: Mime = Mime::new("video/3gpp", ".3gp");
    pub static ref GPP3_AUDIO_CONTAINER: Mime = Mime::new("audio/3gpp", ".3gp");
    pub static ref GPP3_2_AUDIO_VIDEO_CONTAINER: Mime = Mime::new("video/3gpp2", ".3g2");
    pub static ref GPP3_2_AUDIO_CONTAINER: Mime = Mime::new("audio/3gpp2", ".3g2");
    pub static ref ZIP7_ARCHIVE: Mime = Mime::new("application/x-7z-compressed", ".7z");
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use crate::util::mime::*;

    #[test]
    fn test_mime() {
        let mime = Mime::new("audio/aac", ".aac");
        assert_eq!(mime.mime_type, "audio/aac");
        assert_eq!(mime.ext, ".aac");
    }
}