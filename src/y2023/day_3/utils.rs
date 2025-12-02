#[derive(Debug)]
pub struct Num {
    pub line: usize,
    pub start_ind: usize,
    pub end_ind: usize,
}

pub fn to_str<'a>(line: usize, start: usize, end: usize, bytes: &'_ [&'a [u8]]) -> &'a str {
    std::str::from_utf8(&bytes[line][start..=end]).unwrap()
}

pub struct Gear {
    pub nums_adj: [(usize, usize); 2],
}

impl From<Vec<(usize, usize)>> for Gear {
    fn from(value: Vec<(usize, usize)>) -> Self {
        Self {
            nums_adj: value.try_into().unwrap(),
        }
    }
}

pub fn is_digit(c: &u8) -> bool {
    (&b'0'..=&b'9').contains(&c)
}

pub fn adj_symbol(
    bytes_arr: &[&[u8]],
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
) -> bool {
    // we need a loop to return early
    #[allow(clippy::needless_range_loop)]
    for x in start_x..=end_x {
        for y in start_y..=end_y {
            if is_symbol(&bytes_arr[x][y]) {
                return true;
            }
        }
    }
    false
}

pub fn is_symbol(c: &u8) -> bool {
    !is_digit(c) && c != &b'.'
}
