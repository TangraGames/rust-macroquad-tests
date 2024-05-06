use macroquad::{
    color::*,
    shapes::draw_circle,
    window::{clear_background, next_frame, Conf},
};

mod poisson_disk_sampling;

fn window_conf() -> Conf {
    Conf {
        window_title: "Poisson Disk Sampling".to_owned(),
        window_width: 600,
        window_height: 400,
        high_dpi: true,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
	let min_radius: f32 = 25.0f32;
    let region = (800.0f32, 600.0f32);
    let points:Vec<(f32, f32)> = poisson_disk_sampling::generate_points(min_radius, region);

    loop {
        clear_background(BLACK);
        let radius_color:Color = Color::new(0.9, 0.16, 0.22, 0.7);

        for point in points.iter() {
            draw_circle(point.0, point.1, min_radius/2.0, radius_color);
            draw_circle(point.0, point.1, 4.0, LIGHTGRAY);
        }      
        next_frame().await
    }
}