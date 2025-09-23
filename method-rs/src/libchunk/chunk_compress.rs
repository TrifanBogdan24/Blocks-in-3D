fn flatten(
    chunk: &Vec<Vec<Vec<u8>>>,
    width: usize,
    height: usize,
    depth: usize,
) -> Vec<u8> {
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


pub fn chunk_encode(
    chunk: &Vec<Vec<Vec<u8>>>,
    width: usize,
    height: usize,
    depth: usize,
) -> Vec<u8> {
    todo!();
}

pub fn chunk_decode(
    code: &Vec<u8>,
    width: usize,
    height: usize,
    depth: usize,
) -> Vec<u8> {
    todo!();
}
