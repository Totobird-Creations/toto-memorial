use crate::{ Section, Gallery, Group, Image, Date };


pub const S2024 : Section = Section {
    name      : "2024",
    galleries : &[

        Gallery::Tiled(&[
            Group(&[
                Image::new("Peekaboo!", Date::YMD(2024, 12, 14), "gallery/2024/toto_in_a_calendar.jpg")
            ]),
            Group(&[
                Image::new("He's everywhere!", Date::YMD(2024, 12, 14), "gallery/2024/toto_everywhere.jpg").edited()
            ]),
            Group(&[
                Image::new("Rawr", Date::YMD(2024, 12, 15), "gallery/2024/totodino.jpg").edited()
            ]),
            Group(&[
                Image::new("Molotov 'Cockatiel'", Date::YMD(2024, 03, 26), "gallery/2024/firetoto.png").edited()
            ])
        ])

    ]
};
