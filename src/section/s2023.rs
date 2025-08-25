use crate::{ Section, Gallery, Group, Image, Date };


pub const S2023 : Section = Section {
    name      : "2023",
    galleries : &[

        Gallery::Wide(Group(&[
            Image::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_2.jpg"),
            Image::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_3.jpg"),
            Image::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_4.jpg"),
            Image::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_0.jpg"),
            Image::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_1.jpg")
        ])),

        Gallery::Tiled(&[
            Group(&[
                Image::new("So Sleepy", Date::YMD(2023, 06, 28), "gallery/2023/toto_squish_0.jpg"),
                Image::new("So Sleepy", Date::YMD(2023, 06, 28), "gallery/2023/toto_squish_1.jpg"),
                Image::new("So Sleepy", Date::YMD(2023, 06, 28), "gallery/2023/toto_squish_2.jpg"),
                Image::new("So Sleepy", Date::YMD(2023, 06, 28), "gallery/2023/toto_squish_3.jpg")
            ])
        ])

    ]
};
