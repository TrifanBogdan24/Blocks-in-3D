use std::num;
use crate::libchunk::chunk::*;



#[derive(Default)]
struct Run {
    num_occurrences: usize,
    block: u8
}


const IDX_BIT_B1: usize = 7;
const IDX_BIT_B0: usize = 6;


const MAX_NUM_OCCURRENCES: usize = 4095;



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

fn get_runs(flat_chunk: &[u8]) -> Vec<Run> {
    let mut runs = Vec::<Run>::new();
    
    let mut last_block = flat_chunk[0];
    let mut num_occurrences: usize = 0;
    let mut idx: usize = 1;

    while idx < flat_chunk.len() {
        if last_block != flat_chunk[idx] {
            runs.push(Run{num_occurrences: num_occurrences, block: last_block});
            last_block = flat_chunk[idx];
            num_occurrences = 1;
            continue;
        }

        num_occurrences += 1;

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

    if run.num_occurrences < 32 {
        for i in 0..=4 {
            if run.num_occurrences & (1 << i) != 0 {
                byte |= 1 << i;
            }
        }

        bytes.push(byte);
    } else {
        // Primul octet:
        for i in 8..=11 {
            if run.num_occurrences & (1 << i) != 0 {
                byte |= 1 << (i - 8);
            }
        }
        bytes.push(byte);

        // Al doilea octet:
        byte = 0u8;
        for i in 0..=7 {
            if run.num_occurrences & (1 << i) != 0 {
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
    let _ = runs.iter().for_each(|run| encode_run(&mut bytes, run));
    bytes
}

pub fn chunk_decode(
    code: &Vec<u8>,
    width: usize,
    height: usize,
    depth: usize,
) -> Vec<u8> {
    todo!();
}
