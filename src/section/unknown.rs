use crate::{ Section, Gallery, Group, Image, Date };


pub const UNKNOWN : Section = Section {
    name      : "Unknown",
    galleries : &[

        Gallery::Wide(Group(&[
            Image::new("Flippy chicken", Date::Unknown, "gallery/unknown/flippy_chicken.jpg")
        ])),

        Gallery::Tiled(&[
            Group(&[
                Image::new("Bird concern", Date::Unknown, "gallery/unknown/birdconcern.png"),
                Image::new("WHERE IS MY FOOD!?", Date::Unknown, "gallery/unknown/cirdboncern.png").edited()
            ])
        ])

    ]
};
