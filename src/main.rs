use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;


const IMG_WIDTH :usize = 512;
const IMG_HEIGHT : usize = 512;

struct Point {
    row: usize,
    col: usize,
}

fn main() -> std::io::Result<()>{
    println!("Hello, world!");
    fn generate_point(width: usize, height: usize) -> Option<Point> {
        let mut rng = thread_rng();
        if rng.gen() {
            let row = rng.gen_range(0..height);
            let col = rng.gen_range(0..width);
            let point = Point{row, col};
            return Some(point)
        }
        else{
            return None
        }
    }
    fn euclidian_squared_distance(p1: Point, p2: Point)-> usize{
        let row_diff = p1.row - p2.row;
        let col_diff = p1.col - p2.col;
        return row_diff*row_diff + col_diff*col_diff
    }
    let mut file = File::create("voronoi.ppm")?;
    // ppm format:
    // P3\n
    // {img_width} {img_height}\n 
    // 255\n --> maximum color size
    // {a list of rgb triplets}
    let header = String::from("P3\n");
    let header = format!("P3\n{img_width} {img_height}\n255\n", img_width=IMG_WIDTH, img_height=IMG_HEIGHT);
    file.write(header.as_bytes())?;
    //let mut rows : Vec<String> = Vec::new();
    for row in 0..IMG_HEIGHT {
        let mut current_line = String::new();
        for col in 0..IMG_WIDTH {
            current_line.push_str(&String::from("0 255 0 "))
        }
        current_line.push_str(&String::from("\n"));
        file.write(current_line.as_bytes())?;
    }
    return Ok(())
}
