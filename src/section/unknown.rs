use crate::{ Section, Gallery, Group, Media, Date };


pub const UNKNOWN : Section = Section {
    name      : "Unknown",
    galleries : &[

        Gallery::Wide(Group(&[
            Media::new("Flippy chicken", Date::Unknown, "gallery/unknown/flippy_chicken.jpg")
        ])),

        Gallery::Tiled(&[
            Group(&[
                Media::new("Bird concern", Date::Unknown, "gallery/unknown/birdconcern.png"),
                Media::new("WHERE IS MY FOOD!?", Date::Unknown, "gallery/unknown/cirdboncern.png").edited()
            ])
        ])

    ]
};
