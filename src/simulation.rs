use crate::forest::Forest;
use crate::tree::Cell;
use raylib::prelude::*;
use std::time::{Duration, Instant};

const WIDTH: usize = 20;
const HEIGHT: usize = 20;
const CELL_SIZE: i32 = 20;
const INFO_WIDTH: i32 = 300; // Szerokość panelu informacyjnego

pub fn run_simulation() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, thread) = raylib::init()
        .size(
            (WIDTH as i32 * CELL_SIZE + INFO_WIDTH) as i32,
            (HEIGHT as i32 * CELL_SIZE) as i32,
        )
        .title("Forest Fire Simulation")
        .build();

    let mut forest = Forest::new(WIDTH, HEIGHT, 0.6);
    let mut current_day = "Monday".to_string();
    let mut current_hour = 0;
    let mut hours_since_last_update = 0;
    let mut last_update = Instant::now();
    let update_interval = Duration::from_secs(1);
    let mut selected_tree_coords: Option<(usize, usize)> = None;

    loop {
        if last_update.elapsed() >= update_interval {
            current_hour += 1;
            hours_since_last_update += 1;
            if current_hour >= 24 {
                current_hour = 0;
                // Przykładowe przechodzenie do kolejnego dnia
                current_day = match current_day.as_str() {
                    "Monday" => "Tuesday".to_string(),
                    "Tuesday" => "Wednesday".to_string(),
                    "Wednesday" => "Thursday".to_string(),
                    "Thursday" => "Friday".to_string(),
                    "Friday" => "Saturday".to_string(),
                    "Saturday" => "Sunday".to_string(),
                    _ => "Monday".to_string(),
                };
            }

            if hours_since_last_update >= 3 {
                forest.update_rain_probability();
                hours_since_last_update = 0;
            }

            forest.attempt_ignition();
            forest.update();

            last_update = Instant::now();
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_position = rl.get_mouse_position();
            let x = (mouse_position.x / CELL_SIZE as f32) as usize;
            let y = (mouse_position.y / CELL_SIZE as f32) as usize;

            if x < WIDTH && y < HEIGHT {
                if let Cell::Tree(_) = forest.cells[y][x] {
                    selected_tree_coords = Some((x, y));
                } else {
                    selected_tree_coords = None;
                }
            }
        }

        let selected_tree = selected_tree_coords.and_then(|(x, y)| {
            if let Cell::Tree(tree) = forest.cells[y][x] {
                Some(tree)
            } else {
                None
            }
        });

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let color = match forest.cells[y][x] {
                        Cell::Empty => Color::WHITE,
                        Cell::Tree(tree) => {
                            if tree.burnt {
                                Color::BLACK
                            } else if tree.burning {
                                Color::RED
                            } else {
                                Color::GREEN
                            }
                        }
                    };
                    d.draw_rectangle(
                        (x as i32 * CELL_SIZE),
                        (y as i32 * CELL_SIZE),
                        CELL_SIZE,
                        CELL_SIZE,
                        color,
                    );
                    d.draw_rectangle_lines(
                        (x as i32 * CELL_SIZE),
                        (y as i32 * CELL_SIZE),
                        CELL_SIZE,
                        CELL_SIZE,
                        Color::GRAY,
                    );
                }
            }

            let info_x = (WIDTH as i32 * CELL_SIZE) + 10;
            let mut info_y = 10;
            let line_height = 20;

            d.draw_text("Legend:", info_x, info_y, 20, Color::BLACK);
            info_y += line_height;
            d.draw_text("Empty: WHITE", info_x, info_y, 20, Color::BLACK);
            info_y += line_height;
            d.draw_text("Tree: GREEN", info_x, info_y, 20, Color::BLACK);
            info_y += line_height;
            d.draw_text("Burning: RED", info_x, info_y, 20, Color::BLACK);
            info_y += line_height;
            d.draw_text("Burnt: BLACK", info_x, info_y, 20, Color::BLACK);
            info_y += line_height * 2;
            d.draw_text(
                &format!("Day: {}", current_day),
                info_x,
                info_y,
                20,
                Color::BLACK,
            );
            info_y += line_height;
            d.draw_text(
                &format!("Hour: {}", current_hour),
                info_x,
                info_y,
                20,
                Color::BLACK,
            );
            info_y += line_height;
            d.draw_text(
                &format!("Humidity: {}", forest.humidity),
                info_x,
                info_y,
                20,
                Color::BLACK,
            );
            info_y += line_height;
            d.draw_text(
                &format!("Rain Probability: {:.2}%", forest.rain_probability * 100.0),
                info_x,
                info_y,
                20,
                Color::BLACK,
            );
            info_y += line_height;
            d.draw_text(
                &format!("Current Rain: {:.2}", forest.current_rain),
                info_x,
                info_y,
                20,
                Color::BLACK,
            );
            info_y += line_height;
            d.draw_text(
                &format!("Rain Intensity: {}", forest.rain_intensity),
                info_x,
                info_y,
                20,
                Color::BLACK,
            );

            if let Some(tree) = selected_tree {
                info_y += line_height * 2;
                d.draw_text(
                    &format!("Selected Tree Health: {}", tree.health),
                    info_x,
                    info_y,
                    20,
                    Color::BLACK,
                );
                info_y += line_height;
                d.draw_text(
                    &format!("Burning: {}", tree.burning),
                    info_x,
                    info_y,
                    20,
                    Color::BLACK,
                );
                info_y += line_height;
                d.draw_text(
                    &format!("Burnt: {}", tree.burnt),
                    info_x,
                    info_y,
                    20,
                    Color::BLACK,
                );
            }
        }
    }
}
