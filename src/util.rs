use std::cmp::Ordering;


const RAND_CHARS : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn rand_string<const LEN : usize>() -> String {
    let mut string = [0u8; LEN];
    for i in 0..LEN {
        string[i] = unsafe { *RAND_CHARS.as_bytes().get_unchecked(rand::random_range(0..RAND_CHARS.len())) };
    }
    unsafe { String::from_utf8_unchecked(string.to_vec()) }
}


pub fn cmp_date(a : Option<(u16, u8, u8,)>, b : Option<(u16, u8, u8,)>) -> Ordering {
    match ((a, b,)) {
        (None, None,)    => Ordering::Equal,
        (Some(_), None,) => Ordering::Less,
        (None, Some(_),) => Ordering::Greater,
        (Some(a), Some(b),) => {
            a.0.cmp(&b.0)
                .then(a.1.cmp(&b.1))
                .then(a.2.cmp(&b.2))
        }
    }
}
