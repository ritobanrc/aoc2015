use std::ops::{Index, IndexMut};

/// Stores a 2D array of values in row-major order
pub struct Grid2D<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid2D<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        Self {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }
}

impl<T> Grid2D<T> {
    pub fn iter(&self) -> Grid2DIter<T> {
        Grid2DIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

impl<T, I> Index<I> for Grid2D<T>
where
    I: Into<(usize, usize)>,
{
    type Output = T;
    fn index(&self, coord: I) -> &Self::Output {
        let coord = coord.into();
        assert!(coord.0 < self.width);
        assert!(coord.1 < self.height);
        &self.data[coord.0 + coord.1 * self.width]
    }
}

impl<T, I> IndexMut<I> for Grid2D<T>
where
    I: Into<(usize, usize)>,
{
    fn index_mut(&mut self, coord: I) -> &mut Self::Output {
        let coord = coord.into();
        assert!(coord.0 < self.width);
        assert!(coord.1 < self.height);
        &mut self.data[coord.0 + coord.1 * self.width]
    }
}

pub struct Grid2DIter<'a, T> {
    grid: &'a Grid2D<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for Grid2DIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        if self.x >= self.grid.width {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= self.grid.height {
            None
        } else {
            Some((self.x, self.y, &self.grid[(self.x, self.y)]))
        }
    }
}
