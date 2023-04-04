use nannou::prelude::*;

const ROWS: usize = 4;
const COLUMNS: usize = 8;
const SCALE: f32 = 1.8;
const CIRCLE_RADIUS: f32 = 15.0 * SCALE;
const CIRCLE_SPACING: f32 = 15.0 * SCALE;
const ANIMATION_SPEED: f32 = 0.4;
const CIRCLE_INTERVAL: f32 = 0.35;
const COLOR_SPEED_MULTIPLIER: f32 = 300.0;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let grid_width = COLUMNS as f32 * (CIRCLE_RADIUS * 2.0 + CIRCLE_SPACING) - CIRCLE_SPACING;
    let grid_height = ROWS as f32 * (CIRCLE_RADIUS * 2.0 + CIRCLE_SPACING) - CIRCLE_SPACING;

    let window_center = Vec2::new(app.window_rect().w() / 2.0, app.window_rect().h() / 2.0);
    let start_x = window_center.x - (grid_width / 2.0) - app.window_rect().w() / 2.0;
    let start_y = window_center.y - (grid_height / 2.0) - app.window_rect().h() / 2.0;

    let time = app.time;
    let circle_interval = CIRCLE_INTERVAL;

    let hue = time * COLOR_SPEED_MULTIPLIER % 360.0;
    let color = Hsv::new(hue, 1.0, 1.0);

    let total_drop_time = (ROWS * COLUMNS) as f32 * circle_interval;
    let loop_time = time % total_drop_time;

    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let x = start_x + column as f32 * (CIRCLE_RADIUS * 2.0 + CIRCLE_SPACING);
            let y = start_y + row as f32 * (CIRCLE_RADIUS * 2.0 + CIRCLE_SPACING);

            let mut column_order = column;
            if row % 2 == 1 {
                column_order = COLUMNS - 1 - column;
            }

            let circle_time_offset = (row * COLUMNS + column_order) as f32 * circle_interval;
            let mut new_y = y;
            if loop_time > circle_time_offset {
                let progress =
                    (((loop_time - circle_time_offset) / ANIMATION_SPEED).min(1.0)).max(0.0);
                new_y -= (CIRCLE_RADIUS * 2.0 + CIRCLE_SPACING) * progress;
            }

            let upward_movement =
                (loop_time / total_drop_time).min(1.0) * (CIRCLE_RADIUS * 2.0 + CIRCLE_SPACING);
            new_y += upward_movement;

            draw.ellipse()
                .x_y(x, new_y)
                .radius(CIRCLE_RADIUS)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
