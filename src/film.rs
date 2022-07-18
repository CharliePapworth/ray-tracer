use crate::{spectrum::Spectrum, filter::BoxFilter};

#[derive (Clone)]
pub struct Film {
    pub filter: BoxFilter,
    pub resoloution: (usize, usize),
    pub tiles: Vec<FilmTile>
}

impl Film {

}

/// A subset of the film. Multiple threads can work on different tiles at the same time,
/// allowing for easy parralelism.
#[derive(Copy, Clone, PartialEq)]
pub struct FilmTile {
    /// The (x, y) coordinates corresponding to the bottom-left of the tile.
    pub bottom_left: (usize, usize),
    /// The (x, y) coordinates corresponding to the top-right of the tile.
    pub top_right: (usize, usize),
    /// Counts the number of samples integrated into the film tile.
    pub samples: i32,
    /// Every time the scene changes (e.g. due to camera movement), the id increments.
    /// Tracking this stops threads from integrating old scenes into the image.
    pub id: i32,
}

impl FilmTile {
    pub fn new(bottom_left: (usize, usize), top_right: (usize, usize), samples: i32, id: i32) -> FilmTile {
        FilmTile { bottom_left, top_right, samples, id }
    }
}

/// Represents a single pixel on the film.
pub struct FilmPixel {
    pub contribution: Spectrum,
    pub weight: i32
}