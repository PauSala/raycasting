pub mod utils;
pub mod vector2;

use macroquad::prelude::*;
use utils::dda;
use utils::dda_grid;
use utils::draw_grid;
use utils::normalize;
use utils::pixel_index;
use utils::point_at_distance;
use utils::MAP;
use utils::ROW_SIZE;
use vector2::is_vertical_side;
use vector2::point_perpendicular;
use vector2::Vector2;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const MINIMAP_WIDTH: u32 = WIDTH / 4;
const MINIMAP_HEIGHT: u32 = HEIGHT / 4;

const VRANGE: i32 = 10;
const VFRACT: i32 = 20;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pixel Buffer Example".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(true);

    // Minimap
    let mut mm_buf = vec![0u8; (MINIMAP_WIDTH * MINIMAP_HEIGHT * 4) as usize];
    let mut mm_texture =
        Texture2D::from_rgba8(MINIMAP_WIDTH as u16, MINIMAP_HEIGHT as u16, &mm_buf);
    set_minimap_background(&mut mm_buf, &mut mm_texture);

    //Main scene
    let mut main_buf = vec![0u8; (WIDTH * HEIGHT * 4) as usize];
    let mut main_texture = Texture2D::from_rgba8(WIDTH as u16, HEIGHT as u16, &main_buf);
    set_background(&mut main_buf, &mut main_texture);

    //Player
    let mut p1 = Vector2::new(1.0, 1.0).to_pixel_coords();
    let mut p2 = point_at_distance(&p1, Vector2::new(20.0, 1.0).to_pixel_coords(), 20.0);

    loop {
        let dir = (p2 - p1).norm();
        let mut vx = 0.2;
        let vy = 0.4;

        if is_key_down(KeyCode::Space) {
            vx *= 2.0;
        }

        if is_key_down(KeyCode::Up) {
            p1.y += vx * dir.y;
            p1.x += vx * dir.x;
            p2.y += vx * dir.y;
            p2.x += vx * dir.x;
        }

        if is_key_down(KeyCode::Down) {
            p1.y -= vx * dir.y;
            p1.x -= vx * dir.x;
            p2.y -= vx * dir.y;
            p2.x -= vx * dir.x;
        }

        if is_key_down(KeyCode::Right) {
            let left_dir = Vector2::new(-dir.y, dir.x);
            p2.y += vy * left_dir.y;
            p2.x += vy * left_dir.x;
        }

        if is_key_down(KeyCode::Left) {
            let right_dir = Vector2::new(dir.y, -dir.x);
            p2.y += 0.4 * right_dir.y;
            p2.x += 0.4 * right_dir.x;
        }

        // Render
        let intersections = intersections(p1, p2);
        render_main(&mut main_buf, &mut main_texture, p1, p2, &intersections);
        render_mini_map(p1, &mm_texture, &intersections);

        // FPS
        let fps = get_fps();
        draw_text(&format!("FPS: {}", fps), 700.0, 15.0, 20.0, WHITE);

        next_frame().await
    }
}

fn render_main(
    buf: &mut Vec<u8>,
    texture: &mut Texture2D,
    p1: Vector2,
    p2: Vector2,
    int: &Vec<(bool, Vector2, f32)>,
) {
    set_background(buf, texture);
    grid_to_world(p1, p2, buf, texture);
    main_scene(buf, texture, &int, p1);
    draw_texture_ex(
        &texture,
        0.0,
        0.0,
        Color {
            r: 1.0,
            g: 0.7,
            b: 1.0,
            a: 1.0,
        },
        DrawTextureParams {
            dest_size: Some(Vec2::new(WIDTH as f32, HEIGHT as f32)),
            ..Default::default()
        },
    );
}

fn main_scene(
    buf: &mut Vec<u8>,
    texture: &mut Texture2D,
    intersect: &Vec<(bool, Vector2, f32)>,
    p1: Vector2,
) {
    let slice_width = WIDTH as usize / intersect.len() as usize;
    for (i, int) in intersect.iter().enumerate() {
        let dist = (int.1 - p1).magnitude();

        let h = HEIGHT as f32;
        let line_height = h / int.2;
        let mut draw_start = -line_height / 2.0 + h / 2.0;
        if draw_start < 0.0 {
            draw_start = 0.0
        };
        let mut draw_end = line_height / 2.0 + h / 2.0;
        if draw_end >= h {
            draw_end = h - 1.0
        };

        if int.0 {
            for y in draw_start as u32..draw_end as u32 {
                for x in i * slice_width..i * slice_width + slice_width {
                    let index = pixel_index(x as u32, y, WIDTH);
                    if is_vertical_side(int.1, ROW_SIZE as f32) {
                        buf[index] = 190;
                        buf[index + 1] = 200;
                        buf[index + 2] = 240;
                        buf[index + 3] = 255 - (dist as u8).min(240);
                    } else {
                        buf[index] = 170;
                        buf[index + 1] = 190;
                        buf[index + 2] = 220;
                        buf[index + 3] = 255 - (dist as u8).min(240);
                    }
                }
            }
        }
    }

    texture.update_from_bytes(WIDTH, HEIGHT, &buf);
}

fn grid_to_world(p1: Vector2, p2: Vector2, buffer: &mut [u8], texture: &Texture2D) {
    let g_int = grid_intersections(p1, p2);
    let slice_width = WIDTH as usize / g_int.len() as usize;
    for (i, v) in g_int.iter().enumerate() {
        for (_, vh) in v {
            let h = HEIGHT as f32;
            let line_height = h / vh;
            let mut draw_start = -line_height / 2.0 + h / 2.0;
            if draw_start < 0.0 {
                draw_start = 0.0
            };
            let mut draw_end = line_height / 2.0 + h / 2.0;
            if draw_end >= h {
                draw_end = h - 1.0
            };

            for x in i * slice_width..i * slice_width + slice_width {
                let index1 = pixel_index(x as u32, draw_end as u32, WIDTH);
                buffer[index1] = 200;
                buffer[index1 + 1] = 200 as u8;
                buffer[index1 + 2] = 200;
                buffer[index1 + 3] = 255 - normalize(h - draw_end, 0.0, h);

                let index2 = pixel_index(x as u32, draw_start as u32, WIDTH);
                buffer[index2] = 200;
                buffer[index2 + 1] = 200 as u8;
                buffer[index2 + 2] = 200;
                buffer[index2 + 3] = 255 - normalize(h - draw_end, 0.0, h);
            }
        }
    }
    texture.update_from_bytes(WIDTH, HEIGHT, &buffer);
}

fn set_background(buffer: &mut [u8], texture: &mut Texture2D) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = pixel_index(x, y, WIDTH);
            if y < HEIGHT / 2 {
                buffer[index + 3] = ((HEIGHT - y) * 255 / HEIGHT) as u8;
            } else {
                buffer[index + 3] = (y * 255 / HEIGHT) as u8;
            }
            buffer[index] = 200;
            buffer[index + 1] = 220;
            buffer[index + 2] = 250;
        }
    }
    // Update the texture with new pixel data
    texture.update_from_bytes(WIDTH, HEIGHT, &buffer);
}

fn render_mini_map(p1: Vector2, texture: &Texture2D, intersections: &Vec<(bool, Vector2, f32)>) {
    draw_texture_ex(
        &texture,
        0.0,
        0.0,
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        DrawTextureParams {
            dest_size: Some(Vec2::new(MINIMAP_WIDTH as f32, MINIMAP_HEIGHT as f32)),
            ..Default::default()
        },
    );

    draw_grid();

    for int in intersections {
        draw_line(
            p1.x,
            p1.y,
            int.1.x,
            int.1.y,
            1.0,
            Color {
                r: 0.9607843,
                g: 0.7411765,
                b: 0.9019608,
                a: 0.1,
            },
        );
    }

    //Player
    draw_circle(p1.x, p1.y, 3.0, MAGENTA);
}

fn intersections(p1: Vector2, p2: Vector2) -> Vec<(bool, Vector2, f32)> {
    let mut res = vec![];
    let mut i = -VRANGE as f32;
    while i < VRANGE as f32 {
        let perp = point_perpendicular(p1, p2, i);
        res.push(dda(p1.to_grid_coords(), perp.to_grid_coords()));
        i += 1.0 / VFRACT as f32;
    }
    res
}

fn grid_intersections(p1: Vector2, p2: Vector2) -> Vec<Vec<(Vector2, f32)>> {
    let mut res = vec![];
    let mut i = -VRANGE as f32;
    while i < VRANGE as f32 {
        let perp = point_perpendicular(p1, p2, i);
        res.push(dda_grid(p1.to_grid_coords(), perp.to_grid_coords()));
        i += 1.0 / VFRACT as f32;
    }
    res
}

fn set_minimap_background(buffer: &mut [u8], texture: &mut Texture2D) {
    for y in 0..MINIMAP_HEIGHT {
        for x in 0..MINIMAP_WIDTH {
            let index = pixel_index(x, y, MINIMAP_WIDTH);
            let grid_pos = Vector2::new(x as f32, y as f32).to_grid_coords();
            if MAP[grid_pos.y.trunc() as usize][grid_pos.x.trunc() as usize] == 1 {
                buffer[index] = 82;
                buffer[index + 1] = 42 as u8;
                buffer[index + 2] = 82;
                buffer[index + 3] = 255;
            } else {
                buffer[index] = 42;
                buffer[index + 1] = 42 as u8;
                buffer[index + 2] = 43;
                buffer[index + 3] = 255;
            }
        }
    }
    // Update the texture with new pixel data
    texture.update_from_bytes(MINIMAP_WIDTH, MINIMAP_HEIGHT, &buffer);
}
