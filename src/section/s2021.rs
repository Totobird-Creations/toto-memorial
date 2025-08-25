use crate::{ Section, Gallery, Group, Image, Date };


pub const S2021 : Section = Section {
    name      : "2021",
    galleries : &[

        Gallery::Tiled(&[
            Group(&[
                Image::new("So Happy", Date::YMD(2021, 12, 16), "gallery/2021/toto_proud_1.jpg"),
                Image::new("So Proud", Date::YMD(2021, 12, 16), "gallery/2021/toto_proud_0.jpg")
            ])
        ])

    ]
};
