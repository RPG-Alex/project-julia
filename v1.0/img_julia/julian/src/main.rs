use image::{ImageBuffer, Rgb};
use nalgebra::{Complex, Normed};

fn julia(c: Complex<f64>, x: f64, y: f64) -> u8{
    let mut z = Complex::new(x,y);

    for i in 0..255 {
        if z.norm() > 2.0 {
            return i as u8;
        }
        z = z * z + c;
    }
    255
}

fn julia_as_u16(c: Complex<f64>, x: f64, y: f64) -> u16 {
    let mut z = Complex::new(x,y);

    for i in 0..255 {
        if z.norm() > 2.0 {
            return i as u16;
        }
        z = z * z + c;
    }
    u16::MAX
}

fn main() {
    let width = 1280;
    let height = 1280;



    let mut img = ImageBuffer::new(width, height);

    for (x,y,pixel) in img.enumerate_pixels_mut() {
        let scale_x = 3.0 / width as f64;
        let scale_y = 3.0 / height as f64;


        let cx = x as f64 * scale_x - 1.5;
        let cy = y as f64 * scale_y - 1.5;

        let c = Complex::new(-0.8, 0.156);
        let value = julia_as_u16(c, cx, cy);

        *pixel = Rgb([value,value,value]);

    }

    let _ = img.save("julia.png");

}
