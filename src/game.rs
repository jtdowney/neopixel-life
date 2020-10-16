use crate::{PIXELS, WIDTH};
use rand::prelude::*;

const INITIAL_STATES: [fn() -> Game; 5] = [
    Game::acorn,
    Game::f_heptomino,
    Game::glider,
    Game::pi_heptomino,
    Game::r_pentomino,
];

#[derive(PartialEq)]
pub struct Game {
    cells: [bool; PIXELS],
}

impl Default for Game {
    fn default() -> Self {
        Game {
            cells: [false; PIXELS],
        }
    }
}

impl Game {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let random_start = INITIAL_STATES.choose(rng).unwrap();
        random_start()
    }

    pub fn acorn() -> Game {
        let mut game = Game::default();
        game.set_alive(4, 6);
        game.set_alive(5, 6);
        game.set_alive(5, 8);
        game.set_alive(7, 7);
        game.set_alive(8, 6);
        game.set_alive(9, 6);
        game.set_alive(10, 6);
        game
    }

    pub fn r_pentomino() -> Game {
        let mut game = Game::default();
        game.set_alive(6, 7);
        game.set_alive(7, 7);
        game.set_alive(7, 6);
        game.set_alive(7, 8);
        game.set_alive(8, 8);
        game
    }

    pub fn pi_heptomino() -> Game {
        let mut game = Game::default();
        game.set_alive(6, 6);
        game.set_alive(6, 7);
        game.set_alive(6, 8);
        game.set_alive(7, 8);
        game.set_alive(8, 8);
        game.set_alive(8, 7);
        game.set_alive(8, 6);
        game
    }

    pub fn f_heptomino() -> Game {
        let mut game = Game::default();
        game.set_alive(6, 8);
        game.set_alive(7, 8);
        game.set_alive(7, 7);
        game.set_alive(7, 6);
        game.set_alive(7, 5);
        game.set_alive(8, 5);
        game.set_alive(9, 5);
        game
    }

    pub fn glider() -> Game {
        let mut game = Game::default();
        game.set_alive(1, 10);
        game.set_alive(2, 9);
        game.set_alive(3, 9);
        game.set_alive(3, 10);
        game.set_alive(3, 11);
        game
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.cells[WIDTH * y + x]
    }

    fn count_living_neighbors(&self, x: usize, y: usize) -> usize {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|&(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(|&(nx, ny)| nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < WIDTH as isize)
        .filter(|&(nx, ny)| self.is_alive(nx as usize, ny as usize))
        .count()
    }

    fn set_alive(&mut self, x: usize, y: usize) {
        let offset = WIDTH * y + x;
        self.cells[offset] = true;
    }
}

impl Iterator for Game {
    type Item = Game;

    fn next(&mut self) -> Option<Game> {
        let mut next = Game::default();
        for y in 0..WIDTH {
            for x in 0..WIDTH {
                let is_alive = self.is_alive(x, y);
                let living_neighbors = self.count_living_neighbors(x, y);
                match (is_alive, living_neighbors) {
                    (true, 2) | (_, 3) => next.set_alive(x, y),
                    _ => {}
                };
            }
        }

        Some(next)
    }
}
