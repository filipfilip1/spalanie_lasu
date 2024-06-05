use crate::tree::{Cell, TreeState};
use rand::Rng;

pub struct Forest {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub humidity: u32,
    pub rain_probability: f64,
    pub current_rain: f64,
    pub rain_intensity: u8,
}

impl Forest {
    pub fn new(width: usize, height: usize, tree_density: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = vec![vec![Cell::Empty; width]; height];

        for y in 0..height {
            for x in 0..width {
                if rng.gen::<f64>() < tree_density {
                    cells[y][x] = Cell::Tree(TreeState::new());
                }
            }
        }

        Forest {
            width,
            height,
            cells,
            humidity: 50,
            rain_probability: 0.5,
            current_rain: 0.0,
            rain_intensity: 0,
        }
    }

    pub fn ignite(&mut self, x: usize, y: usize) {
        if let Cell::Tree(tree) = &mut self.cells[y][x] {
            if !tree.burning && !tree.burnt {
                tree.burning = true;
            }
        }
    }

    pub fn spread_fire(&mut self) {
        if self.rain_intensity > 0 {
            return;
        }

        let mut new_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                if let Cell::Tree(tree) = &self.cells[y][x] {
                    if tree.burning {
                        let mut burning_neighbors = 0;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let ny = y as isize + dy;
                                let nx = x as isize + dx;

                                if ny >= 0
                                    && ny < self.height as isize
                                    && nx >= 0
                                    && nx < self.width as isize
                                {
                                    let ny = ny as usize;
                                    let nx = nx as usize;

                                    if let Cell::Tree(neighbor_tree) = &mut new_cells[ny][nx] {
                                        if neighbor_tree.burning {
                                            burning_neighbors += 1;
                                        } else if !neighbor_tree.burnt {
                                            neighbor_tree.burning = true;
                                        }
                                    }
                                }
                            }
                        }

                        let mut new_tree = tree.clone();
                        new_tree.health = new_tree.health.saturating_sub(1 + burning_neighbors);
                        if new_tree.health == 0 {
                            new_tree.burnt = true;
                            new_tree.burning = false;
                        }
                        new_cells[y][x] = Cell::Tree(new_tree);
                    }
                }
            }
        }

        self.cells = new_cells;
    }

    pub fn update(&mut self) {
        self.spread_fire();
        self.update_rain();
    }

    pub fn attempt_ignition(&mut self) {
        if self.rain_intensity > 0 {
            return;
        }

        let probability = (100 - self.humidity) as f64 / 100.0;
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < probability {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            self.ignite(x, y);
        }
    }

    pub fn update_rain(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.rain_probability {
            let rain_type = rng.gen::<f64>();
            let rain_intensity = if rain_type < 0.6 {
                1 // Lekki deszcz
            } else if rain_type < 0.9 {
                2 // Średni deszcz
            } else {
                3 // Ulewa
            };
            self.humidity = (self.humidity + (rain_intensity as u32)).min(100);
            self.current_rain = rain_intensity as f64;
            self.rain_intensity = rain_intensity;
        } else {
            self.current_rain = 0.0;
            self.rain_intensity = 0;
        }
    }

    pub fn update_rain_probability(&mut self) {
        let mut rng = rand::thread_rng();
        let probability_range = rng.gen::<f64>();

        self.rain_probability = if probability_range < 0.6 {
            rng.gen_range(0.0..0.1)
        } else if probability_range < 0.8 {
            rng.gen_range(0.11..0.3)
        } else if probability_range < 0.9 {
            rng.gen_range(0.31..0.6)
        } else {
            rng.gen_range(0.61..0.99)
        };
    }
}
