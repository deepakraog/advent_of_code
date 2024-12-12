pub type Grid = Vec<Vec<char>>;

pub trait ToGrid {
    fn to_grid(&self) -> Grid;
}

impl ToGrid for str {
    fn to_grid(&self) -> Grid {
        self.lines().map(|line| line.chars().collect()).collect()
    }
}
pub fn in_bounds(x: i32, y: i32, grid: &Grid) -> bool {
    y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[y as usize].len() as i32
}
