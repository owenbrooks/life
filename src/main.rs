use nannou::{color, math::cgmath::Vector2, prelude::*};

const WIDTH_COUNT: usize = 50;
const TOTAL_WIDTH: f32 = 500.;
const GRID_SQUARE_WIDTH: f32 = TOTAL_WIDTH/WIDTH_COUNT as f32;

struct Model {
    grid: [bool; WIDTH_COUNT*WIDTH_COUNT],
    mouse_pos: Vector2<f32>,
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
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
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
                    model.grid[index] = !model.grid[index];
                },
                None => {},
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


    for i in 0..WIDTH_COUNT {
        for j in 0..WIDTH_COUNT {
            // let is_white = (i%2!=0) && (j%2==0) || (i%2==0) && (j%2!=0);
            let is_white = model.grid[(j*WIDTH_COUNT + i) as usize];
            if is_white {
                draw.rect()
                    .x_y(i as f32 * GRID_SQUARE_WIDTH - TOTAL_WIDTH/2., j as f32 * (-GRID_SQUARE_WIDTH) + TOTAL_WIDTH/2.)
                    .w_h(GRID_SQUARE_WIDTH, GRID_SQUARE_WIDTH)
                    .color(WHITE);
            } 
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn pos_to_index(pos: Vector2<f32>, square_width: f32, square_count: usize) -> Option<usize> {
    let max_x = square_width*(square_count+1) as f32 / 2.;
    let max_y = max_x;

    if pos.x > max_x || pos.x < -max_x || pos.y > max_y || pos.y < -max_y {
        return None
    }

    let x_ind = ((square_count / 2) as f32 + pos.x / square_width).round() as usize;
    let y_ind = ((square_count / 2) as f32 - pos.y / square_width).round() as usize;
    Some(y_ind * square_count + x_ind)
}