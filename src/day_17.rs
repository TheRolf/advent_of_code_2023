#![allow(dead_code, unused_variables, unused_mut)]

use pathfinding::prelude::dijkstra;

use advent_of_code_2023::*;

struct CityMap {
    heat_loss: Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

impl CityMap {
    pub fn new(input: &Vec<Vec<char>>) -> Self {
        let height = input.len();
        let width = input[0].len();
        let heat_loss = input
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        Self {
            heat_loss,
            height,
            width,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Block {
    i: usize,
    j: usize,
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

impl Block {
    pub fn successors_a(&self, city_map: &CityMap) -> Vec<(Block, usize)> {
        let mut successors = Vec::new();

        // bottom-right corner should lead to a dummy node with cost 0
        if self.i == city_map.height - 1 && self.j == city_map.width - 1 {
            successors.push((
                Block {
                    i: self.i + 1,
                    j: self.j + 1,
                    left: 0,
                    right: 0,
                    up: 0,
                    down: 0,
                },
                0,
            ));
        }

        // UP movement
        if self.i > 0 && self.up < 3 && self.down == 0 {
            successors.push((
                Block {
                    i: self.i - 1,
                    j: self.j,
                    left: 0,
                    right: 0,
                    up: self.up + 1,
                    down: 0,
                },
                city_map.heat_loss[self.i - 1][self.j],
            ));
        }

        // DOWN movement
        if self.i < city_map.height - 1 && self.down < 3 && self.up == 0 {
            successors.push((
                Block {
                    i: self.i + 1,
                    j: self.j,
                    left: 0,
                    right: 0,
                    up: 0,
                    down: self.down + 1,
                },
                city_map.heat_loss[self.i + 1][self.j],
            ));
        }

        // LEFT movement
        if self.j > 0 && self.left < 3 && self.right == 0 {
            successors.push((
                Block {
                    i: self.i,
                    j: self.j - 1,
                    left: self.left + 1,
                    right: 0,
                    up: 0,
                    down: 0,
                },
                city_map.heat_loss[self.i][self.j - 1],
            ));
        }

        // RIGHT movement
        if self.j < city_map.width - 1 && self.right < 3 && self.left == 0 {
            successors.push((
                Block {
                    i: self.i,
                    j: self.j + 1,
                    left: 0,
                    right: self.right + 1,
                    up: 0,
                    down: 0,
                },
                city_map.heat_loss[self.i][self.j + 1],
            ));
        }
        successors
    }

    pub fn successors_b(&self, city_map: &CityMap) -> Vec<(Block, usize)> {
        let mut successors = Vec::new();

        // bottom-right corner should lead to a dummy node with cost 0
        if self.i == city_map.height - 1
            && self.j == city_map.width - 1
            && (self.left >= 4 || self.right >= 4 || self.up >= 4 || self.down >= 4)
        {
            successors.push((
                Block {
                    i: self.i + 1,
                    j: self.j + 1,
                    left: 0,
                    right: 0,
                    up: 0,
                    down: 0,
                },
                0,
            ));
        }

        // UP movement
        if self.i > 0
            && (self.left >= 4 || self.right >= 4 || self.up > 0)
            && self.up < 10
            && self.down == 0
        {
            successors.push((
                Block {
                    i: self.i - 1,
                    j: self.j,
                    left: 0,
                    right: 0,
                    up: self.up + 1,
                    down: 0,
                },
                city_map.heat_loss[self.i - 1][self.j],
            ));
        }

        // DOWN movement
        if (self.i == 0 && self.j == 0) // starting location
            || (self.i < city_map.height - 1
                && (self.left >= 4 || self.right >= 4 || self.down > 0)
                && self.down < 10
                && self.up == 0)
        {
            successors.push((
                Block {
                    i: self.i + 1,
                    j: self.j,
                    left: 0,
                    right: 0,
                    up: 0,
                    down: self.down + 1,
                },
                city_map.heat_loss[self.i + 1][self.j],
            ));
        }

        // LEFT movement
        if self.j > 0
            && (self.up >= 4 || self.down >= 4 || self.left > 0)
            && self.left < 10
            && self.right == 0
        {
            successors.push((
                Block {
                    i: self.i,
                    j: self.j - 1,
                    left: self.left + 1,
                    right: 0,
                    up: 0,
                    down: 0,
                },
                city_map.heat_loss[self.i][self.j - 1],
            ));
        }

        // RIGHT movement
        if (self.i == 0 && self.j == 0) // starting location
            || (self.j < city_map.width - 1
                && (self.up >= 4 || self.down >= 4 || self.right > 0)
                && self.right < 10
                && self.left == 0)
        {
            successors.push((
                Block {
                    i: self.i,
                    j: self.j + 1,
                    left: 0,
                    right: self.right + 1,
                    up: 0,
                    down: 0,
                },
                city_map.heat_loss[self.i][self.j + 1],
            ));
        }
        successors
    }
}

pub fn main() {
    let input = puzzle_input_asarray(17, false);
    let city_map = CityMap::new(&input);
    let goal: Block = Block {
        i: city_map.height,
        j: city_map.width,
        left: 0,
        right: 0,
        up: 0,
        down: 0,
    };
    let result_a = dijkstra(
        &Block {
            i: 0,
            j: 0,
            left: 0,
            right: 0,
            up: 0,
            down: 0,
        },
        |b| b.successors_a(&city_map),
        |b| *b == goal,
    )
    .unwrap();
    println!("{}", result_a.1);

    let result_b = dijkstra(
        &Block {
            i: 0,
            j: 0,
            left: 0,
            right: 0,
            up: 0,
            down: 0,
        },
        |b| b.successors_b(&city_map),
        |b| *b == goal,
    )
    .unwrap();
    println!("{}", result_b.1);
}
