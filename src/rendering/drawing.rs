pub fn rgb(r: f32, g: f32, b: f32, a: f32) -> [f32; 4] {
    [r / 255.0, g / 255.0, b / 255.0, a]
}