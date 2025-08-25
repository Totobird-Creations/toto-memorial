use maud::{ Markup, Render, html };
use image::ImageReader;


pub struct Section {
    pub name      : &'static str,
    pub galleries : &'static [Gallery]
}
impl Render for Section {
    fn render(&self) -> Markup { html!{
        div .section #(format!("s{}", self.name)) {
            h1 { (self.name) }
            hr;
            div .galleries {
                @for gallery in self.galleries {
                    (gallery.render(self.name))
                }
            }
            hr;
        }
    } }
}


pub enum Gallery {
    Wide(Group),
    Tiled(&'static [Group])
}
impl Gallery {
    fn render(&self, section_name : &str) -> Markup {
        match (self) {
            Self::Wide(group) => html!{
                div .gallery_wide {
                    (group.render(section_name, "w0"))
                }
            },
            Self::Tiled(groups) => {
                let mut left         = Vec::with_capacity(groups.len() / 2 + 1);
                let mut left_height  = 0.0;
                let mut right        = Vec::with_capacity(groups.len() / 2 + 1);
                let mut right_height = 0.0;
                for group in *groups {
                    let image = &group.0[0];
                    let dims  = ImageReader::open(format!("site/{}", image.path)).unwrap()
                        .into_dimensions().unwrap();
                    let ar    = (dims.1 as f32) / (dims.0 as f32);
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


pub struct Group(pub &'static [Image]);
impl Group {
    fn render(&self, section_name : &str, gallery_name : &str) -> Markup {
        let first_img = &self.0[0];
        html!{
            div .img_wrapper {
                input type="checkbox" name="open_img_group";
                img src=(first_img.path);
                h1 .img_wrapper_above {
                    @for _ in 0..self.0.len() { "◆" }
                }
                h1 .img_wrapper_below {
                    @for _ in 0..self.0.len() { "◆" }
                }
                div .img_group {
                    div .img_group_main { }
                    div .img_group_select_wrapper {
                        div .img_group_select {
                            @for (i, img,) in self.0.iter().enumerate() {

                                @let group_id = format!("s{section_name}_g{gallery_name}");
                                div .img_group_select_option {
                                    input type="radio" name=(group_id) checked[i == 0];
                                    img src=(img.path);
                                    div .img_single {
                                        img src=(img.path);
                                        h1 .img_single_label { (img.label) }
                                        h1 .img_single_date { (img.date) }
                                        @if (img.edited) {
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


pub struct Image {
    pub label  : &'static str,
    pub date   : Date,
    pub path   : &'static str,
    pub edited : bool
}
impl Image {

    #[inline(always)]
    pub const fn new(label : &'static str, date : Date, path : &'static str) -> Self {
        Self { label, date, path, edited : false }
    }

    #[inline(always)]
    pub const fn edited(mut self) -> Self {
        self.edited = true;
        self
    }

}


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
