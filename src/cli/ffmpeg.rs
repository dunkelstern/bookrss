use std::process::{Command, ExitStatus, Stdio};
use std::str::FromStr;
use std::path::Path;

use serde::{de, Deserializer, Deserialize};
use serde_json;

#[derive(Deserialize, Debug)]
pub struct MediaTags {
    pub major_brand: Option<String>, // you can recognize aax files with this one
    
    // one of these is the author
    pub artist: Option<String>, // first choice as it is the standard
    pub author: Option<String>, // this one is mostly audible specific

    // one of these is the book name
    pub album: Option<String>, // first choice
    pub parent_title: Option<String>, // second choice, if available at all
    pub title: String, // third choice, should be available always

    // one of these is a short description
    pub comment: Option<String>, // id3 tag standard
    pub description: Option<String>, // used in audible formats

    // one of these is kind of a publication date
    pub date: Option<String>, // used in aax, probably only year
    pub pub_date_start: Option<String>, // used in older audible format

    // narrator/speaker
    pub narrator: Option<String>, // probably only used in older audible format
}

#[derive(Deserialize, Debug)]
pub struct MediaFormat {
    pub format_name: String,
    #[serde(deserialize_with = "f64_from_str")]
    pub duration: f64,
    #[serde(deserialize_with = "usize_from_str")]
    pub size: usize,
    #[serde(deserialize_with = "u32_from_str")]
    pub bit_rate: u32,
    pub tags: MediaTags,
}

#[derive(Deserialize, Debug)]
pub struct MediaFormatContainer {
    pub format: MediaFormat,
}

/// Identify media file (read metadata), uses `ffprobe`
pub fn identify(filename: &Path) -> Result<MediaFormat, serde_json::Error> {
    let output = Command::new("ffprobe")
        .arg("-print_format")
        .arg("json")
        .arg("-show_format")
        .arg(filename)
        .output()
        .expect("running ffprobe");
    
    
    let result = serde_json::from_str::<MediaFormatContainer>(
        &String::from_utf8_lossy(&output.stdout).into_owned()
    );

    match result {
        Ok(format) => Ok(format.format),
        Err(err) => Err(err),
    }
}

/// Decode `aax` audible audio book
pub fn de_aax(filename: &Path, output: &Path, magic_bytes: &str) -> ExitStatus {
    let mut cmd = Command::new("ffmpeg");
        
    cmd 
        .arg("-v")
        .arg("1")
        .arg("-activation_bytes")
        .arg(magic_bytes)
        .arg("-i")
        .arg(filename)
        .arg("-vn")
        .arg("-c:a")
        .arg("copy")
        .arg(output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .expect("running ffmpeg")
}

/// Convert string to f64 while deserializing
fn f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    f64::from_str(&s).map_err(de::Error::custom)
}

/// Convert string to usize while deserializing
fn usize_from_str<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    usize::from_str(&s).map_err(de::Error::custom)
}

/// Convert string to u32 while deserializing
fn u32_from_str<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    u32::from_str(&s).map_err(de::Error::custom)
}
