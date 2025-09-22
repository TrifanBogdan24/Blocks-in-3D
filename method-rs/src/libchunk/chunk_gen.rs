
pub fn is_inside(
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize
) -> bool {
    x < width && y < height && z < depth
}


pub fn chunk_place_block(
    chunk: &mut Vec<Vec<Vec<char>>>,
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize,
    block: char
) {
    if !is_inside(width, height, depth, x as usize, y as usize, z as usize) {
        return;
    }
    chunk[x][y][z] = block;
}


fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}


pub fn chunk_fill_cuboid(
    chunk: &mut Vec<Vec<Vec<char>>>,
    width: usize, height: usize, depth: usize,
    x0: usize, y0: usize, z0: usize,
    x1: usize, y1: usize, z1: usize,
    block: char
) {
    for x in min(x0, x1)..=max(x0, x1) {
        for y in min(y0, y1)..=max(y0, y1) {
            for z in min(z0, z1)..=max(z0, z1) {
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
    chunk: &mut Vec<Vec<Vec<char>>>,
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize,
    radius: f32, block: char
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
