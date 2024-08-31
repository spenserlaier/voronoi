use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
//use std::cmp;


const IMG_WIDTH :usize = 512;
const IMG_HEIGHT : usize = 512;
//const NUM_POINTS : usize = (IMG_WIDTH*IMG_HEIGHT) / 10; 
const NUM_POINTS :usize = 10;

struct Point {
    row: usize,
    col: usize,
}

fn generate_point(rng: &mut ThreadRng, width: usize, height: usize) -> Result<Point, String> {
    let row:usize = rng.gen_range(0..height);
    let col:usize = rng.gen_range(0..width);
    let point = Point{row, col};
    return Ok(point)
}
fn euclidian_squared_distance(p1: &Point, p2: &Point)-> isize{
    let row_diff = p1.row as isize - p2.row as isize; 
    let col_diff = p1.col as isize - p2.col as isize;
    return row_diff*row_diff + col_diff*col_diff
}
#[derive(Clone)]
struct Color {
    r: usize,
    g: usize,
    b: usize,
}
fn hex_to_color(hex: usize)-> Color {
    let r = 0x0000FF & (hex >> 16);
    let g = 0x0000FF & (hex >> 8);
    let b = 0x0000FF & hex;
    return Color {r, g, b}
}
fn write_colors_to_image(colors: Vec<Color>) -> std::io::Result<()>{
    let mut file = File::create("voronoi.ppm")?;
    // ppm format:
    // P3\n
    // {img_width} {img_height}\n 
    // 255\n --> maximum color size
    // {a list of rgb triplets}
    let header = format!("P3\n{img_width} {img_height}\n255\n", img_width=IMG_WIDTH, img_height=IMG_HEIGHT);
    file.write(header.as_bytes())?;
    for color in colors {
        let color_str = format!("{r} {g} {b} \n", r=color.r, g=color.g, b = color.b);
        file.write(color_str.as_bytes()).unwrap();
    }
    return Ok(());
}
fn main(){
    let color_hex_vals = [0x5E0B15, 0x90323D, 0xD9CAB3, 0xBC8034, 0x8C7A6B];
    let mut colors : Vec<Color>= Vec::new();
    for hex in color_hex_vals {
        colors.push(hex_to_color(hex));
    }
    //let mut rows : Vec<String> = Vec::new();
    let mut image_colors: Vec<Color> = Vec::new();
    let mut points: Vec<Point> = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..NUM_POINTS {
        points.push(generate_point(&mut rng, IMG_WIDTH, IMG_HEIGHT).unwrap());
    }
    println!("we made it to the point of generating image colors");
    
    for row in 0..IMG_HEIGHT {
        for col in 0..IMG_WIDTH {
            let image_point = Point{row, col};
            let mut prev_dist = (IMG_WIDTH*IMG_WIDTH + IMG_HEIGHT*IMG_HEIGHT) as isize;
            let mut prev_idx = IMG_WIDTH;
            for idx in 0..points.len() {
                let point = points.get(idx).unwrap();
                let new_dist = euclidian_squared_distance(&image_point, &point);
                if new_dist < prev_dist{
                    prev_dist = new_dist;
                    prev_idx = idx;
                }
            }
            let color_idx = prev_idx % colors.len();
            let pixel_color = colors.get(color_idx).unwrap();
            image_colors.push(pixel_color.clone());
        }
    }
    println!("we made it to the point of writing colors to the image");
    write_colors_to_image(image_colors).unwrap();
  //  for row in 0..IMG_HEIGHT {
  //      let mut current_line = String::new();
  //      for col in 0..IMG_WIDTH {
  //          //TODO: if i remember correctly, the ppm format might impose limits on line size.
  //          //if this is the case, it may make sense to put each color on its own line. requires
  //          //research.
  //          current_line.push_str(&String::from("0 255 0 "))
  //      }
  //      current_line.push_str(&String::from("\n"));
  //      file.write(current_line.as_bytes())?;
  //  }
}
