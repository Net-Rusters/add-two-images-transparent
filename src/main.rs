
use image::*;

fn main() {
    let mut img1 = image::open("./src/image1.jpg").unwrap();
    let mut img2 = image::open("./src/image2.jpg").unwrap();

    let (width, height) = img1.dimensions();

    img2 = img2.crop(0, 0, width, height);

    for i in 0..width {
        for j in 0..height {
            let mut pixel1 = img1.get_pixel(i, j);
            let mut pixel2 = img2.get_pixel(i, j);
            let mut channels = pixel2.channels_mut();
            channels[3] = 170;
            pixel1.blend(&pixel2);
            img1.put_pixel(i, j, pixel1);
        }
    }
    img1.save("test.jpg").unwrap();
}
