use crate::{ Section, Gallery, Group, Media, Date };

pub const S2023 : Section = Section {
    name      : "2023",
    galleries : &[

        Gallery::Wide(Group(&[
            Media::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_2.jpg"),
            Media::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_3.jpg"),
            Media::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_4.jpg"),
            Media::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_0.jpg"),
            Media::new("Cuddles", Date::YMD(2023, 02, 15), "gallery/2023/toto_cuddle_1.jpg")
        ])),

        Gallery::Tiled(&[
            Group(&[
                Media::new("So sleepy", Date::YMD(2023, 06, 28), "gallery/2023/toto_squish_2.jpg"),
                Media::new("So sleepy", Date::YMD(2023, 06, 28), "gallery/2023/toto_squish_0.jpg"),
                Media::new("So sleepy", Date::YMD(2023, 06, 28), "gallery/2023/toto_squish_1.jpg"),
                Media::new("So sleepy", Date::YMD(2023, 06, 28), "gallery/2023/toto_squish_3.jpg")
            ]),
            Group(&[
                Media::new("Tasty dice", Date::YMD(2023, 02, 19), "gallery/2023/diceroll.mp4"),
                Media::new("Tasty dice", Date::YMD(2023, 02, 19), "gallery/2023/diceroll.mp4")
            ])
        ])

    ]
};
