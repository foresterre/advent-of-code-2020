use aoc2020::{read_file, TResult};
use std::ops::Mul;

#[derive(Default, Debug)]
struct Sled {
    position: Position,
    arrived: bool,
}

impl Sled {
    fn slide(&mut self, slope_offset: Offset) -> Position {
        self.position.slide(slope_offset);
        self.position
    }

    fn update_has_arrived(&mut self, area: &BiomeStableArea) {
        if self.position.at_destination(area) {
            self.arrived = true;
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn at_destination(&self, area: &BiomeStableArea) -> bool {
        self.y >= area.height()
    }

    fn slide(&mut self, slope_offset: Offset) {
        self.x = self.x + slope_offset.right;
        self.y = self.y + slope_offset.down;
    }
}

#[derive(Debug, Copy, Clone)]
struct Offset {
    right: usize,
    down: usize,
}

impl Offset {
    fn new(right: usize, down: usize) -> Self {
        Self { right, down }
    }
}

#[derive(Debug)]
struct BiomeStableArea {
    // Stores y by x tiles
    map: Vec<Vec<Tile>>,
}

impl BiomeStableArea {
    fn tile_at(&self, position: Position) -> &Tile {
        let y = position.y % self.map.len();
        let x = position.x % self.map[y].len();

        &self.map[y][x]
    }

    fn height(&self) -> usize {
        self.map.len()
    }
}

impl<'s> From<&'s str> for BiomeStableArea {
    fn from(string: &'s str) -> Self {
        let map = string
            .lines()
            .map(|s| s.chars().map(Into::<Tile>::into).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { map }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Tree,
    Square,
}

impl From<char> for Tile {
    fn from(tile: char) -> Self {
        match tile {
            '.' => Self::Square,
            '#' => Self::Tree,
            c => unreachable!("The advent of code input is nice ^^, {}", c),
        }
    }
}

impl Tile {
    fn is_tree(&self) -> bool {
        self == &Self::Tree
    }
}

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input");
    let input = read_file(path).expect("Unable to read file");

    println!("exercise 1: {}", part1(&input).unwrap());
    println!("exercise 2: {}", part2(&input).unwrap());
}

fn detected_trees_for_slopes<'iter, I: Iterator<Item = &'iter Offset> + 'iter>(
    area: &'iter BiomeStableArea,
    offsets: I,
) -> impl Iterator<Item = usize> + 'iter {
    offsets.map(move |offset| detected_trees(area, *offset))
}

fn detected_trees(area: &BiomeStableArea, offset: Offset) -> usize {
    std::iter::repeat(offset)
        .scan(Sled::default(), |sled, slope| {
            let new_position = sled.slide(slope);

            let trees = if area.tile_at(new_position).is_tree() {
                1
            } else {
                0
            };

            sled.update_has_arrived(&area);

            if sled.arrived {
                None
            } else {
                Some(trees)
            }
        })
        .sum()
}

fn part1(input: &str) -> TResult<usize> {
    let area = BiomeStableArea::from(input);

    let sum = detected_trees(&area, Offset::new(3, 1));

    Ok(sum)
}

fn part2(input: &str) -> TResult<usize> {
    let area = BiomeStableArea::from(input);
    let offsets = &[
        Offset::new(1, 1),
        Offset::new(3, 1),
        Offset::new(5, 1),
        Offset::new(7, 1),
        Offset::new(1, 2),
    ];

    let trees = detected_trees_for_slopes(&area, offsets.iter()).fold(1usize, Mul::mul);

    Ok(trees)
}
