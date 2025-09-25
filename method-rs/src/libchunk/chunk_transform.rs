use crate::libchunk::chunk::BLOCK_AIR;

pub fn chunk_rotate_y(
    chunk: &mut Vec<Vec<Vec<u8>>>,
    width: &mut usize, height: &mut usize, depth: &mut usize
) {
    let mut new_mat: Vec<Vec<Vec<u8>>> = vec![vec![vec![BLOCK_AIR; *width]; *height]; *depth];

    let new_width: usize = *depth;
    let new_depth: usize = *width;

    for x in 0..new_width {
        for y in 0..*height {
            for z in 0..new_depth {
                new_mat[x][y][z] = chunk[z][y][*depth - 1 - x];
            }
        }
    }

    *width = new_width;
    *depth = new_depth;
    *chunk = new_mat;
}

pub fn chunk_apply_gravity(
    _chunk: &mut Vec<Vec<Vec<u8>>>,
    _width: &mut usize, _height: &mut usize, _depth: &mut usize
) {
    // TODO
}