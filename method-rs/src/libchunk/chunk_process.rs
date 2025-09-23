use crate::libchunk::chunk_gen::is_inside;

fn wrapper(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize,
    target_block: u8, shell_block: u8
) {
    for i in -1..=1 {
        let px = (x as isize) + (i as isize);
        if px < 0 {
            continue;
        }

        for j in -1..=1 {
            let py = (y as isize) + (j as isize);
            if py < 0 {
                continue;
            }

            for k in -1..=1 {
                let pz = (z as isize) + (k as isize);
                if pz < 0 {
                    continue;
                }
                if i * j * k != 0 {
                    continue;
                }
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }

                let px = px as usize;
                let py = py as usize;
                let pz = pz as usize;

                if !is_inside(width, height, depth, px, py, pz) {
                    continue;
                }

                if chunk[px][py][pz] == target_block {
                    continue;
                }
                chunk[px][py][pz] = shell_block;
            }
        }
    }
}

pub fn chunk_shell(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize, height: usize, depth: usize,
    target_block: u8, shell_block: u8
) {
    let mut points_stack: Vec<Vec<usize>> = vec![];

    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                if chunk[x][y][z] != target_block {
                    continue;
                }

                points_stack.push(
                    vec![x, y, z]
                );

            }
        }
    }

    while !points_stack.is_empty() {
        let pct = points_stack.pop().expect("[ERROR] Empty stack");
        let px = pct[0];
        let py = pct[1];
        let pz = pct[2];

        wrapper(chunk, width, height, depth,
            px, py, pz,
            target_block, shell_block);
    }
}

fn fill_algorithm_x0z(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize,
    target_block: u8, new_block: u8
) {
    if !is_inside(width, height, depth, x, y, z) {
        return;
    }

    if chunk[x][y][z] != target_block {
        return;
    }

    chunk[x][y][z] = new_block;
    let dx: Vec<isize> = vec![0, 0, -1, 1];
    let dz: Vec<isize> = vec![-1, 1, 0, 0];

    for i in 0..dx.len() {
        let nx = (x as isize) + dx[i];
        let nz = (z as isize) + dz[i];

        if nx < 0 || nz < 0 {
            continue;
        }

        fill_algorithm_x0z(chunk, width, height, depth,
            nx as usize, y, nz as usize,            
            target_block, new_block);
    }
}


pub fn chunk_fill_xz(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize,
    block: u8
) {
    if !is_inside(width, height, depth, x as usize, y as usize, z as usize) {
        return;
    }

    fill_algorithm_x0z(chunk, width, height, depth, x, y, z, chunk[x][y][z], block);
}


fn fill_algorithm_3d(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize,
    target_block: u8, new_block: u8
) {
    if !is_inside(width, height, depth, x, y, z) {
        return;
    }
    if chunk[x][y][z] != target_block {
        return;
    }

    chunk[x][y][z] = new_block;

    let dx: Vec<isize> = vec![0, 0, 0, 0, -1, 1];
    let dy: Vec<isize> = vec![0, 0, -1, 1, 0, 0];
    let dz: Vec<isize> = vec![-1, 1, 0, 0, 0, 0];

    for i in 0..dx.len() {
        let nx = (x as isize) + dx[i];
        let ny = (y as isize) + dy[i];
        let nz = (z as isize) + dz[i];

        if nx < 0 || ny < 0 || nz < 0 {
            continue;
        }

        fill_algorithm_3d(chunk, width, height, depth,
            nx as usize, ny as usize, nz as usize,
            target_block, new_block);
    }
}


pub fn chunk_fill(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: usize, height: usize, depth: usize,
    x: usize, y: usize, z: usize,
    block: u8
) {
    if !is_inside(width, height, depth, x, y, z) {
        return;
    }
    if chunk[x][y][z] == block {
        return;
    }

    fill_algorithm_3d(
        chunk, width, height, depth,
        x, y, z,
        chunk[x][y][z], block);

}
