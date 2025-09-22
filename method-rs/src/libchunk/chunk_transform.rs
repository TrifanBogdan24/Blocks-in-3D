pub fn chunk_rotate_y(
    chunk: &mut Vec<Vec<Vec<char>>>,
    width: usize, height: usize, depth: usize
) {
    let mut new_mat: Vec<Vec<Vec<char>>> = vec![vec![vec![' '; width]; height]; depth];

    for x in 0..depth {
        for y in 0..height {
            for z in 0..width {
                new_mat[x][y][z] = chunk[z][y][depth - 1 - x];
            }
        }
    }

    *chunk = new_mat;
}

