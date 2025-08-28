use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{ self, File },
    io::Write
};
use maud::{ DOCTYPE, html };

mod util;
use util::cmp_date;

mod media;
use media::{ MediaGroup, MediaDisplayKind };

mod render;
use render::render_sections;


struct Section {
    name      : Cow<'static, str>,
    galleries : Vec<Gallery>
}
enum Gallery {
    Wide(MediaGroup),
    Tiled(Vec<MediaGroup>)
}

fn main() {
     _ = fs::remove_dir_all("site/generated");
     fs::create_dir("site/generated").unwrap();

    let mut raw_sections = HashMap::<Option<u16>, Vec<MediaGroup>>::new();
    for entry in fs::read_dir("site/media").unwrap().map(|e| e.unwrap()) {
        let     etype = entry.file_type().unwrap();
        let     epath = entry.path();
        let mut group = if (etype.is_dir()) {
            media::handle_dir(&epath)
        } else if (etype.is_file()) {
            MediaGroup::from(media::handle_file(&epath))
        } else {
            panic!("{:?} is not a directory or file", epath);
        };
        if (epath.file_name().unwrap().to_str().unwrap().contains(".wide")) {
            group.kind = MediaDisplayKind::Wide;
        }
        raw_sections.entry(group.medias[group.cover_idx].date.map(|(y, _, _,)| y)).or_default()
            .push(group);
    }

    let mut raw_sections = raw_sections.into_iter().collect::<Vec<_>>();
    raw_sections.sort_by(|(a, _,), (b, _,)| a.is_none().cmp(&b.is_none()).then(a.cmp(b)));
    let mut sections = Vec::new();
    for (key, mut groups,) in raw_sections {
        groups.sort_by(|a, b| cmp_date(a.medias[a.cover_idx].date, b.medias[b.cover_idx].date));
        let mut galleries = Vec::new();
        for group in groups { match (group.kind) {
            MediaDisplayKind::Wide  => { galleries.push(Gallery::Wide(group)) },
            MediaDisplayKind::Tiled => { match (galleries.last_mut()) {
                None | Some(Gallery::Wide(_)) => { galleries.push(Gallery::Tiled(vec![group])); },
                Some(Gallery::Tiled(t))       => { t.push(group); }
            } }
        } }
        sections.push(Section {
            name      : key.map_or(Cow::Borrowed("Unknown"), |k| Cow::Owned(format!("{k:0>4}"))),
            galleries
        });
    }

    let main = html!{ (DOCTYPE) html {
        head {
            link rel="stylesheet" type="text/css" href="style.css";
        }
        body {
            div #header {
                h1 { "Toto" }
                img src="toto_peek.png";
            }
            (render_sections(&sections))
            div #footer { }
            script src="script.js" { }
        }
    } };

    let mut f = File::create("site/index.html").unwrap();
    write!(f, "{}", main.into_string()).unwrap();
}
