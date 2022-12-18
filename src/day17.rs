#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tile {
    Frozen,
    Falling,
    Empty,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Left = -1,
    Right = 1,
}
#[derive(Debug)]
struct Grid {
    tiles: Vec<[Tile; 7]>,
}

impl Grid {
    fn new() -> Self {
        Self { tiles: vec![] }
    }
    fn blow(&mut self, direction: Direction) {
        let dir = direction as isize;
        let can_move = self.tiles.iter().all(|row| {
            row.iter().enumerate().all(|(idx, tile)| {
                if *tile == Tile::Falling {
                    let Some(new_idx) = idx.checked_add_signed(dir) else { return false };
                    matches!(row.get(new_idx), Some(Tile::Empty | Tile::Falling))
                } else {
                    true
                }
            })
        });
        if can_move {
            for row in self.tiles.iter_mut() {
                match direction {
                    Direction::Left => {
                        for i in 0..7 {
                            if row[i] == Tile::Falling {
                                row[i] = Tile::Empty;
                                row[i.wrapping_add_signed(dir)] = Tile::Falling;
                            }
                        }
                    }
                    Direction::Right => {
                        for i in (0..7).rev() {
                            if row[i] == Tile::Falling {
                                row[i] = Tile::Empty;
                                row[i.wrapping_add_signed(dir)] = Tile::Falling;
                            }
                        }
                    }
                }
            }
        }
    }
    fn drop(&mut self) -> bool {
        if self.tiles.iter().enumerate().rev().all(|(row_idx, row)| {
            row.iter().enumerate().all(|(col_idx, tile)| {
                if row_idx == 0 {}
                if *tile == Tile::Falling {
                    let Some(new_row_idx) = row_idx.checked_sub(1) else { return false };
                    matches!(
                        self.tiles.get(new_row_idx).and_then(|row| row.get(col_idx)),
                        Some(Tile::Empty | Tile::Falling)
                    )
                } else {
                    true
                }
            })
        }) {
            for row_idx in 1..self.tiles.len() {
                for col_idx in 0..7 {
                    if self.tiles[row_idx][col_idx] == Tile::Falling {
                        self.tiles[row_idx][col_idx] = Tile::Empty;
                        self.tiles[row_idx - 1][col_idx] = Tile::Falling;
                    }
                }
            }
            true
        } else {
            false
        }
    }
    fn push_row(&mut self, row: [Tile; 7]) {
        self.tiles.push(row);
    }
    fn push_rows(&mut self, rows: &[[Tile; 7]]) {
        self.tiles.extend_from_slice(rows);
    }
    fn pop_empty_rows(&mut self) {
        while self
            .tiles
            .last()
            .unwrap()
            .iter()
            .all(|tile| *tile == Tile::Empty)
        {
            self.tiles.pop();
        }
    }
    fn freeze(&mut self) {
        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                if *tile == Tile::Falling {
                    *tile = Tile::Frozen;
                }
            }
        }
    }
    fn dbg(&self) {
        for row in self.tiles.iter().rev() {
            for tile in row.iter() {
                match tile {
                    Tile::Frozen => print!("#"),
                    Tile::Falling => print!("O"),
                    Tile::Empty => print!("."),
                }
            }
            println!();
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut winds = input
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid input"),
        })
        .cycle();
    let mut i = 0;
    let mut grid = Grid::new();
    let rocks = [
        &HORIZONTAL as &[[Tile; 7]],
        &PLUS,
        &ANGLE,
        &VERTICAL,
        &SQUARE,
    ]
    .into_iter()
    .cycle()
    .take(2022);
    for rock in rocks {
        i += 1;
        grid.push_rows(&EMPTY);
        grid.push_rows(rock);
        while {
            grid.blow(winds.next().unwrap());
            let res = grid.drop();
            res
        } {}
        grid.pop_empty_rows();
        grid.freeze();
        println!("{i}");
    }
    grid.tiles.len()
}

const VERTICAL: [[Tile; 7]; 4] = [[
    Tile::Empty,
    Tile::Empty,
    Tile::Falling,
    Tile::Empty,
    Tile::Empty,
    Tile::Empty,
    Tile::Empty,
]; 4];

const EMPTY: [[Tile; 7]; 3] = [[Tile::Empty; 7]; 3];

const HORIZONTAL: [[Tile; 7]; 1] = [[
    Tile::Empty,
    Tile::Empty,
    Tile::Falling,
    Tile::Falling,
    Tile::Falling,
    Tile::Falling,
    Tile::Empty,
]; 1];
const PLUS: [[Tile; 7]; 3] = [
    [
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Falling,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
    ],
    [
        Tile::Empty,
        Tile::Empty,
        Tile::Falling,
        Tile::Falling,
        Tile::Falling,
        Tile::Empty,
        Tile::Empty,
    ],
    [
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Falling,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
    ],
];
const ANGLE: [[Tile; 7]; 3] = [
    [
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Falling,
        Tile::Empty,
        Tile::Empty,
    ],
    [
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Falling,
        Tile::Empty,
        Tile::Empty,
    ],
    [
        Tile::Empty,
        Tile::Empty,
        Tile::Falling,
        Tile::Falling,
        Tile::Falling,
        Tile::Empty,
        Tile::Empty,
    ],
];
const SQUARE: [[Tile; 7]; 2] = [
    [
        Tile::Empty,
        Tile::Empty,
        Tile::Falling,
        Tile::Falling,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
    ],
    [
        Tile::Empty,
        Tile::Empty,
        Tile::Falling,
        Tile::Falling,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
    ],
];
