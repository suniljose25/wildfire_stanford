use fastrand;
use nannou::prelude::*;

const SIZE: usize = 25 + 2;
const SCALE: f32 = 20.0;

fn main() {
    let side = (SIZE.to_f32().unwrap() * SCALE) as u32;
    nannou::app(model)
        .size(side, side)
        .update(update).simple_window(view).run();
}

#[derive(Clone, Copy)]
enum CellStatus {
    Empty,
    Tree,
    Burning,
}

struct Model {
    _size: usize,
    grid: Vec<Vec<CellStatus>>,
}

fn model(_app: &App) -> Model {
    let grid_size = SIZE;

    let mut grid: Vec<Vec<CellStatus>> = vec![];
    for i in 0..grid_size {
        let mut row: Vec<CellStatus> = vec![];
        for j in 0..grid_size {
            if i == 0 || i == grid_size - 1 || j == 0 || j == grid_size - 1 {
                row.push(CellStatus::Empty);
                continue;
            }
            row.push(CellStatus::Tree);
        }
        grid.push(row);
    }

    grid[grid_size / 2][grid_size / 2] = CellStatus::Burning;

    Model { _size: grid_size, grid }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let mut new_grid = _model.grid.clone();

    for (i, row) in _model.grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            new_grid[i][j] = match cell {
                CellStatus::Empty => CellStatus::Empty,
                CellStatus::Tree => {
                    let neighbors = get_neighbors(_model, i, j);
                    spread(&cell, neighbors)
                }
                CellStatus::Burning => CellStatus::Empty
            }
        }
    }

    _model.grid = new_grid;
}

fn spread(site: &CellStatus, neighbors: Vec<CellStatus>) -> CellStatus {
    match site {
        CellStatus::Empty => CellStatus::Empty,
        CellStatus::Tree => {
            if neighbors.iter().any(|&x| {
                match x {
                    CellStatus::Burning => true,
                    _ => false
                }
            }) {
                let prob_catch: f32 = 0.5;
                let rand: f32 = fastrand::f32();
                if rand < prob_catch {
                    CellStatus::Burning
                } else {
                    CellStatus::Tree
                }
            } else {
                CellStatus::Tree
            }
        }
        CellStatus::Burning => CellStatus::Empty
    }
}

fn get_neighbors(_model: &Model, x: usize, y: usize) -> Vec<CellStatus> {
    let mut neighbor_cells: Vec<CellStatus> = vec![];
    
    if x > 0 {
        neighbor_cells.push(_model.grid[x - 1][y]);
    }
    if x < _model._size - 1 {
        neighbor_cells.push(_model.grid[x + 1][y]);
    }
    if y > 0 {
        neighbor_cells.push(_model.grid[x][y - 1]);
    }
    if y < _model._size - 1 {
        neighbor_cells.push(_model.grid[x][y + 1]);
    }

    neighbor_cells
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = _app.draw();
    let x_offset = _app.window_rect().w() / 2.0 - SCALE / 2.0;
    let y_offset = _app.window_rect().h() / 2.0 - SCALE / 2.0;

    for (i, row) in _model.grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let x = i as f32 * SCALE;
            let y = j as f32 * SCALE;
            match cell {
                CellStatus::Empty => {
                    draw.rect()
                        .x_y(x - x_offset, y - y_offset)
                        .w_h(SCALE, SCALE)
                        .color(YELLOW);
                }
                CellStatus::Tree => {
                    draw.rect()
                        .x_y(x - x_offset, y - y_offset)
                        .w_h(SCALE, SCALE)
                        .color(GREEN);
                }
                CellStatus::Burning => {
                    draw.rect()
                        .x_y(x - x_offset, y - y_offset)
                        .w_h(SCALE, SCALE)
                        .color(RED);
                }
            }
        }
    }

    draw.to_frame(_app, &frame).unwrap();
}
