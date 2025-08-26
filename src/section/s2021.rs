use crate::{ Section, Gallery, Group, Media, Date };


pub const S2021 : Section = Section {
    name      : "2021",
    galleries : &[

        Gallery::Tiled(&[
            Group(&[
                Media::new("So Happy", Date::YMD(2021, 12, 16), "gallery/2021/toto_proud_1.jpg"),
                Media::new("So Proud", Date::YMD(2021, 12, 16), "gallery/2021/toto_proud_0.jpg")
            ])
        ])

    ]
};
