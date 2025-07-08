use image::{GrayImage, Luma};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use vm_parser::vm_execute;

struct Program {}

impl Program {
    fn execute(&self, vx: f64, vy: f64) -> f64 {
        vm_execute!();
    }
}

fn main() {
    let program: Program = Program {};
    let mut img = GrayImage::new(1024, 1024);
    let img_usize = &mut img as *const _ as usize;
    (0..1024).into_par_iter().for_each(|y| {
        // WHEEEEEEEEE
        let img = unsafe { &mut *(img_usize as *mut GrayImage) };
        for x in 0..1024 {
            let vx = (x as f64 / 512.0) - 1.0;
            let vy = 1.0 - (y as f64 / 512.0);

            let out = program.execute(vx, vy);
            let pix = if out > 0.0 { 0 } else { 255 };
            *img.get_pixel_mut(x, y) = Luma([pix]);
        }
    });

    img.save("out.png").unwrap();
}
