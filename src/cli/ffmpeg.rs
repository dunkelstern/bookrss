use std::process::{Command, ExitStatus, Stdio};
use std::str::FromStr;
use std::path::Path;

use serde::{de, Deserializer, Deserialize};
use serde_json;

#[derive(Deserialize, Debug)]
struct MediaTags {
    major_brand: Option<String>, // you can recognize aax files with this one
    
    // one of these is the author
    artist: Option<String>, // first choice as it is the standard
    author: Option<String>, // this one is mostly audible specific

    // one of these is the book name
    album: Option<String>, // first choice
    parent_title: Option<String>, // second choice, if available at all
    title: String, // third choice, should be available always

    // one of these is a short description
    comment: Option<String>, // id3 tag standard
    description: Option<String>, // used in audible formats

    // one of these is kind of a publication date
    date: Option<String>, // used in aax, probably only year
    pub_date_start: Option<String>, // used in older audible format

    // narrator/speaker
    narrator: Option<String>, // probably only used in older audible format
}

#[derive(Deserialize, Debug)]
struct MediaFormat {
    format_name: String,
    #[serde(deserialize_with = "f64_from_str")]
    duration: f64,
    #[serde(deserialize_with = "usize_from_str")]
    size: usize,
    #[serde(deserialize_with = "u32_from_str")]
    bit_rate: u32,
    tags: MediaTags,
}

#[derive(Deserialize, Debug)]
struct MediaFormatContainer {
    format: MediaFormat,
}

#[derive(Debug)]
pub struct MediaInfo {
    pub format: String,
    pub author: String,
    pub title: String,
    pub description: String,
    pub date: String,
    pub narrator: String,
    pub duration: f64,
    pub size: usize,
    pub bit_rate: u32,
}

/// Identify media file (read metadata), uses `ffprobe`
pub fn identify(filename: &Path) -> Result<MediaInfo, serde_json::Error> {
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

    println!("{:?}", result);

    match result {
        Ok(format) => Ok(parse_format(format.format)),
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

/// convert to podcast like format
pub fn convert(filename: &Path, output: &Path, encoder: &str) -> ExitStatus {
    let mut cmd = Command::new("ffmpeg");
        
    cmd 
        .arg("-v")
        .arg("1")
        .arg("-i")
        .arg(filename)
        .arg("-vn")
        .arg("-c:a")
        .arg(encoder)
        .arg(output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .expect("running ffmpeg")
}

fn parse_format(format: MediaFormat) -> MediaInfo {
    let format_result = 
        if format.format_name.contains("mp4") {
            format.tags.major_brand.unwrap_or(String::from("mp4")).trim().to_string()
        } else {
            format.format_name
        };
    

    MediaInfo {
        format: format_result,
        author: 
            format.tags.artist.unwrap_or(
            format.tags.author.unwrap_or(
            String::from("unknown"))),

        title: 
            format.tags.album.unwrap_or(
            format.tags.parent_title.unwrap_or(
            format.tags.title)),

        description: 
            format.tags.comment.unwrap_or(
            format.tags.description.unwrap_or(
            String::from(""))),

        date: 
            format.tags.date.unwrap_or(
            format.tags.pub_date_start.unwrap_or(
            String::from(""))),

        narrator: 
            format.tags.narrator.unwrap_or(
            String::from("unknown")),

        duration: format.duration,
        size: format.size,
        bit_rate: format.bit_rate,
    }
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
