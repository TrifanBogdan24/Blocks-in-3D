use crate::libchunk::chunk::BLOCK_AIR;
use crate::libchunk::chunk::*;


#[derive(Default)]
struct Run {
    num_occurrences: usize,
    block: u8
}

const B0: usize = 0;
const B5: usize = 5;
const B8: usize = 8;
const B11: usize = 11;

const IDX_BIT_B1: usize = 7;
const IDX_BIT_B0: usize = 6;


const MAX_NUM_OCCURRENCES: usize = 4095;



fn flatten(
    chunk: &Vec<Vec<Vec<u8>>>,
    width: usize,
    height: usize,
    depth: usize,
) -> Vec<u8> {
    let mut array: Vec<u8> = vec![0; width * height * depth];

    for y in 0..height {
        for z in 0..depth {
            for x in 0..width {
                let idx = y * (depth * width) + z * width + x;
                array[idx] = chunk[x][y][z];
            }
        }
    }

    array
}

fn get_runs(flat_chunk: &[u8]) -> Vec<Run> {
    let mut runs = Vec::<Run>::new();

    if flat_chunk.len() == 0 {
        return runs;
    }

    let mut idx: usize = 1;
    let mut num_occurrences: usize = 1;
    let mut last_block = flat_chunk[0];

    while idx < flat_chunk.len() {
        if last_block != flat_chunk[idx] {
            runs.push(Run{num_occurrences: num_occurrences, block: last_block});
            last_block = flat_chunk[idx];
            num_occurrences = 1;
        } else {
            num_occurrences += 1;
        }

        if num_occurrences == MAX_NUM_OCCURRENCES {
            runs.push(Run{num_occurrences: MAX_NUM_OCCURRENCES, block: last_block});
            num_occurrences = 0;
        }

        idx += 1;
    }

    if num_occurrences > 0 {
        runs.push(Run{num_occurrences: num_occurrences, block: last_block});
    } 

    runs
}


fn encode_run(bytes: &mut Vec<u8>, run: &Run) {
    let mut byte = 0u8;

    match run.block {
        BLOCK_GRASS => byte |= 1 << IDX_BIT_B0,
        BLOCK_WOOD => byte |= 1 << IDX_BIT_B1,
        BLOCK_STONE => byte |= (1 << IDX_BIT_B0) | (1 << IDX_BIT_B1),
        _ => ()
    }

    if run.num_occurrences < (1 << B5) {
        // bb0nnnnn
        for i in B0..B5 {
            if run.num_occurrences & (1 << i) > 0 {
                byte |= 1 << i;
            }
        }
        bytes.push(byte);
    } else {
        // bb10nnnn nnnnnnnn
        
        byte |= 1 << B5;

        // Primul octet:
        for i in B8..=B11 {
            if run.num_occurrences & (1 << i) > 0 {
                byte |= 1 << (i - B8);
            }
        }
        bytes.push(byte.clone());

        // Al doilea octet:
        byte = 0u8;
        for i in B0..B8 {
            if run.num_occurrences & (1 << i) > 0 {
                byte |= 1 << i;
            }
        }
        bytes.push(byte);
    }
}


pub fn chunk_encode(
    chunk: &Vec<Vec<Vec<u8>>>,
    width: usize,
    height: usize,
    depth: usize,
) -> Vec<u8> {
    let flat_chunk = flatten(chunk, width, height, depth);
    let runs = get_runs(&flat_chunk);

    let mut bytes = Vec::<u8>::new();
    runs.iter().for_each(|run| encode_run(&mut bytes, run));

    bytes
}

pub fn chunk_decode(
    code: &Vec<u8>,
    width: usize,
    height: usize,
    depth: usize,
) -> Vec<Vec<Vec<u8>>> {
    let mut chunk: Vec<Vec<Vec<u8>>> = vec![vec![vec![BLOCK_AIR; depth]; height]; width];

    let (mut x, mut y, mut z) = (0usize, 0usize, 0usize);
    let mut idx: usize = 0;

    while idx < code.len() {
        let block: u8 = match (code[idx] & (1 << IDX_BIT_B1) > 0, code[idx] & (1 << IDX_BIT_B0) > 0)  {
            (true, true) => BLOCK_STONE,  // b1b0 = 11
            (true, false) => BLOCK_WOOD,  // b1b0 = 10
            (false, true) => BLOCK_GRASS, // b1b0 = 01
            _ => BLOCK_AIR                // b1b0 = 00
        };


        let mut num_occurrences = 0usize;

        if code[idx] & (1 << B5) == 0 {
            // bb0nnnnn
            for i in B0..B5 {
                if code[idx] & (1 << i) > 0 {
                    num_occurrences |= 1 << i;
                }
            }
        } else {
            // bb10nnnn nnnnnnnn

            // Primul octet:
            for i in B8..B11 {
                if code[idx] & (1 << (i - B8)) > 0 {
                    num_occurrences |= 1 << B8;
                }
            }

            // Al doilea octet:
            idx += 1;
            for i in B0..B8 {
                if code[idx] & (1 << i) > 0 {
                    num_occurrences |= 1 << i;
                }
            }
        }

        idx += 1;

        for _ in 0..num_occurrences {
            chunk[x][y][z] = block;

            x += 1;
            if x < width {
                continue;
            }

            x = 0;
            z += 1;

            if z < depth {
                continue;
            }

            z = 0;
            y += 1;

            if y >= height {
                return chunk;
            }
        }
    }

    chunk
}
