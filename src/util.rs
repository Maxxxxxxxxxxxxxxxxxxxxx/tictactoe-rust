use std::io;

pub fn read_buffer() -> std::io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn vec_is_subset<T: std::cmp::PartialEq>(vec_a: &Vec<T>, vec_b: &Vec<T>) -> bool {
    let mut state: bool = false;
    let mut count_valid: usize = 0;

    for i in 0..vec_a.len() {
        let current = &vec_a[i];
        if vec_find(&vec_b, current) {
            count_valid += 1;
        }
    }

    if count_valid == vec_a.len() { state = true }

    return state;
}

pub fn parse_winline(winline: [usize; 3]) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new();
    for digit in winline {
        let copy = digit.clone();
        vec.push(copy)
    }
    return vec;
}

pub fn charset_indexof(charset: [char; 9], ch: char) -> Option<usize> {
    let mut index: Option<usize> = None;
    for i in 0..charset.len() {
        if charset[i] == ch { index = Some(i); break }
    }
    return index
}

pub fn clear_console() {
    print!("{}[2J", 27 as char);
}
    
fn vec_find<T: std::cmp::PartialEq>(iterable: &Vec<T>, value: &T) -> bool {
    let mut state: bool = false;
    for element in iterable { if value == element { state = true; break } }
    return state;
}