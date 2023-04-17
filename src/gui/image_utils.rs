use crate::core::AntGrid;
use egui::ColorImage;
use image::{DynamicImage, GenericImage, Pixel};
fn grid_to_img(grid: &AntGrid) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(grid.cols as u32, grid.rows as u32);
    for elem in grid.grid.iter() {
        let color = elem.1.borrow().color().to_rgba();
        img.put_pixel(elem.0.x as u32, elem.0.y as u32, color);
    }
    return img;
}
pub(crate) fn get_image(grid: &AntGrid) -> ColorImage {
    let image = grid_to_img(grid);
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    return ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
}
