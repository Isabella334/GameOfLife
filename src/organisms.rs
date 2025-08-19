pub fn add_block(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x] = true;
    grid[y][x + 1] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 1] = true;
}

pub fn add_blinker(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x] = true;
    grid[y+1][x] = true;
    grid[y+2][x] = true;
}

pub fn add_glider(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x-1] = true;
    grid[y+1][x] = true;
    grid[y+1][x+1] = true;
    grid[y+2][x-1] = true;
    grid[y+2][x] = true;
}

pub fn add_beehive(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x-1] = true;
    grid[y-1][x] = true;
    grid[y-1][x+1] = true;
    grid[y][x+2] = true;
    grid[y+1][x] = true;
    grid[y+1][x+1] = true;
}

pub fn add_toad(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x-1] = true;
    grid[y+1][x-1] = true;
    grid[y+2][x] = true;
    grid[y-1][x+1] = true;
    grid[y][x+2] = true;
    grid[y+1][x+2] = true;
}

pub fn add_loaf(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x] = true;
    grid[y-1][x+1] = true;
    grid[y+1][x+1] = true;
    grid[y-1][x+2] = true;
    grid[y+2][x+2] = true;
    grid[y][x+3] = true;
    grid[y+1][x+3] = true;
}

pub fn add_beacon(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x] = true;
    grid[y-1][x] = true;
    grid[y-1][x-1] = true;
    grid[y][x-1] = true;

    grid[y+2][x+2] = true;
    grid[y+1][x+2] = true;
    grid[y+1][x+1] = true;
    grid[y+2][x+1] = true;
}

pub fn add_lwss(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x] = true;
    grid[y-1][x] = true;
    grid[y-1][x-1] = true;
    grid[y][x-1] = true;
    grid[y-1][x+1] = true;
    grid[y-2][x+1] = true;
    grid[y-2][x] = true;
    grid[y-1][x+2] = true;
    grid[y][x+2] = true;
    grid[y][x+3] = true;
    grid[y+1][x+1] = true;
    grid[y+1][x+2] = true;
}

pub fn add_boat(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x] = true;
    grid[y-1][x] = true;
    grid[y-1][x+1] = true;
    grid[y][x+2] = true;
    grid[y+1][x+1] = true;
}

pub fn add_tub(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    grid[y][x] = true;
    grid[y-1][x+1] = true;
    grid[y][x+2] = true;
    grid[y+1][x+1] = true;
}