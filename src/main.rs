use nannou::{color, math::cgmath::Vector2, prelude::*};
use std::time::Duration;

const WIDTH_COUNT: usize = 100;
const TOTAL_WIDTH: f32 = 1000.;
const GRID_SQUARE_WIDTH: f32 = TOTAL_WIDTH/WIDTH_COUNT as f32;

struct Model {
    grid: [bool; WIDTH_COUNT*WIDTH_COUNT],
    mouse_pos: Vector2<f32>,
    time_since_last_tick: std::time::Duration,
    paused: bool,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Life")
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model {
        grid: [false; WIDTH_COUNT*WIDTH_COUNT],
        mouse_pos: Vector2 {x: 0., y: 0.},
        time_since_last_tick: Duration::new(0, 0),
        paused: true,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.time_since_last_tick += update.since_last;
    if model.time_since_last_tick >= Duration::new(0, 1000) && !model.paused {
        model.grid = update_grid(model.grid);
        model.time_since_last_tick = Duration::new(0, 0);
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MouseMoved(pos) => {
            model.mouse_pos = Vector2 {x: pos.x, y: pos.y};
        },
        WindowEvent::MousePressed(_) => {
            let clicked_index = pos_to_index(model.mouse_pos, GRID_SQUARE_WIDTH, WIDTH_COUNT);
            match clicked_index {
                Some(index) => {
                    // println!("{}", index);
                    // println!("Pos: {:?}", model.mouse_pos);
                    // println!("{} has {} alive neighbours.", index, cell_neighbours(index).iter().filter(|i| model.grid[**i]).count());
                    model.grid[index] = !model.grid[index];
                },
                None => {},
            }
        },
        WindowEvent::KeyPressed(key) => {
            match key {
                Key::P => {model.paused = !model.paused},
                _ => {},
            }
        }
        _other => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background 
    let background_colour = color::hsv(0.0, 0.0, 0.03);
    draw.background().color(background_colour);

    // draw.rect().w_h(TOTAL_WIDTH, TOTAL_WIDTH).color(LIGHTGRAY);


    for i in 0..WIDTH_COUNT {
        for j in 0..WIDTH_COUNT {
            // let is_white = (i%2!=0) && (j%2==0) || (i%2==0) && (j%2!=0);
            let is_white = model.grid[(j*WIDTH_COUNT + i) as usize];
            if is_white {
                draw.rect()
                    .x_y(i as f32 * GRID_SQUARE_WIDTH - TOTAL_WIDTH/2. + GRID_SQUARE_WIDTH/2., j as f32 * (-GRID_SQUARE_WIDTH) + TOTAL_WIDTH/2. - GRID_SQUARE_WIDTH / 2.)
                    .w_h(GRID_SQUARE_WIDTH, GRID_SQUARE_WIDTH)
                    .color(WHITE);
            } 
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn pos_to_index(pos: Vector2<f32>, square_width: f32, square_count: usize) -> Option<usize> {
    let max_x = TOTAL_WIDTH / 2.;
    let max_y = max_x;

    if pos.x >= max_x || pos.x <= -max_x || pos.y >= max_y || pos.y <= -max_y {
        return None
    }

    let grid_pos = pos + Vector2{x: max_x, y: -max_y};
    let x_ind = (grid_pos.x / square_width).floor() as usize;
    let y_ind = (-grid_pos.y / square_width).floor() as usize;
    Some(y_ind * square_count + x_ind)
}

fn update_grid(curr_grid: [bool; WIDTH_COUNT*WIDTH_COUNT]) -> [bool; WIDTH_COUNT*WIDTH_COUNT] {
    let mut new_grid = curr_grid.clone();
    for index in 0..curr_grid.len() {
        let alive_neighbours = cell_neighbours(index).iter().filter(|i| curr_grid[**i]).count();
        if alive_neighbours == 3 {
            new_grid[index] = true;  
        } else if alive_neighbours < 2 || alive_neighbours > 3 {
            new_grid[index] = false;
        } 
    }
    new_grid
}

fn cell_neighbours(index: usize) -> Vec<usize> {
    let mut neighbours: Vec<usize> = vec![];
    let x = index % WIDTH_COUNT;
    let y = index / WIDTH_COUNT;

    let has_top = y >= 1;
    let has_left = x >= 1;
    let has_bottom = y < WIDTH_COUNT - 1;
    let has_right = x < WIDTH_COUNT - 1;

    if has_left {
        let middle_left = index - 1;
        neighbours.push(middle_left);

        if has_top {
            let top_left = index - WIDTH_COUNT - 1;
            neighbours.push(top_left);
        }
        if has_bottom {
            let bottom_left = index + WIDTH_COUNT - 1;
            neighbours.push(bottom_left);
        }
    }
    if has_right {
        let middle_right = index + 1;
        neighbours.push(middle_right);

        if has_top {
            let top_right = index - WIDTH_COUNT + 1;
            neighbours.push(top_right);
        }
        if has_bottom {
            let bottom_right = index + WIDTH_COUNT + 1;
            neighbours.push(bottom_right);
        }
    }
    if has_top {
        let top_middle = index - WIDTH_COUNT;
        neighbours.push(top_middle);
    }
    if has_bottom {
        let bottom_middle = index + WIDTH_COUNT;
        neighbours.push(bottom_middle);
    }

    neighbours
}