fn flatten(
    chunk: &Vec<Vec<Vec<char>>>,
    width: usize,
    height: usize,
    depth: usize,
) -> Vec<char> {
    let mut array = Vec::with_capacity(width * height * depth);

    for y in 0..height {
        for z in 0..depth {
            for x in 0..width {
                array.push(chunk[x][y][z]);
            }
        }
    }

    array
}
