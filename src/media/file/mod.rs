use std::{
    fs::File,
    io::{ BufReader, Seek },
    path::PathBuf
};
use image::ImageReader;
use nom_exif::{
    MediaSource as MediaMetaSource,
    MediaParser as MediaMetaParser,
    ExifIter as ExifMetadata,
    ExifTag as ExifMetaTag,
    TrackInfo as TrackMetadata,
    TrackInfoTag as TrackMetaTag,
    EntryValue as MediaMetaValue
};
use ffmpeg_next::format::input as load_media;
use chrono::Datelike;
use serde_json::{ Value as JsonValue, from_str as json_from_str };


mod video;


#[derive(Debug)]
pub struct MediaInfo {
    pub source      : String,
    pub date        : Option<(u16, u8, u8,)>,
    pub thumbnail   : String,
    pub resolution  : (u32, u32,),
    pub label       : String,
    pub edited      : bool,
    pub kind        : MediaKind,
    pub is_cover    : bool
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MediaKind{
    Image,
    Video
}


pub fn handle_file(path : &PathBuf) -> MediaInfo {
    println!("{}", path.display());
    let     ext = path.extension().unwrap().to_str().unwrap();
    let mut f   = File::open(&path).unwrap();

    let mut ictx    = load_media(&path).unwrap();
    let     tagdict = ictx.metadata();
    let     title   = tagdict
        .get("title")
        .or_else(|| tagdict.get("TITLE"))
        .map(|s| s.to_string());

    let mut date    = None;
    let mut comment = None;

    let     source = MediaMetaSource::seekable(&f).unwrap();
    let mut parser = MediaMetaParser::new();
    if (source.has_exif()) {
        let meta = parser.parse::<_, _, ExifMetadata>(source).unwrap();
        for (mut entry) in meta { if let Some(tag) = entry.tag() {
            match (tag) {
                ExifMetaTag::CreateDate       => { date    = Some(entry.take_value().unwrap()); },
                ExifMetaTag::DateTimeOriginal => { date    = Some(entry.take_value().unwrap()); },
                ExifMetaTag::UserComment      => { comment = Some(entry.take_value().unwrap()); },
                _ => { }
            }
        } }
    } else if (source.has_track()) {
        let meta = parser.parse::<_, _, TrackMetadata>(source).unwrap();
        for entry in meta {
            match (entry.0) {
                TrackMetaTag::CreateDate => { date = Some(entry.1); },
                _ => { }
            }
        }
    }

    let date = date.map(|v| {
        let nd = match (v) {
            MediaMetaValue::Time(dt)          => dt.date_naive(),
            MediaMetaValue::NaiveDateTime(dt) => dt.date(),
            _ => panic!("Unsupported date type {v:?}")
        };
        (nd.year().try_into().unwrap(), nd.month().try_into().unwrap(), nd.day().try_into().unwrap(),)
    });
    let comment = comment.and_then(|v| {
        match (v) {
            MediaMetaValue::Text(t)      => Some(t),
            MediaMetaValue::U8Array(v)   => String::from_utf8(v).ok(),
            MediaMetaValue::Undefined(v) => String::from_utf8(v).ok(),
            _ => None
        }
    });
    let title = title.or_else(|| {
        comment.and_then(|c| ((! c.starts_with("ASCII@")) && json_from_str::<JsonValue>(&c).is_err()).then(|| c))
    }).unwrap_or_default();

    let thumbnail;
    let resolution;
    let kind;

    match (ext) {
        "jpg" | "jpeg" => {
            f.rewind().unwrap();
            thumbnail  = path.to_path_buf();
            resolution = ImageReader::new(BufReader::new(&f)).with_guessed_format().unwrap().into_dimensions().unwrap();
            kind       = MediaKind::Image;
        },
        "webm" | "mp4" => {
            let generated = video::generate_thumbnail(&mut ictx, &path);
            thumbnail  = generated.path;
            resolution = (generated.image.width(), generated.image.height(),);
            kind       = MediaKind::Video;
        },
        _ => panic!("unknown media format {ext:?}")
    }


    let source_path = path.strip_prefix("site").unwrap().to_str().unwrap().to_string();
    MediaInfo {
        date,
        thumbnail   : thumbnail.strip_prefix("site").unwrap().to_str().unwrap().to_string(),
        resolution,
        label       : title,
        edited      : source_path.contains(".edited"),
        kind,
        is_cover    : source_path.contains(".cover"),
        source      : source_path
    }
}
