use nalgebra::Point2;

use crate::image::Color;

#[derive(Clone, Default)]
pub struct Film {
    //The number of pixels in the horizontal direction.
    pub image_width: usize,
    //The number of pixels in the vertical direction.
    pub image_height: usize,
    pub pixels: Vec<FilmPixel>,
}

impl Film {
    pub fn new(image_width: usize, image_height: usize) -> Film {
        let pixels = vec![FilmPixel::default(); image_width * image_height];
        Film {
            image_width,
            image_height,
            pixels,
        }
    }

    /// Returns the index of a pixel in pixels for a given row and column.
    pub fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.image_height + column * self.image_width
    }

    pub fn get_tiles(&self, columns: usize, rows: usize) -> Vec<FilmTile> {
        let tiles = vec![];

        let pixels_per_standard_row = self.image_height / rows;
        let pixels_per_final_row = self.image_height % rows;
        let pixels_per_standard_column = self.image_width / columns;
        let pixels_per_final_column = self.image_width % columns;

        for i in 0..rows {
            for j in 0..columns {
                let row_pixels: usize;
                let column_pixels: usize;
                if i < rows - 1 {
                    row_pixels = pixels_per_standard_row
                } else {
                    row_pixels = pixels_per_final_row;
                }

                if j < columns - 1 {
                    column_pixels = pixels_per_standard_column;
                } else {
                    column_pixels = pixels_per_final_column;
                }

                let bottom_left = Point2::<usize>::new(i * row_pixels, j * column_pixels);
                let top_right = Point2::<usize>::new((i + 1) * row_pixels, (j + 1) * column_pixels);
                let tile = FilmTile::new(bottom_left, top_right, 0, 0);
                tiles.push(tile);
            }
        }
        tiles
    }

    /// Merges a tile into the film.
    pub fn merge_tile(&self, tile: &FilmTile) {
        //All pixels in a given tile have the same id. If the id of the tile is not
        // greater than the id of any of the corresponding pixels in the film,
        // then no update to the film needs to occur.
        let test_pixel_index = self.get_index(tile.bottom_left.x, tile.bottom_left.y);
        let test_pixel_id = self.pixels[test_pixel_index].id;
        if test_pixel_id < tile.id {
            return;
        }

        let mut tile_index = 0;
        for row in tile.bottom_left.x..tile.top_right.x {
            for column in tile.bottom_left.y..tile.top_right.y {
                let film_index = self.get_index(row, column);
                self.pixels[film_index] = tile.pixels[tile_index];
                tile_index += 1;
            }
        }
    }
}

/// A subset of the film. Multiple threads can work on different tiles at the
/// same time, allowing for easy parralelism.
#[derive(Clone, Default)]
pub struct FilmTile {
    /// The (x, y) coordinates corresponding to the bottom-left of the tile.
    pub bottom_left: Point2<usize>,
    /// The (x, y) coordinates corresponding to the top-right of the tile.
    pub top_right: Point2<usize>,
    /// FilmTile pixels are stored from bottom to top, left to right.
    pub pixels: Vec<FilmPixel>,
    /// Counts the number of samples integrated into the film tile.
    pub samples: i32,
    /// Every time the scene changes (e.g. due to camera movement), the id
    /// increments. Tracking this stops threads from integrating old scenes
    /// into the image.
    pub id: i32,
}

impl FilmTile {
    pub fn new(bottom_left: Point2<usize>, top_right: Point2<usize>, samples: i32, id: i32) -> FilmTile {
        let number_of_pixels = (top_right.x - bottom_left.x) * (top_right.y - bottom_left.y);
        let film_pixels = vec![FilmPixel::default(); number_of_pixels];
        FilmTile {
            bottom_left,
            top_right,
            samples,
            pixels: film_pixels,
            id,
        }
    }

    pub fn clear(&mut self) {
        self.samples = 0;
        self.pixels = vec![FilmPixel::default(); self.pixels.len()];
    }
}

/// Represents a single pixel on the film.
#[derive(Default, Copy, Clone)]
pub struct FilmPixel {
    pub contribution: Color,
    pub weight: usize,
    pub id: i32,
}