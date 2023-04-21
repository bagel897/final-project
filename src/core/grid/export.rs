use egui::ColorImage;
use image::{Pixel, Rgb};

pub(crate) struct Export {
    colors: Vec<Vec<Rgb<u8>>>,
    rows: usize,
    cols: usize,
}
impl Export {
    pub fn new(colors: Vec<Vec<Rgb<u8>>>, rows: usize, cols: usize) -> Self {
        Export { colors, rows, cols }
    }
    pub fn to_image(&self) -> ColorImage {
        const COLORS: usize = 4;
        let mut image_buffer: Vec<u8> = vec![0; self.rows * self.cols * COLORS];
        for (x, row) in self.colors.iter().enumerate() {
            for (y, color) in row.iter().enumerate() {
                let color_a = color.to_rgba();
                let idx = COLORS * (x + y * self.cols);
                for i in 0..COLORS {
                    image_buffer[idx + i] = color_a[i];
                }
            }
        }
        let size = [self.cols as _, self.rows as _];
        return ColorImage::from_rgba_unmultiplied(size, image_buffer.as_slice());
    }
}
