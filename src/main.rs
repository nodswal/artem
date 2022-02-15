use std::{fs::File, io::Write, panic};

use colored::*;
use image::{GenericImageView, Rgba};

//import cli
mod cli;

fn main() {
    //get args from cli
    let matches = cli::build_cli().get_matches();

    //density char map
    let density = if matches.is_present("density") {
        match matches.value_of("density").unwrap() {
            "short" | "s" | "0" => r#"Ñ@#W$9876543210?!abc;:+=-,._ "#,
            "medium" | "m" | "1" => {
                r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#
            }
            "long" | "l" | "2" => {
                r#"¶@ØÆMåBNÊßÔR#8Q&mÃ0À$GXZA5ñk2S%±3Fz¢yÝCJf1t7ªLc¿+?(r/¤²!*;"^:,'.` "#
            }
            _ => matches.value_of("density").unwrap(),
        }
    } else {
        r#"Ñ@#W$9876543210?!abc;:+=-,._ "#
    };

    //this should be save to unwrap since the input has to be non-null
    let img = match image::open(matches.value_of("INPUT").unwrap()) {
        Ok(img) => img,
        //Todo use error function
        Err(_) => panic!("Image not found"),
    };

    let width = img.width();
    let height = img.height();

    //get target size from args
    let target_size = match matches
        .value_of("size")
        .unwrap() //this should always be at least "80", so it should be safe to unwrap
        .parse::<u32>()
    {
        Ok(v) => v.clamp(80, 200),
        Err(_) => panic!("Could not work with size input value"),
    };

    //clamp image width to a maximum of 80
    //todo add custom tiling
    let columns = if width > target_size {
        target_size
    } else {
        width
    };
    let scale = 0.43;

    //calculate tiles
    let tile_width = width / columns;
    let tile_height = (tile_width as f64 / scale).floor() as u32;

    let rows = height / tile_height;

    let mut terminal_output = String::new();
    let mut file_output = String::new();
    for row in 0..rows {
        for col in 0..columns {
            //get a single tile
            let tile_row = row * tile_height;
            let tile_col = col * tile_width;
            //create a pixel block from multiple pixels
            let mut pixel_block: Vec<Rgba<u8>> = Vec::new();
            //crop image to smaller block
            let crop = img.crop_imm(tile_col, tile_row, tile_width, tile_height);
            for pixel in crop.pixels() {
                //add pixel to block
                pixel_block.push(pixel.2);
            }
            //get and display density char
            //this returns a normal and a colored string
            let char = get_pixel_density(pixel_block, density);
            //save the normal string to the output file
            file_output.push_str(char.0.as_str());
            //save the colored string for the terminal output
            terminal_output.push_str(char.1.to_string().as_str());
        }
        //add new line
        terminal_output.push('\n');
        file_output.push('\n');
    }
    //check if no colors should be used
    if matches.is_present("no-color") {
        //print the "normal" non-colored conversion
        println!("{}", file_output);
    } else {
        //print colored terminal conversion
        println!("{}", terminal_output);
    }

    //create and write to output file
    if matches.is_present("output-file") && matches.value_of("output-file").is_some() {
        let mut file = match File::create(matches.value_of("output-file").unwrap()) {
            Ok(f) => f,
            Err(_) => panic!("Could not create file"),
        };

        match file.write(file_output.as_bytes()) {
            Ok(_) => {}
            Err(_) => panic!("Could not write to file"),
        };
    }
}

//Remap a value from one range to another.
fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

//Convert a pixel block to a char.
//The converted char will be returned as a string and as a colored string.
fn get_pixel_density(block: Vec<Rgba<u8>>, density: &str) -> (String, ColoredString) {
    let mut block_avg: f64 = 0f64;
    //color
    let mut red: f64 = 0f64;
    let mut blue: f64 = 0f64;
    let mut green: f64 = 0f64;

    //average all pixel in a block
    for pixel in &block {
        let r = pixel.0[0] as f64;
        let g = pixel.0[1] as f64;
        let b = pixel.0[2] as f64;
        //save the pixel values
        red += r;
        blue += b;
        green += g;
        //luminosity color http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/
        let pixel_luminosity = 0.21 * r + 0.72 * g + 0.07 * b;
        block_avg += pixel_luminosity;
    }

    block_avg /= block.len() as f64;
    //block average color
    red /= block.len() as f64;
    blue /= block.len() as f64;
    green /= block.len() as f64;

    //swap to range for white to black values
    //convert from rgb values (0 - 255) to the density string index (0 - string length)
    // let density_index = map_range((0f64, 255f64), (density.len() as f64, 0f64), block_avg)
    let density_index = map_range((0f64, 255f64), (0f64, density.len() as f64), block_avg)
        .floor()
        .clamp(0f64, density.len() as f64);

    //todo use directional chars
    //get correct char from map
    let density_char = density.chars().nth(density_index as usize);
    if density_char.is_some() {
        //return non an colored string
        (
            density_char.unwrap().to_string(),
            //use truecolor since it is supported basically everywhere
            density_char.unwrap().to_string().truecolor(
                red.floor() as u8,
                green.floor() as u8,
                blue.floor() as u8,
            ),
        )
    } else {
        //return non an colored string
        (' '.to_string(), ' '.to_string().normal())
    }
}
