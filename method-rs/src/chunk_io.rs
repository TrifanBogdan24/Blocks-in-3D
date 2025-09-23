use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn print_chunk(
    chunk: &Vec<Vec<Vec<u8>>>,
    width: usize,
    height: usize,
    depth: usize
) {
    for x in 0..width {
        for y in (0..height).rev() {
            for z in 0..depth {
                print!("{} ", chunk[x][y][z]);
            }
            println!();
        }
    }
}


pub fn fread_chunk(file_in: &str) -> (Vec<Vec<Vec<u8>>>, usize, usize, usize) {
    // Read the whole file as a string
    let input = fs::read_to_string(file_in)
        .expect("Failed to read params file");

    let mut lines = input.lines();

    // First three lines are width, height, depth
    let width: usize = lines
        .next()
        .expect("Missing width")
        .trim()
        .parse()
        .expect("Invalid width");
    let height: usize = lines
        .next()
        .expect("Missing height")
        .trim()
        .parse()
        .expect("Invalid height");
    let depth: usize = lines
        .next()
        .expect("Missing depth")
        .trim()
        .parse()
        .expect("Invalid depth");

    // Initialize chunk with zeros
    let mut chunk = vec![vec![vec![0u8; depth]; height]; width];

    for x in 0..width {
        for y in (0..height).rev() {
            // Skip optional blank lines
            let mut line = lines.next().unwrap_or("").trim();
            while line.is_empty() {
                line = lines.next().unwrap_or("").trim();
            }

            let nums: Vec<u8> = line
                .split_whitespace()
                .map(|s| s.parse::<u8>().expect("Invalid matrix value"))
                .collect();

            assert_eq!(
                nums.len(),
                depth,
                "Line length does not match declared depth"
            );

            for (z, &val) in nums.iter().enumerate() {
                chunk[x][y][z] = val;
            }
        }
    }

    (chunk, width, height, depth)
}


pub fn fread_basic_params(file_params: &str) -> (usize, usize, usize, u8) {
    let file = File::open(file_params).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap(); 

    // Split line by whitespace
    let mut parts = line.trim().split_whitespace();

    let x: usize = parts.next().expect("Missing x").parse().expect("Invalid x");
    let y: usize = parts.next().expect("Missing y").parse().expect("Invalid y");
    let z: usize = parts.next().expect("Missing z").parse().expect("Invalid z");

    let block: u8 = parts.next().expect("Missing block").parse().expect("Invalid block");

    (x, y, z, block)
}

fn create_file_out(file_out: &str) -> File {
    let path = Path::new(file_out);

    // Create all parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect(&format!(
            "[ERROR] cannot create parent directories for {:?}",
            file_out
        ));
    }

    File::create(path)
        .expect(&format!("[ERROR] cannot create {:?} output file", file_out))
}

pub fn fwrite_chunk(
    file_out: &str,
    chunk: &Vec<Vec<Vec<u8>>>,
    width: usize,
    height: usize,
    depth: usize
) {
    let mut file: File = create_file_out(file_out);
    writeln!(file, "{}", width).expect("Failed to write width");
    writeln!(file, "{}", height).expect("Failed to write height");
    writeln!(file, "{}", depth).expect("Failed to write depth");

    for x in 0..width {
        for y in (0..height).rev() {
            for z in 0..depth {
                write!(file, "{} ", chunk[x][y][z]).expect("Failed to write chunk value");
            }
            writeln!(file).expect("Failed to write newline");
        }
        writeln!(file).expect("Failed to write slice separator newline");
    }
}

pub fn fwrite_encode(
    file_out: &str,
    code: &[u8]
) {
    let mut file: File = create_file_out(file_out);
    file.write(&code).expect(&format!("Cannot write bytes in {}", file_out));
}
