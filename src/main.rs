use std::{
    fs::File,
    io::Write
};
use maud::{ DOCTYPE, html };


mod section;
use section::SECTIONS;

mod gallery;
use gallery::*;


fn main() {
    let main = html!{ (DOCTYPE) html {
        head {
            link rel="stylesheet" type="text/css" href="style.css";
        }
        body {
            div #header {
                h1 { "Toto" }
                img src="toto_peek.png";
            }
            div #section_links {
                hr;
                h1 {
                    @for (i, section, ) in SECTIONS.iter().enumerate() {
                        @if (i > 0) { p { "-" } }
                        a href=(format!("#s{}", section.name)) { p { (section.name) } }
                    }
                }
                hr;
            }
            div #sections_wrapper {
                div #sections {
                    @for section in SECTIONS {
                        (section)
                    }
            } }
            div #footer { }
        }
    } };

    let mut f = File::create("site/index.html").unwrap();
    write!(f, "{}", main.into_string()).unwrap();
}
