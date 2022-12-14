use itertools::Itertools;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Sand,
    Rock,
}
#[derive(Debug)]
struct Cave {
    tiles: Vec<Vec<Tile>>,
}
impl Cave {
    fn from_paths(paths: &[Vec<(usize, usize)>]) -> Self {
        let mut tiles = Self {
            tiles: vec![vec![Tile::Empty; 1000]; 1000],
        };
        for path in paths {
            for ((x1, y1), (x2, y2)) in path.iter().copied().tuple_windows() {
                if x1 == x2 {
                    tiles.draw_line_vertical(x1, y1, y2);
                } else {
                    tiles.draw_line_horizontal(x1, x2, y1);
                }
            }
        }
        tiles
    }
    fn draw_line_horizontal(&mut self, mut x1: usize, mut x2: usize, y: usize) {
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }
        for x in x1..=x2 {
            self.tiles[x][y] = Tile::Rock;
        }
    }
    fn draw_line_vertical(&mut self, x: usize, mut y1: usize, mut y2: usize) {
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        }
        for y in y1..=y2 {
            self.tiles[x][y] = Tile::Rock;
        }
    }
    fn drop_sand(&mut self, mut x: usize, mut y: usize) -> bool {
        if self.tiles[x][y] == Tile::Sand {
            return true;
        }
        loop {
            y += 1;
            if let Some(tile) = self.tiles[x].get(y) {
                match tile {
                    Tile::Empty => continue,
                    _ => {
                        if matches!(self.tiles[x - 1][y], Tile::Empty) {
                            x -= 1;
                            continue;
                        }
                        if matches!(self.tiles[x + 1][y], Tile::Empty) {
                            x += 1;
                            continue;
                        }
                        self.tiles[x][y - 1] = Tile::Sand;
                        return false;
                    }
                }
            } else {
                return true;
            }
        }
    }
    #[allow(dead_code)]
    fn dbg(&self) {
        for row in &self.tiles {
            for tile in row {
                print!(
                    "{}",
                    match tile {
                        Tile::Empty => ' ',
                        Tile::Sand => '0',
                        Tile::Rock => '#',
                    }
                );
            }
            println!();
        }
    }
}

pub fn part1(input: &str) -> usize {
    let paths = parse_paths(input);
    let mut cave = Cave::from_paths(&paths);
    while !cave.drop_sand(500, 0) {}
    cave.tiles
        .iter()
        .flatten()
        .filter(|&&tile| tile == Tile::Sand)
        .count()
}

pub fn part2(input: &str) -> usize {
    let paths = parse_paths(input);
    let mut cave = Cave::from_paths(&paths);
    let mut last_row = 0;
    for y in (0..cave.tiles[0].len()).rev() {
        let mut found = false;
        for x in 0..cave.tiles.len() {
            if cave.tiles[x][y] == Tile::Rock {
                found = true;
                break;
            }
        }
        if found {
            last_row = y;
            break;
        }
    }
    cave.draw_line_horizontal(0, 999, last_row + 2);
    while !cave.drop_sand(500, 0) {}
    cave.tiles
        .iter()
        .flatten()
        .filter(|&&tile| tile == Tile::Sand)
        .count()
}

fn parse_paths(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            line.split("->")
                .map(|s| {
                    let mut it = s.split(',');
                    let x = it.next().unwrap().trim().parse().unwrap();
                    let y = it.next().unwrap().trim().parse().unwrap();
                    (x, y)
                })
                .collect()
        })
        .collect()
}
