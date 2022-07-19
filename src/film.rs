use crate::{spectrum::Spectrum, filter::BoxFilter, image::Color};

#[derive (Clone)]
pub struct Film {
    pub filter: BoxFilter,
    pub resoloution: (usize, usize)
}

impl Film {
    pub fn new(filter: BoxFilter, resoloution: (usize, usize)) -> Film {
        Film { filter, resoloution }
    }

    pub fn get_tiles(&self, columns: usize, rows: usize) -> Vec<FilmTile> {
        let tiles = vec![];

        let pixels_per_standard_row = self.resoloution.1 / rows;
        let pixels_per_final_row = self.resoloution.1 % rows;
        let pixels_per_standard_column = self.resoloution.0 / columns;
        let pixels_per_final_column = self.resoloution.0 % columns;

        for i in 0..rows {
            for j in 0..columns {
                let row_pixels: usize;
                let column_pixels: usize;
                if i < rows - 1 {
                    row_pixels = pixels_per_standard_row
                } else {
                    row_pixels = pixels_per_final_row;
                }

                if j < columns - 1{
                    column_pixels = pixels_per_standard_column;
                } else {
                    column_pixels = pixels_per_final_column;
                }

                let bottom_left = (i * row_pixels, j * column_pixels);
                let top_right = ((i + 1) * row_pixels, (j + 1) * column_pixels);
                let tile = FilmTile::new(bottom_left, top_right, 0, 0); 
                tiles.push(tile);
            }
        }
        tiles
    }

    pub fn merge_tile(&self, tile: &FilmTile) {

    }
}

/// A subset of the film. Multiple threads can work on different tiles at the same time,
/// allowing for easy parralelism.
#[derive(Clone)]
pub struct FilmTile {
    /// The (x, y) coordinates corresponding to the bottom-left of the tile.
    pub bottom_left: (usize, usize),
    /// The (x, y) coordinates corresponding to the top-right of the tile.
    pub top_right: (usize, usize),

    pub film_pixels: Vec<FilmPixel>,
    /// Counts the number of samples integrated into the film tile.
    pub samples: usize,
    /// Every time the scene changes (e.g. due to camera movement), the id increments.
    /// Tracking this stops threads from integrating old scenes into the image.
    pub id: usize,
}

impl FilmTile {
    pub fn new(bottom_left: (usize, usize), top_right: (usize, usize), samples: usize, id: usize) -> FilmTile {
        let number_of_pixels = (top_right.0 - bottom_left.0) * (top_right.1 - bottom_left.1);
        let film_pixels = vec![FilmPixel::default(); number_of_pixels];
        FilmTile { bottom_left, top_right, samples, film_pixels, id }
    }
}

/// Represents a single pixel on the film.
#[derive(Default, Copy, Clone)]
pub struct FilmPixel {
    pub contribution: Color,
    pub weight: usize
}