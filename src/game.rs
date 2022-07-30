use crate::util;

const STD_CHAR_CROSS:  char = 'X';
const STD_CHAR_CIRCLE: char = 'O';
const STD_CHAR_EMPTY:  char = ' ';

const STD_KEYSET: [char; 9] = [
    'q', 'w', 'e',
    'a', 's', 'd',
    'z', 'x', 'c' 
];

const STD_WINLINES: [[usize; 3]; 8] = [
    [0,1,2], [3,4,5], [6,7,8],  // horizontal 
    [0,3,6], [1,4,7], [2,5,8],  // vertical 
    [0,4,8], [6,4,2]            // diagonal
];

// 0 1 2
// 3 4 5
// 6 7 8

pub struct Game {
    board: Matrix,
    is_end: bool,
    is_player_turn: bool,
    turn: i32,
    p1_char: char,
    p2_char: char,
    key_set: [char; 9],
    pub p1_indices: Vec<usize>,
    pub p2_indices: Vec<usize>,
}

pub struct Matrix {
    size: usize,
    xdim: i32,
    ydim: i32,
    pub fields: Vec<Point>,
}

pub struct Point {
    x: i32,
    y: i32,
    pub is_drawn: bool,
    pub sign: char,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { 
            x: x, 
            y: y, 
            is_drawn: false, 
            sign: STD_CHAR_EMPTY,
        }
    }

    pub fn sign(&mut self, sign: char) {
        self.sign = sign;
        self.is_drawn = true;
    }
}

impl Matrix {
    pub fn new(xdim: i32, ydim: i32) -> Matrix {
        let size: usize = (xdim*ydim) as usize;
        let mut vec: Vec<Point> = Vec::with_capacity(size);
        let mut x_coord: i32 = 0;
        let mut y_coord: i32 = xdim;

        for i in 0..size {
            if i % xdim as usize == 0 {
                x_coord = 0;
                y_coord -= 1;
            }
            vec.push(Point::new(x_coord, y_coord));
            x_coord += 1;
        }

        Matrix {
            xdim: xdim,
            ydim: ydim,
            size: size,
            fields: vec 
        }
    }

    fn index_to_coord(&self, index: usize) -> Vec<i32> {
        let point = &self.fields[index];
        return vec![point.x, point.y]
    }

    fn point_indexof(&self, x_dim: i32, y_dim: i32) -> usize {
        let mut val: usize = 0;
        for i in 0..self.fields.len() {
            let pnt = &self.fields[i];              // finds point by given 
            if pnt.x == x_dim && pnt.y == y_dim {   // x and y coords
                val = i;                            // and returns its index in the vector
                break;
            }
        }
        return val;
    }

    pub fn show(&self) {
        for i in 0..self.fields.len() {
            let fld = &self.fields[i];

            print!("[{}] ", fld.sign);
            if (i+1) % self.xdim as usize == 0 { print!("\n") }
        }
    }

    pub fn show_with_coords(&self) {
        for i in 0..self.fields.len() {
            let fld = &self.fields[i];

            print!("[{}, {}] ", fld.x, fld.y);
            if (i+1) % self.xdim as usize == 0 { print!("\n") }
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Matrix::new(3, 3),
            is_end: false,
            is_player_turn: true,
            turn: 0,

            p1_char:    STD_CHAR_CROSS,
            p2_char:    STD_CHAR_CIRCLE,
            key_set:    STD_KEYSET,
            p1_indices: Vec::new(),
            p2_indices: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        while !self.is_end {

            util::clear_console();
            println!("Ruch: {}", self.turn);
            &self.board.show();

            let input_char: char = util::read_buffer()
                .unwrap()
                .chars()
                .next()
                .unwrap();

            let index = util::charset_indexof(STD_KEYSET, input_char);
            let coord_vec2= &self.board.index_to_coord(index.unwrap());

            self.put(coord_vec2[0], coord_vec2[1]);
            self.is_player_turn = !self.is_player_turn;
            self.turn += 1;

            // println!("point: ({}, {}), index in keyset: {}", coord_vec2[0], coord_vec2[1], index.unwrap());
        }
    }

    fn put(&mut self, x: i32, y: i32) {
        let point_index = self.board.point_indexof(x, y);
        if self.is_player_turn {
            self.p1_indices.push(point_index);
            self.board.fields[point_index].sign(self.p1_char);
        } 
        else {
            self.p2_indices.push(point_index);
            self.board.fields[point_index].sign(self.p2_char);
        }
        self.check_win();
    }

    fn end(&mut self) {
        let p_number = if self.is_player_turn { '1' } else { '2' };

        self.is_end = true;
        util::clear_console();
        self.board.show();
        println!("Wygrywa gracz {}! Liczba ruchow: {}", p_number, self.turn);
    }

    fn check_win(&mut self) -> bool {
        let indices = if self.is_player_turn { &self.p1_indices } else { &self.p2_indices };
        for win_line in STD_WINLINES {
            let winline_vec = util::parse_winline(win_line);
            let state = util::vec_is_subset(&winline_vec, indices);
            if state { self.end(); break }
        }
        return true;
    }

    pub fn test(&mut self) {
        &self.board.show();
        print!("\n");
        // println!("{}", &self.board.point_indexof(1,1))
        // &self.board.show_with_coords();
    }
}