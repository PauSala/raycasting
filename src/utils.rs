use macroquad::{color::Color, shapes::draw_line};

use crate::{vector2::Vector2, MINIMAP_HEIGHT, MINIMAP_WIDTH};

pub const GRID_ROWS: usize = 20;
pub const GRID_COLS: usize = 20;
pub const ROW_SIZE: usize = MINIMAP_HEIGHT as usize / GRID_ROWS;
pub const COL_SIZE: usize = MINIMAP_WIDTH as usize / GRID_COLS;

pub static MAP: [[u8; GRID_COLS]; GRID_ROWS] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub fn draw_grid() {
    for i in 0..GRID_COLS {
        draw_line(
            (COL_SIZE * i) as f32,
            0.0,
            (COL_SIZE * i) as f32,
            MINIMAP_HEIGHT as f32,
            1.0,
            Color {
                r: 0.4,
                g: 0.4,
                b: 0.4,
                a: 0.5,
            },
        )
    }
    for i in 0..GRID_ROWS {
        draw_line(
            0.0,
            (ROW_SIZE * i) as f32,
            MINIMAP_WIDTH as f32,
            (ROW_SIZE * i) as f32,
            1.0,
            Color {
                r: 0.4,
                g: 0.4,
                b: 0.4,
                a: 0.5,
            },
        );
    }
}

pub fn pixel_index(x: u32, y: u32, width: u32) -> usize {
    ((y * width + x) * 4) as usize
}

pub fn point_at_distance(p1: &Vector2, pos: Vector2, distance: f32) -> Vector2 {
    let direction = (pos - *p1).norm();
    *p1 + Vector2 {
        x: direction.x * distance,
        y: direction.y * distance,
    }
}

/// DDA algorithm to perform raycasting
///
/// # Arguments
///
/// * `p1` - The starting point of the ray in grid coordinates
/// * `p2` - The ending point of the ray in grid coordinates
pub fn dda(p1: Vector2, p2: Vector2) -> (bool, Vector2, f32) {
    let v_ray_dir = (p2 - p1).norm();
    let delta_dist = Vector2::new(
        (1.0 + (v_ray_dir.y / v_ray_dir.x).powi(2)).sqrt(),
        (1.0 + (v_ray_dir.x / v_ray_dir.y).powi(2)).sqrt(),
    );
    let mut v_step = Vector2::default();
    let mut side_dist = Vector2::default();
    let mut v_map_check = Vector2::new(p1.x.trunc(), p1.y.trunc());
    let perp_wall_dist;
    let mut side;

    if v_ray_dir.x < 0.0 {
        v_step.x = -1.0;
        side_dist.x = (p1.x - v_map_check.x) * delta_dist.x;
    } else {
        v_step.x = 1.0;
        side_dist.x = ((v_map_check.x + 1.0) - p1.x) * delta_dist.x;
    }

    if v_ray_dir.y < 0.0 {
        v_step.y = -1.0;
        side_dist.y = (p1.y - v_map_check.y) * delta_dist.y;
    } else {
        v_step.y = 1.0;
        side_dist.y = ((v_map_check.y + 1.0) - p1.y) * delta_dist.y;
    }

    let mut f_distance;
    loop {
        if side_dist.x < side_dist.y {
            v_map_check.x += v_step.x;
            f_distance = side_dist.x;
            side_dist.x += delta_dist.x;
            side = 0;
        } else {
            v_map_check.y += v_step.y;
            f_distance = side_dist.y;
            side_dist.y += delta_dist.y;
            side = 1;
        }

        if (v_map_check.y as usize) < GRID_COLS
            && (v_map_check.x as usize) < GRID_ROWS
            && MAP[v_map_check.y as usize][v_map_check.x as usize] == 1
        {
            let int = (p1 + v_ray_dir * f_distance).to_pixel_coords();
            if side == 0 {
                perp_wall_dist = side_dist.x - delta_dist.x;
            } else {
                perp_wall_dist = side_dist.y - delta_dist.y;
            }
            return (true, int, perp_wall_dist);
        }

        if (v_map_check.y as usize) == GRID_COLS
            || (v_map_check.x as usize) == GRID_ROWS
            || (v_map_check.y) == -1.0
            || (v_map_check.x) == -1.0
        {
            let int = (p1 + v_ray_dir * f_distance).to_pixel_coords();
            return (false, int, 0.0);
        }
    }
}

pub fn dda_grid(p1: Vector2, p2: Vector2) -> Vec<(Vector2, f32)> {
    let v_ray_dir = (p2 - p1).norm();
    let delta_dist = Vector2::new(
        (1.0 + (v_ray_dir.y / v_ray_dir.x).powi(2)).sqrt(),
        (1.0 + (v_ray_dir.x / v_ray_dir.y).powi(2)).sqrt(),
    );
    let mut v_step = Vector2::default();
    let mut side_dist = Vector2::default();
    let mut v_map_check = Vector2::new(p1.x.trunc(), p1.y.trunc());
    let mut perp_wall_dist;
    let mut side;
    let mut ints = Vec::new();

    if v_ray_dir.x < 0.0 {
        v_step.x = -1.0;
        side_dist.x = (p1.x - v_map_check.x) * delta_dist.x;
    } else {
        v_step.x = 1.0;
        side_dist.x = ((v_map_check.x + 1.0) - p1.x) * delta_dist.x;
    }

    if v_ray_dir.y < 0.0 {
        v_step.y = -1.0;
        side_dist.y = (p1.y - v_map_check.y) * delta_dist.y;
    } else {
        v_step.y = 1.0;
        side_dist.y = ((v_map_check.y + 1.0) - p1.y) * delta_dist.y;
    }

    let mut f_distance;
    loop {
        if side_dist.x < side_dist.y {
            v_map_check.x += v_step.x;
            f_distance = side_dist.x;
            side_dist.x += delta_dist.x;
            side = 0;
        } else {
            v_map_check.y += v_step.y;
            f_distance = side_dist.y;
            side_dist.y += delta_dist.y;
            side = 1;
        }

        if side == 0 {
            perp_wall_dist = side_dist.x - delta_dist.x;
        } else {
            perp_wall_dist = side_dist.y - delta_dist.y;
        }

        let int = (p1 + v_ray_dir * f_distance).to_pixel_coords();
        ints.push((int, perp_wall_dist));

        if (v_map_check.y as usize) == GRID_COLS
            || (v_map_check.x as usize) == GRID_ROWS
            || (v_map_check.y) == -1.0
            || (v_map_check.x) == -1.0
        {
            return ints;
        }
    }
}

pub fn normalize(value: f32, min_value: f32, max_value: f32) -> u8 {
    let normalized_value = ((value - min_value) / (max_value - min_value)) * 255.0;
    normalized_value.clamp(0.0, 255.0) as u8
}
