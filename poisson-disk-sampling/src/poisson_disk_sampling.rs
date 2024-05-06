/**************************************************************************
 * Poisson disk sampling in a rectangular region
 * Based on paper by Robert Bridson, University of British Columbia
 * Fast Poisson Disk Sampling in Arbitrary Dimensions
 * https://www.cs.ubc.ca/~rbridson/docs/bridson-siggraph07-poissondisk.pdf
 *************************************************************************/

use macroquad::rand::gen_range;
// the rand function from the Macroquad library has cross platform support

// how many times to try and place a point before rejection
// this is k in the paper: Fast Poisson Disk Sampling in Arbitrary Dimensions
const MAX_ATTEMPTS:i32 = 30;

fn distance_squared(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    return (x1 - y1) * (x1 - y1) + (x2 - y2) * (x2 - y2)
}

fn screen_to_grid(x:f32, y:f32, cell_size:f32) -> (i32, i32) {
    return ((y / cell_size) as i32, (x / cell_size) as i32)
}

fn new_sample_point(x: f32, y:f32, radius:f32,angle_in_rad:f32) -> (f32, f32) {
    return ( (x + angle_in_rad.cos() * radius) as f32, (y + angle_in_rad.sin() * radius) as f32)
}

fn cell_in_grid(row:i32, col:i32, grid_rows:i32, grid_cols:i32) -> bool {

    if 0 <= col && col < grid_cols && 0 <= row && row < grid_rows {
        return true;
    }

    else {
        return false;
    }
}

fn point_in_rect(x:f32, y:f32, width:f32, height:f32) -> bool {

    if 0.0f32 < x && width > x && 0.0f32 < y && y < height {
        return true;
}
    else {
        return false;
    }
}

fn grid_to_index(row: i32, column:i32, grid_columns:i32) -> i32 {
    return column + row * grid_columns;
}

// fn index_to_grid(index:i32, columns:i32) -> (i32, i32)
// {
//     let row:i32 = index / columns;
//     let col:i32 = index % columns;
//     return (row, col);
// }

fn screen_to_index(x:f32, y:f32, cell_size:f32, grid_columns:i32) -> i32 {
    let grid_pos = screen_to_grid(x, y, cell_size);
    return grid_to_index(grid_pos.0, grid_pos.1, grid_columns);
}

pub fn generate_points(radius:f32, region:(f32, f32))->Vec<(f32, f32)> {
    // STEP 0: Initialization
    const DIST: f32 = 25.0f32;
    let cell_size = radius / 2.0f32.sqrt();
    
    let rows:i32 = (region.1 / cell_size).ceil() as i32;
    let cols:i32 = (region.0 / cell_size).ceil() as i32;
    let grid_size:i32 = rows * cols;
    
    let mut grid:Vec<i32> = Vec::new();
    for _i in 0..grid_size {
        grid.push(-1);
    }
        
    let mut active_list:Vec<(f32, f32)> = Vec::new();
    let mut points:Vec<(f32, f32)> = Vec::new();
    
    // STEP 1: Select initial sample
    let initial_sample = (region.0 / 2.0f32, region.1 / 2.0f32);
    let grid_pos = screen_to_grid(initial_sample.0, initial_sample.1, cell_size);
    let initial_index = grid_to_index(grid_pos.0, grid_pos.1, cols as i32) as usize;
    
    active_list.push(initial_sample);
    points.push(initial_sample);
    grid[initial_index] = 0;
    
    // STEP 2: Generate points
    
    while active_list.len() > 0 {
        let n = gen_range(0, active_list.len());
        let sample = active_list[n];
        
        let mut remove_sample: bool = true;
        for _k in 0..MAX_ATTEMPTS {
            let angle = gen_range(0.0f32, 360.0f32).to_radians();
            let radius = gen_range(DIST, DIST * 2.0);
            let new_point = new_sample_point(sample.0, sample.1, radius, angle);
            let new_point_grid = screen_to_grid(new_point.0, new_point.1, cell_size);
            let mut point_valid:bool = true;
            
            if !point_in_rect(new_point.0, new_point.1, region.0, region.1) {
                point_valid = false;   
            }
            
            else {
                'outer: for r in -2..3 {
                    for c in -2..3 {
                        let cell_x = new_point_grid.0 + c;
                        let cell_y = new_point_grid.1 + r;
                        if cell_in_grid(cell_x, cell_y, rows, cols) {
                            let id:usize = grid_to_index(cell_x, cell_y, cols) as usize;
                            if grid[id] >= 0 {
                                let p:usize = grid[id] as usize;
                                let d = distance_squared(new_point.0, new_point.1, points.get(p).unwrap().0, points.get(p).unwrap().1);
                                if d < DIST * DIST {
                                    point_valid = false;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
            
            if point_valid {
                active_list.push(new_point);
                points.push(new_point);
                
                let i:usize = screen_to_index(new_point.0, new_point.1, cell_size, cols) as usize;
                grid[i] = points.len() as i32 - 1;
                remove_sample = false;
            }
        }
        if remove_sample {
            active_list.remove(n);
        }
    }
    return points;
}
