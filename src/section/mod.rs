use crate::Section;


mod s2013;
mod s2014;
mod s2015;
mod s2016;
mod s2017;
mod s2018;
mod s2019;
mod s2020;
mod s2021;
mod s2022;
mod s2023;
mod s2024;
mod s2025;
mod unknown;


pub const SECTIONS : &[Section] = &[
    s2013::S2013,
    s2014::S2014,
    s2015::S2015,
    s2016::S2016,
    s2017::S2017,
    s2018::S2018,
    s2019::S2019,
    s2020::S2020,
    s2021::S2021,
    s2022::S2022,
    s2023::S2023,
    s2024::S2024,
    s2025::S2025,
    unknown::UNKNOWN
];
