use crate::core::AntGrid;
use egui::ColorImage;
use image::Pixel;

pub(crate) fn get_image(grid: &AntGrid) -> ColorImage {
    const COLORS: usize = 4;
    let mut image_buffer: Vec<u8> = vec![0; grid.rows * grid.cols * COLORS];
    for elem in grid.grid.iter() {
        let color = elem.1.borrow().color().to_rgba();
        let idx = COLORS * (elem.0.x + elem.0.y * grid.cols);
        for i in 0..COLORS {
            image_buffer[idx + i] = color[i];
        }
    }
    let size = [grid.cols as _, grid.rows as _];
    return ColorImage::from_rgba_unmultiplied(size, image_buffer.as_slice());
}
