use rand::Rng;
use std::fmt;

fn rows<T>(grid: &Vec<Vec<T>>) -> usize {
    grid.len()
}

fn cols<T>(grid: &Vec<Vec<T>>) -> usize {
    grid[0].len()
}

#[derive(Clone, Default)]
pub struct Board {
    pub cells: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(cells: Vec<Vec<bool>>) -> Board {
        Board { cells }
    }

    pub fn rand_init(rows: usize, cols: usize, init_probability: f64) -> Board {
        let mut cells = vec![vec![false; cols]; rows];

        let mut rng = rand::thread_rng();
        for i in 0..(rows - 1) {
            for j in 0..(cols - 1) {
                let x: f64 = rng.gen();
                cells[i][j] = x < init_probability;
            }
        }
        Board::new(cells)
    }

    pub fn clear(&mut self) {
        self.cells = Vec::new();
    }

    /// Returns a matrix Nij, where Nij is the number of neighbors of cell at position i, j
    fn get_neighbor_count(&self) -> Vec<Vec<u32>> {
        fn add_neighbors(row: usize, col: usize, count: &mut Vec<Vec<u32>>) {
            let window_width = 3;
            for i in 0..window_width {
                for j in 0..window_width {
                    // Subtract to start index from 1
                    let r = (row + i) as i32 - 1;
                    let c = (col + j) as i32 - 1;
                    if r as usize == row && c as usize == col {
                        continue;
                    }
                    let r = (r).rem_euclid(rows(count) as i32) as usize;
                    let c = (c).rem_euclid(cols(count) as i32) as usize;
                    count[r][c] += 1;
                }
            }
        }
        let mut count = vec![vec![0; cols(&self.cells)]; rows(&self.cells)];
        for i in 0..rows(&self.cells) {
            for j in 0..cols(&self.cells) {
                if self.cells[i][j] {
                    add_neighbors(i, j, &mut count);
                }
            }
        }
        count
    }

    /// Returns a new board with the standard game of life updates
    pub fn update(&self) -> Self {
        if self.cells.is_empty() {
            return (*self).clone();
        }
        let neighbor_count = self.get_neighbor_count();
        let mut cells = vec![vec![false; cols(&self.cells)]; rows(&self.cells)];
        for i in 0..rows(&self.cells) {
            for j in 0..cols(&self.cells) {
                let neighbors = neighbor_count[i][j];
                // Survive if not alone and not too crowded
                if self.cells[i][j] && neighbors >= 2 && neighbors <= 3 {
                    cells[i][j] = true;
                }
                // Life by reproduction
                if !self.cells[i][j] && neighbors == 3 {
                    cells[i][j] = true;
                }
            }
        }
        Board {
            cells,
            ..(*self).clone()
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for &cell in row {
                if cell {
                    write!(f, "■")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::Board;

    #[test]
    fn neighbors() {
        let cols = 4;
        let rows = 4;
        let mut cells = vec![vec![false; cols]; rows];
        cells[0][1] = true;
        cells[1][1] = true;

        let board = Board::new(cells);
        let neighbors = board.get_neighbor_count();
        assert_eq!(neighbors[0][0], 2);
        assert_eq!(neighbors[0][2], 2);
        assert_eq!(neighbors[1][0], 2);
        assert_eq!(neighbors[0][3], 0);
        assert_eq!(neighbors[3][0], 1);
        assert_eq!(neighbors[3][1], 1);
    }

    #[test]
    fn dead_cell_with_3_neighbors_comes_to_life() {
        let cols = 4;
        let rows = 4;
        let mut cells = vec![vec![false; cols]; rows];
        cells[0][1] = true;
        cells[0][0] = true;
        cells[1][0] = true;

        let board = Board::new(cells);

        let neighbors = board.get_neighbor_count();
        assert_eq!(neighbors[1][1], 3);

        let board = board.update();
        assert!(board.cells[1][1]);
    }

    #[test]
    fn cell_with_too_few_neighbors_dies() {
        let cols = 4;
        let rows = 4;
        let mut cells = vec![vec![false; cols]; rows];
        cells[0][0] = true;

        let board = Board::new(cells);

        let board = board.update();
        assert!(!board.cells[0][0]);
    }

    #[test]
    fn cell_with_too_many_neighbors_dies() {
        let cols = 4;
        let rows = 4;
        let mut cells = vec![vec![false; cols]; rows];
        cells[0][0] = true;
        cells[0][1] = true;
        cells[1][0] = true;
        cells[1][1] = true;
        cells[3][0] = true;

        let board = Board::new(cells);

        let board = board.update();
        assert!(!board.cells[0][0]);
    }
}
