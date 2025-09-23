#![cfg(test)]

use rstest::rstest;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

use blocks_in_3D::chunk_io::*;
use blocks_in_3D::libchunk::chunk::{self, *};
use blocks_in_3D::libchunk::chunk_gen::*;
use blocks_in_3D::libchunk::chunk_transform::*;
use blocks_in_3D::libchunk::chunk_process::*;
use blocks_in_3D::libchunk::chunk_compress::*;




fn cmp_binary_files(file1: &str, file2: &str) -> bool {
    let f1 = fs::read(file1)
        .expect(&format!("[EROARE] cannot read file {:?}", file1));
    let f2 = fs::read(file2)
        .expect(&format!("[EROARE] cannot read file {:?}", file2));
    f1 == f2
}


fn cmp_text_files(file1: &str, file2: &str) -> bool {
    // Read files as strings
    let content1 = fs::read_to_string(file1)
        .expect(&format!("[ERROR] cannot read file {:?}", file1));
    let content2 = fs::read_to_string(file2)
        .expect(&format!("[ERROR] cannot read file {:?}", file2));

    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();

    // Compare line by line up to the shortest file
    let min_len = lines1.len().min(lines2.len());
    for i in 0..min_len {
        if lines1[i].trim_end() != lines2[i].trim_end() {
            return false; // first mismatch
        }
    }

    // If one file has extra lines, it's a mismatch
    if lines1.len() != lines2.len() {
        return false;
    }

    true // all lines match
}



#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
fn test_task1(#[case] idx: usize) {
    let file_params: String = format!("../tests/params/task1/{}.param", idx);
    let file_in: String = format!("../tests/input/task1/{}.in", idx);
    let file_out: String = format!("../tests-out/method-rs/task1/{}.out", idx);
    let file_ref: String = format!("../tests/ref_output/task1/{}.ref", idx);

    let (mut chunk, width, height, depth) = fread_chunk(&file_in);
    let (x, y, z, block) = fread_block_coordinates(&file_params);
    
    chunk_place_block(&mut chunk, width, height, depth, x, y, z, block);
    fwrite_chunk(&file_out, &chunk, width, height, depth);

    assert!(
        cmp_text_files(&file_out, &file_ref),
        "Files {:?} and {:?} are NOT the identical",
        file_out,
        file_ref
    )
}

#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
fn test_task2(#[case] idx: usize) {
    let file_params: String = format!("../tests/params/task2/{}.param", idx);
    let file_in: String = format!("../tests/input/task2/{}.in", idx);
    let file_out: String = format!("../tests-out/method-rs/task2/{}.out", idx);
    let file_ref: String = format!("../tests/ref_output/task2/{}.ref", idx);

    let file = File::open(file_params).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap(); 

    // Split line by whitespace
    let mut parts = line.trim().split_whitespace();

    let x0: usize = parts.next().expect("Missing x0").parse().expect("Invalid x0");
    let y0: usize = parts.next().expect("Missing y0").parse().expect("Invalid y0");
    let z0: usize = parts.next().expect("Missing z0").parse().expect("Invalid z0");

    let x1: usize = parts.next().expect("Missing x1").parse().expect("Invalid x1");
    let y1: usize = parts.next().expect("Missing y1").parse().expect("Invalid y1");
    let z1: usize = parts.next().expect("Missing z1").parse().expect("Invalid z1");

    let block: u8 = parts.next().expect("Missing block").parse().expect("Invalid block");

    let (mut chunk, width, height, depth) = fread_chunk(&file_in);
    chunk_fill_cuboid(&mut chunk, width, height, depth, x0, y0, z0, x1, y1, z1, block);
    fwrite_chunk(&file_out, &chunk, width, height, depth);

    assert!(
        cmp_text_files(&file_out, &file_ref),
        "Files {:?} and {:?} are NOT the identical",
        file_out,
        file_ref
    )
}


#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
fn test_task3(#[case] idx: usize) {
    let file_params: String = format!("../tests/params/task3/{}.param", idx);
    let file_in: String = format!("../tests/input/task3/{}.in", idx);
    let file_out: String = format!("../tests-out/method-rs/task3/{}.out", idx);
    let file_ref: String = format!("../tests/ref_output/task3/{}.ref", idx);

    let file = File::open(file_params).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap(); 

    // Split line by whitespace
    let mut parts = line.trim().split_whitespace();

    let x: usize = parts.next().expect("Missing x").parse().expect("Invalid x");
    let y: usize = parts.next().expect("Missing y").parse().expect("Invalid y");
    let z: usize = parts.next().expect("Missing z").parse().expect("Invalid z");

    let radius: f32 = parts.next().expect("Missing radius").parse().expect("Invalid radius");
    let block: u8 = parts.next().expect("Missing block").parse().expect("Invalid block");


    let (mut chunk, width, height, depth) = fread_chunk(&file_in);
    chunk_fill_sphere(&mut chunk, width, height, depth, x, y, z, radius, block);
    fwrite_chunk(&file_out, &chunk, width, height, depth);

    assert!(
        cmp_text_files(&file_out, &file_ref),
        "Files {:?} and {:?} are NOT the identical",
        file_out,
        file_ref
    )
}


#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
fn test_task4(#[case] idx: usize) {
    let file_params: String = format!("../tests/params/task4/{}.param", idx);
    let file_in: String = format!("../tests/input/task4/{}.in", idx);
    let file_out: String = format!("../tests-out/method-rs/task4/{}.out", idx);
    let file_ref: String = format!("../tests/ref_output/task4/{}.ref", idx);


    
    let file = File::open(file_params).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap(); 

    // Split line by whitespace
    let mut parts = line.trim().split_whitespace();

    let target_block: u8 = parts.next().expect("Missing target block").parse().expect("Invalid target block");
    let shell_block: u8 = parts.next().expect("Missing shell block").parse().expect("Invalid shell block");

    let (mut chunk, width, height, depth) = fread_chunk(&file_in);
    chunk_shell(&mut chunk, width, height, depth, target_block, shell_block);
    fwrite_chunk(&file_out, &chunk, width, height, depth);

    assert!(
        cmp_text_files(&file_out, &file_ref),
        "Files {:?} and {:?} are NOT the identical",
        file_out,
        file_ref
    )
}


#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
#[case(3)]
fn test_task5(#[case] idx: usize) {
    let file_params: String = format!("../tests/params/task5/{}.param", idx);
    let file_in: String = format!("../tests/input/task5/{}.in", idx);
    let file_out: String = format!("../tests-out/method-rs/task5/{}.out", idx);
    let file_ref: String = format!("../tests/ref_output/task5/{}.ref", idx);

    let (mut chunk, width, height, depth) = fread_chunk(&file_in);
    let (x, y, z, block) = fread_block_coordinates(&file_params);
    
    chunk_fill_xz(&mut chunk, width, height, depth, x, y, z, block);
    fwrite_chunk(&file_out, &chunk, width, height, depth);

    assert!(
        cmp_text_files(&file_out, &file_ref),
        "Files {:?} and {:?} are NOT the identical",
        file_out,
        file_ref
    )
}


#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(4)]
fn test_task6(#[case] idx: usize) {
    let file_params: String = format!("../tests/params/task6/{}.param", idx);
    let file_in: String = format!("../tests/input/task6/{}.in", idx);
    let file_out: String = format!("../tests-out/method-rs/task6/{}.out", idx);
    let file_ref: String = format!("../tests/ref_output/task6/{}.ref", idx);

    let (mut chunk, width, height, depth) = fread_chunk(&file_in);
    let (x, y, z, block) = fread_block_coordinates(&file_params);
    
    chunk_fill(&mut chunk, width, height, depth, x, y, z, block);
    fwrite_chunk(&file_out, &chunk, width, height, depth);

    assert!(
        cmp_text_files(&file_out, &file_ref),
        "Files {:?} and {:?} are NOT the identical",
        file_out,
        file_ref
    )
}



#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
#[case(3)]
fn test_task7(#[case] idx: usize) {
    let file_in: String = format!("../tests/input/task7/{}.in", idx);
    let file_out: String = format!("../tests-out/method-rs/task7/{}.out", idx);
    let file_ref: String = format!("../tests/ref_output/task7/{}.ref", idx);

    let (mut chunk, mut width, mut height, mut depth) = fread_chunk(&file_in);
    chunk_rotate_y(&mut chunk, &mut width, &mut height, &mut depth);
    fwrite_chunk(&file_out, &chunk, width, height, depth);

    assert!(
        cmp_text_files(&file_out, &file_ref),
        "Files {:?} and {:?} are NOT the identical",
        file_out,
        file_ref
    )
}


#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(4)]
fn test_task9(#[case] idx: usize) {
    let file_in: String = format!("../tests/input/task9/{}.in", idx);
    let file_out: String = format!("../tests-out/method-rs/task9/{}.out", idx);
    let file_ref: String = format!("../tests/ref_output/task9/{}.ref", idx);

    let (chunk, width, height, depth) = fread_chunk(&file_in);

    let code: Vec<u8> = chunk_encode(&chunk, width, height, depth);
    fwrite_encode(&file_out, &code);    

    assert!(
        cmp_binary_files(&file_out, &file_ref),
        "Files {:?} and {:?} are NOT the identical",
        file_out,
        file_ref
    )
}

#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(4)]
fn test_task10(#[case] idx: usize) {
    let file_params: String = format!("../tests/params/task10/{}.param", idx);
    let file_in: String = format!("../tests/input/task10/{}.in", idx);
    let file_out: String = format!("../tests-out/method-rs/task10/{}.out", idx);
    let file_ref: String = format!("../tests/ref_output/task10/{}.ref", idx);

    let (width, height, depth) = fread_chunk_sizes(&file_params);
    let code: Vec<u8> = fread_code(&file_in);

    let chunk = chunk_decode(&code, width, height, depth);
    fwrite_chunk(&file_out, &chunk, width, height, depth);


    assert!(
        cmp_text_files(&file_out, &file_ref),
        "Files {:?} and {:?} are NOT the identical",
        file_out,
        file_ref
    )
}
