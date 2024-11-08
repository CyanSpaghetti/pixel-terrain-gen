use image::Rgb;

struct Terrain {
    green:Rgb<u8>,
    brown:Rgb<u8>,
    blue:Rgb<u8>,
}

pub fn get_color(noise:f64) -> Rgb<u8> {

    let t = Terrain {
        green : Rgb([0, 228, 54]),
        brown : Rgb([171, 82, 54]),
        blue : Rgb([41, 173, 255]),
    };

    

    if noise >= 0.6 {
        return t.green;
    } else if noise >= 0.5 {
        return t.brown;
    }
    t.blue
} 