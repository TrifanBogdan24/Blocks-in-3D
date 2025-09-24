use std::cmp::{min, max};

pub fn is_inside(
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize
) -> bool {
    x < width && y < height && z < depth
}


pub fn chunk_place_block(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize,
    block: u8
) {
    if !is_inside(width, height, depth, x as usize, y as usize, z as usize) {
        return;
    }
    chunk[x][y][z] = block;
}




pub fn chunk_fill_cuboid(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize, height: usize, depth: usize,
    x0: isize, y0: isize, z0: isize,
    x1: isize, y1: isize, z1: isize,
    block: u8
) {
    let min_x = max(min(x0, x1), 0isize) as usize;
    let min_y = max(min(y0, y1), 0isize) as usize;
    let min_z = max(min(z0, z1), 0isize) as usize;

    let max_x = min(max(x0, x1), (width - 1) as isize) as usize;
    let max_y = min(max(y0, y1), (height - 1) as isize) as usize;
    let max_z = min(max(z0, z1), (depth - 1) as isize) as usize;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                chunk_place_block(
                    chunk, width, height, depth, 
                    x, y, z, block);
            }
        } 
    }
}


fn euclidian_dist(
    x0: usize, y0: usize, z0: usize,
    x1: usize, y1: usize, z1: usize
) -> f32 {
    ((x0 - x1).pow(2) as f32 + (y0 - y1).pow(2) as f32 + (z0 - z1).pow(2) as f32).sqrt()
}

pub fn chunk_fill_sphere(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize,
    radius: f32, block: u8
) {
    let r: usize = radius.ceil().abs() as usize;

    for i in 0..=(2*r) {
        if i >= x {
            continue;
        }
        for j in 0..=(2*r) {
            if j >= y {
                continue;
            }
            for k in 0..=(2*r) {
                if k >= z {
                    continue;
                }
                let dist: f32 = euclidian_dist(x, y, z, x + i, y as usize + j, z as usize + k);

                if dist > radius {
                    continue;
                }

                chunk_place_block(chunk, width, height, depth, x + i, y + j, z + k, block);

            }
        }
    }
}
