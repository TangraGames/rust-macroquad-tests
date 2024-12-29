use macroquad::{
    color::*,
    shapes::draw_circle,
    window::{clear_background, next_frame, Conf},
};

mod poisson_disk_sampling;

const WINDOW_WIDTH:i32 = 800;
const WINDOW_HEIGHT:i32 = 600;

fn draw_points(points: &Vec<(f32, f32)>, min_radius: f32) {
    let radius_color:Color = Color::new(0.9, 0.16, 0.22, 0.7);

    for point in points.iter() {
	// draw circle around each point with half radius
        draw_circle(point.0, point.1, min_radius/2.0, radius_color);
	// draw circle on point
        draw_circle(point.0, point.1, 2.0, LIGHTGRAY);
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Poisson Disk Sampling".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        high_dpi: true,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let min_radius: f32 = 25.0f32;
    let region: (f32, f32) = (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let points:Vec<(f32, f32)> = poisson_disk_sampling::generate_points(min_radius, region);

    loop {
        clear_background(BLACK);
        draw_points(&points, min_radius);
        next_frame().await
    }
}
