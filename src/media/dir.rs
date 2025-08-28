use crate::{
    media::{ MediaInfo, handle_file },
    util::cmp_date
};
use std::{
    fs,
    path::Path
};


#[derive(Debug, Default)]
pub struct MediaGroup {
    pub medias    : Vec<MediaInfo>,
    pub cover_idx : usize,
    pub kind      : MediaDisplayKind
}
impl From<MediaInfo> for MediaGroup {
    fn from(info : MediaInfo) -> Self {
        Self { medias : vec![info], ..Self::default() }
    }
}

#[derive(Debug, Default)]
pub enum MediaDisplayKind {
    Wide, // TODO
    #[default]
    Tiled
}


pub fn handle_dir(path : &Path) -> MediaGroup {
    let mut group = MediaGroup::default();
    for entry in fs::read_dir(&path).unwrap().map(|e| e.unwrap()) {
        let media = handle_file(&entry.path());
        if (media.is_cover) {
            group.cover_idx = group.medias.len();
        }
        group.medias.push(media);
    }
    group.medias.sort_by(|a, b| cmp_date(a.date, b.date));
    group
}
