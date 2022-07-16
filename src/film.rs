use crate::{spectrum::Spectrum, filter::BoxFilter};

pub struct Film {
    pub filter: BoxFilter,
    pub resoloution: (usize, usize),
    pub tiles: Vec<FilmTile>
}

impl Film {

}

/// A subset of the film. Multiple threads can work on different tiles at the same time,
/// allowing for easy parralelism.
pub struct FilmTile {
    /// The (x, y) coordinates corresponding to the bottom-left of the tile.
    pub bottom_left: (usize, usize),
    /// The (x, y) coordinates corresponding to the top-right of the tile.
    pub top_right: (usize, usize)
}

/// Represents a single pixel on the film.
pub struct FilmPixel {
    pub contribution: Spectrum,
    pub weight: i32
}