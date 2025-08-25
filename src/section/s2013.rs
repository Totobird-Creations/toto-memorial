use crate::{ Section, Gallery, Group, Image, Date };


pub const S2013 : Section = Section {
    name      : "2013",
    galleries : &[

        Gallery::Wide(Group(&[
            Image::new("First halloween", Date::YMD(2013, 10, 30), "gallery/2013/first_halloween.jpg")
        ]))

    ]
};
