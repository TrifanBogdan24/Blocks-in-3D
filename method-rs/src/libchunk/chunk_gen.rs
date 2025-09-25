use std::cmp::{max, min};

pub fn is_inside(width: usize, height: usize, depth: usize, x: isize, y: isize, z: isize) -> bool {
    0 <= x
        && x < (width as isize)
        && 0 <= y
        && y < (height as isize)
        && 0 <= z
        && z < (depth as isize)
}

pub fn chunk_place_block(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize,
    height: usize,
    depth: usize,
    x: isize,
    y: isize,
    z: isize,
    block: u8,
) {
    if !is_inside(width, height, depth, x, y, z) {
        return;
    }
    chunk[x as usize][y as usize][z as usize] = block;
}

pub fn chunk_fill_cuboid(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize,
    height: usize,
    depth: usize,
    x0: isize,
    y0: isize,
    z0: isize,
    x1: isize,
    y1: isize,
    z1: isize,
    block: u8,
) {
    let min_x = max(min(x0, x1), 0isize);
    let min_y = max(min(y0, y1), 0isize);
    let min_z = max(min(z0, z1), 0isize);

    let max_x = min(max(x0, x1), (width - 1) as isize);
    let max_y = min(max(y0, y1), (height - 1) as isize);
    let max_z = min(max(z0, z1), (depth - 1) as isize);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                chunk_place_block(chunk, width, height, depth, x, y, z, block);
            }
        }
    }
}

fn euclidian_dist(x0: isize, y0: isize, z0: isize, x1: isize, y1: isize, z1: isize) -> f32 {
    ((x0 - x1).pow(2) as f32 + (y0 - y1).pow(2) as f32 + (z0 - z1).pow(2) as f32).sqrt()
}

pub fn chunk_fill_sphere(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize,
    height: usize,
    depth: usize,
    x: isize,
    y: isize,
    z: isize,
    radius: f32,
    block: u8,
) {
    let r = radius.ceil() as isize;

    for i in -r..=r {
        for j in -r..=r {
            for k in -r..=r {
                let dist: f32 = euclidian_dist(x, y, z, x + i, y + j, z + k);

                if dist > radius {
                    continue;
                }

                chunk_place_block(chunk, width, height, depth, x + i, y + j, z + k, block);
            }
        }
    }
}
