use crate::{ EXT_IMAGE, EXT_VIDEO };
use std::{
    borrow::Cow,
    fs::File,
    io::Write
};
use maud::{ Markup, Render, html };
use image::{
    ImageFormat, ImageReader,
    imageops::FilterType as ImageFilterType
};
use ffmpeg_next::{
    codec::context::Context as ContextDecoder,
    format::{
        input as load_video,
        Pixel as VideoPixel
    },
    media::Type as MediaType,
    software::scaling::{
        context::Context as ScalingContext,
        flag::Flags as ScalingFlags
    },
    util::frame::Video
};


const RAND_CHARS : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn rand_string<const LEN : usize>() -> String {
    let mut string = [0u8; LEN];
    for i in 0..LEN {
        string[i] = unsafe { *RAND_CHARS.as_bytes().get_unchecked(rand::random_range(0..RAND_CHARS.len())) };
    }
    unsafe { String::from_utf8_unchecked(string.to_vec()) }
}


#[derive(Clone)]
pub struct Section {
    pub name      : &'static str,
    pub galleries : &'static [Gallery]
}
impl Section {
    pub fn render(self) -> Markup { html!{
        div .section #(format!("s{}", self.name)) {
            h1 { (self.name) }
            hr;
            div .galleries {
                @for gallery in self.galleries {
                    (gallery.clone().render(self.name))
                }
            }
            hr;
        }
    } }
}


#[derive(Clone)]
pub enum Gallery {
    Wide(Group),
    Tiled(&'static [Group])
}

impl Gallery {
    fn render(self, section_name : &str) -> Markup {
        match (self) {
            Self::Wide(group) => {
                html!{
                    div .gallery_wide {
                        (group.into_thumbnailed().render(section_name, "w0"))
                    }
                }
            },
            Self::Tiled(groups) => {
                let mut left         = Vec::with_capacity(groups.len() / 2 + 1);
                let mut left_height  = 0.0;
                let mut right        = Vec::with_capacity(groups.len() / 2 + 1);
                let mut right_height = 0.0;
                for group in groups {
                    let group = group.clone().into_thumbnailed();
                    let media = &group.0[0];
                    let ar    = (media.thumbnail_dims.1 as f32) / (media.thumbnail_dims.0 as f32);
                    if (left_height <= right_height) {
                        left_height += ar;
                        left.push(group);
                    } else {
                        right_height += ar;
                        right.push(group);
                    }
                }
                html!{
                    div .gallery_tiled {
                        div {
                            @for (i, group,) in left.iter().enumerate() {
                                (group.render(section_name, &format!("tl{i}")))
                            }
                        }
                        div {
                            @for (i, group,) in right.iter().enumerate() {
                                (group.render(section_name, &format!("tr{i}")))
                            }
                        }
                    }
                }
            }
        }
    }
}


#[derive(Clone)]
pub struct Group(pub &'static [Media]);
struct ThumbnailedGroup(Vec<ThumbnailedMedia>);

impl Group {
    fn into_thumbnailed(self) -> ThumbnailedGroup {
        ThumbnailedGroup(self.0.into_iter().map(|media| media.clone().into_thumbnailed()).collect::<Vec<_>>())
    }
}

impl ThumbnailedGroup {
    fn render(&self, section_name : &str, gallery_name : &str) -> Markup {
        let first_item = &self.0[0];
        html!{
            div .img_wrapper {
                input type="checkbox" name="open_img_group";
                img src=(first_item.thumbnail_path);
                @if (first_item.kind == MediaKind::Video ) {
                    div .watermark_video { span { "▶" } }
                }
                h1 .img_wrapper_group_count {
                    @for _ in 0..self.0.len() { "◆" }
                }
                div .img_group {
                    div .img_group_main { }
                    div .img_group_select_wrapper {
                        div .img_group_select {
                            @for (i, item,) in self.0.iter().enumerate() {

                                @let group_id = format!("s{section_name}_g{gallery_name}");
                                div .img_group_select_option {
                                    input type="radio" name=(group_id) checked[i == 0];
                                    img src=(item.thumbnail_path);
                                    @if (item.kind == MediaKind::Video ) {
                                        div .watermark_video { span { "▶" } }
                                    }
                                    div .img_single {
                                        @match (item.kind) {

                                            MediaKind::Image => {
                                                img .media src=(item.source.path);
                                            },

                                            MediaKind::Video => {
                                                div .video_control hidden {
                                                    input .video_control_play type="checkbox" name="toggle_video_play";
                                                    div .video_control_seek {
                                                        div .video_control_seek_bars {
                                                            div .video_control_seek_progress { }
                                                            div .video_control_seek_hover { }
                                                        }
                                                    }
                                                }
                                                video .media controls {
                                                    source src=(item.source.path);
                                                }
                                            }

                                        }
                                        h1 .img_single_label { (item.source.label) }
                                        h1 .img_single_date { (item.source.date) }
                                        @if (item.source.edited) {
                                            h1 .img_single_edited { "✎" }
                                        }
                                    }
                                }

                            }
                        }
                    }
                }
            }
        }
    }
}


#[derive(Clone)]
pub struct Media {
    pub label  : &'static str,
    pub date   : Date,
    pub path   : &'static str,
    pub edited : bool
}
impl Media {

    #[inline(always)]
    pub const fn new(label : &'static str, date : Date, path : &'static str) -> Self {
        Self { label, date, path, edited : false }
    }

    #[inline(always)]
    pub const fn edited(mut self) -> Self {
        self.edited = true;
        self
    }

    fn into_thumbnailed(self) -> ThumbnailedMedia {
        for ext in EXT_IMAGE {
            if (self.path.ends_with(ext)) {
                let dims = ImageReader::open(format!("site/{}", self.path)).unwrap()
                    .into_dimensions().unwrap();
                return ThumbnailedMedia {
                    thumbnail_path : Cow::Borrowed(self.path),
                    thumbnail_dims : dims,
                    kind           : MediaKind::Image,
                    source         : self
                };
            }
        }
        for ext in EXT_VIDEO {
            if (self.path.ends_with(ext)) {
                let mut ictx         = load_video(&format!("site/{}", self.path)).unwrap();
                let     video_input  = ictx.streams().best(MediaType::Video).unwrap();
                let     stream_index = video_input.index();
                let     ctxdecoder   = ContextDecoder::from_parameters(video_input.parameters()).unwrap();
                let mut decoder      = ctxdecoder.decoder().video().unwrap();
                let mut decoded      = Video::empty();
                'packet_loop : for (stream, packet,) in ictx.packets() {
                    if (stream.index() == stream_index) {
                        decoder.send_packet(&packet).unwrap();
                        while (decoder.receive_frame(&mut decoded).is_ok()) {
                            break 'packet_loop;
                        }
                    }
                }
                let mut frame = Video::empty();
                ScalingContext::get(decoder.format(), decoder.width(), decoder.height(), VideoPixel::RGB24, decoder.height(), decoder.width(), ScalingFlags::BILINEAR).unwrap()
                    .run(&decoded, &mut frame).unwrap();
                drop(decoded);
                let     dims = (frame.width(), frame.height(),);
                let     thumbnail_path = format!("generated/{}-{}.jpg", self.path.split('/').last().unwrap(), rand_string::<16>());
                let     real_path      = format!("site/{}", thumbnail_path);
                let mut thumbnail      = File::create(&real_path).unwrap();
                thumbnail.write_all(format!("P6\n{} {}\n255\n", dims.0, dims.1).as_bytes()).unwrap();
                thumbnail.write_all(frame.data(0)).unwrap();
                let     thumbnail = ImageReader::open(&real_path).unwrap().with_guessed_format().unwrap().decode().unwrap();
                thumbnail.resize_exact(thumbnail.height(), thumbnail.width(), ImageFilterType::Gaussian)
                    .save_with_format(&real_path, ImageFormat::Jpeg).unwrap();
                drop(thumbnail);
                return ThumbnailedMedia {
                    thumbnail_path : Cow::Owned(thumbnail_path),
                    thumbnail_dims : dims,
                    kind           : MediaKind::Video,
                    source         : self
                };
            }
        }
        panic!("Unrecognised extension for file {:?}", self.path);
    }

}


#[derive(Clone, Copy)]
pub enum Date {
    YMD(u32, u8, u8),
    Y(u32),
    Unknown
}
impl Render for Date {
    fn render(&self) -> Markup { match (self) {
        Self::YMD(y, m, d) => html!{ (format!("{y:0>4}-{m:0>2}-{d:0>2}")) },
        Self::Y(y)         => html!{ (format!("{y:0>4}")) },
        Self::Unknown      => html!{ }
    } }
}


struct ThumbnailedMedia {
    thumbnail_path : Cow<'static, str>,
    thumbnail_dims : (u32, u32,),
    kind           : MediaKind,
    source         : Media
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum MediaKind {
    Image,
    Video
}
