
use noise::{NoiseFn, Perlin};
use image::{ImageBuffer, Rgb};
use rand::Rng;

mod colors;
use colors::get_color;

fn main() {
    // Image size
    let width:u32 = 128;
    let height:u32 = 128;
    // Random gen
    let mut rng = rand::thread_rng();
    let seed:u32 = rng.gen();
    // Create a Perlin noise generator
    let perlin:Perlin   = Perlin::new(seed); 
    // Create an image buffer
    let mut img:ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let mut perimg:ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Parameters for noise scaling
    let scale:f64 = 30.0;

    for y in 0..height {
        for x in 0..width {
            // Normalized coordinates
            let nx = x as f64 / scale;
            let ny = y as f64 / scale;

            // Generate noise value between -1 and 1 and scale it to 0-255
            let noise_value = perlin.get([nx, ny]) + 0.25;
            
            // Assign a color with a slight blue tint for terrain
            img.put_pixel(x, y, get_color(noise_value));
            // show the noise
            let color_value = ((noise_value) * 254.0) as u8;
            perimg.put_pixel(x, y, Rgb([color_value, color_value, color_value]));
        }
    }

    // Save the image as a PNG
    img.save("terrain.png").expect("Failed to save the image");
    perimg.save("noise.png").expect("Failed to save noise showcase");
    println!("Texture generated and saved as 'terrain.png'.");
}