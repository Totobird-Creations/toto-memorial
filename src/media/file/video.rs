use crate::util::rand_string;
use std::{
    io::Cursor,
    path::{ Path, PathBuf }
};
use image::{
    DynamicImage,
    ImageFormat,
    ImageReader
};
use ffmpeg_next::{
    codec::context::Context as ContextDecoder,
    format::{
        Pixel as VideoPixel,
        context::Input as MediaInput
    },
    media::Type as MediaType,
    software::scaling::{
        context::Context as ScalingContext,
        flag::Flags as ScalingFlags
    },
    util::frame::Video
};


pub struct Thumbnail {
    pub image : DynamicImage,
    pub path  : PathBuf
}

pub fn generate_thumbnail(ictx : &mut MediaInput, video_path : &Path) -> Thumbnail {
    let thumbnail_path = format!("site/generated/{}-{}.jpg",
        video_path.file_name().unwrap().to_str().unwrap(),
        rand_string::<16>()
    ).into();

    let     input   = ictx.streams().best(MediaType::Video).unwrap();
    let     index   = input.index();
    let     ctxdec  = ContextDecoder::from_parameters(input.parameters()).unwrap();
    let mut decoder = ctxdec.decoder().video().unwrap();
    let mut decoded = Video::empty();
    let mut success = false;
    for (stream, packet,) in ictx.packets() {
        if (stream.index() == index) {
            decoder.send_packet(&packet).unwrap();
            if (decoder.receive_frame(&mut decoded).is_ok()) {
                success = true;
                break;
            }
        }
    }
    if (! success) {
        decoder.send_eof().unwrap();
        assert!(decoder.receive_frame(&mut decoded).is_ok());
    }
    let mut frame = Video::empty();
    ScalingContext::get(decoder.format(), decoder.width(), decoder.height(), VideoPixel::RGB24, decoder.width(), decoder.height(), ScalingFlags::BILINEAR).unwrap()
        .run(&decoded, &mut frame).unwrap();
    drop(decoded);

    let thumbnail_data = format!("P6\n{} {}\n255\n", frame.width(), frame.height())
        .as_bytes().into_iter()
        .chain(frame.data(0))
        .cloned()
        .collect::<Vec<_>>();
    let thumbnail = ImageReader::new(Cursor::new(thumbnail_data))
        .with_guessed_format().unwrap()
        .decode().unwrap();
    thumbnail.save_with_format(&thumbnail_path, ImageFormat::Jpeg).unwrap();

    Thumbnail {
        image : thumbnail,
        path  : thumbnail_path
    }
}
