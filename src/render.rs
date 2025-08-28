use crate::{
    Section, Gallery,
    media::{ MediaGroup, MediaInfo, MediaKind },
    util::rand_string
};
use maud::{ Markup, html };


pub fn render_sections(sections : &[Section]) -> Markup { html!{
    div #section_links {
        hr;
        h1 {
            @for (i, section,) in sections.iter().enumerate() {
                @if (i > 0) { p { "-" } }
                a href=(format!("#s{}", section.name)) { p { (section.name) } }
            }
        }
        hr;
    }
    div #sections_wrapper {
        div #sections {
            @for section in sections {

                div .section #(format!("s{}", section.name)) {
                    h1 { (section.name) }
                    hr;
                    (render_galleries(&section.galleries))
                    hr;
                }

            }
    } }
} }


fn render_galleries(galleries : &[Gallery]) -> Markup { html!{
    div .galleries {
        @for gallery in galleries { @match (gallery) {
            Gallery::Wide(group)   => { (render_gallery_wide(group)) },
            Gallery::Tiled(groups) => {
                @if (groups.len() == 1) {
                    (render_gallery_wide(&groups[0]))
                } @else {
                    (render_gallery_tiled(groups))
                }
            }
        } }
    }
} }


fn render_gallery_wide(group : &MediaGroup) -> Markup { html!{
    div .gallery_wide { (render_group(group)) }
} }


fn render_gallery_tiled(groups : &[MediaGroup]) -> Markup {
    let mut left   = Vec::with_capacity(groups.len() / 2 + 1);
    let mut right  = Vec::with_capacity(left.capacity());
    let mut hleft  = 0.0;
    let mut hright = 0.0;
    for group in groups {
        let cover_media = &group.medias[group.cover_idx];
        let ar          = (cover_media.resolution.1 as f32) / (cover_media.resolution.0 as f32);
        if (hleft <= hright) {
            hleft += ar;
            left.push(group);
        } else {
            hright += ar;
            right.push(group);
        }
    }
    html!{ div .gallery_tiled {
        div {
            @for group in left { (render_group(group)) }
        }
        div {
            @for group in right { (render_group(group)) }
        }
    } }
}


fn render_group(group : &MediaGroup) -> Markup {
    let cover_media = &group.medias[group.cover_idx];
    html!{
        div .img_wrapper {
            input type="checkbox" name="open_img_group";
            img src=(cover_media.thumbnail);
            @if (cover_media.kind == MediaKind::Video ) {
                div .watermark_video { span { "▶" } }
            }
            h1 .img_wrapper_group_count {
                @for _ in 0..group.medias.len() { "◆" }
            }
            div .img_group {
                div .img_group_main { }
                div .img_group_select_wrapper {
                    div .img_group_select {
                        @let group_name = rand_string::<16>();
                        @for (i, media,) in group.medias.iter().enumerate() {
                            (render_media(media, i == group.cover_idx, &group_name))
                        }
                    }
                }
            }
        }
    }
}


fn render_media(media : &MediaInfo, selected : bool, group_name : &str) -> Markup { html!{
    div .img_group_select_option {
        input type="radio" checked[selected]name=(group_name);
        img src=(media.thumbnail);
        @if (media.kind == MediaKind::Video ) {
            div .watermark_video { span { "▶" } }
        }
        div .img_single {
            @match (media.kind) {

                MediaKind::Image => {
                    img .media src=(media.source);
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
                        source src=(media.source);
                    }
                }

            }
            h1 .img_single_label { (media.label) }
            h1 .img_single_date {
                @if let Some((y, m, d,)) = media.date {
                    (format!("{y:0>4}-{m:0>2}-{d:0>2}"))
                }
            }
            @if (media.edited) {
                h1 .img_single_edited { "✎" }
            }
        }
    }
} }
