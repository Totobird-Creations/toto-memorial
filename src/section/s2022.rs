use crate::{ Section, Gallery, Group, Image, Date };


pub const S2022 : Section = Section {
    name      : "2022",
    galleries : &[

        Gallery::Wide(Group(&[
            Image::new("Hey, got any food?", Date::YMD(2022, 03, 17), "gallery/2022/toto_peek.png"),
            Image::new("Hey, got any food?", Date::YMD(2022, 03, 17), "gallery/2022/toto_peek_large.jpg")
        ])),

        Gallery::Tiled(&[
            Group(&[
                Image::new("You kinda look like food", Date::YMD(2022, 02, 11), "gallery/2022/piano_glare.jpg")
            ]),
            Group(&[
                Image::new("Warm in here", Date::YMD(2022, 09, 02), "gallery/2022/sleeve_bird.jpg")
            ])
        ])

    ]
};
