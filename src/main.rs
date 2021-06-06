use std::collections::VecDeque;
use std::fmt;

enum Error {
    InvalidGrid,
}

#[derive(Copy, Clone)]
enum SudokuCell {
    Solved(u16),
    Unsolved(u16),
}

type SudokuGrid = Vec<Vec<SudokuCell>>;

impl SudokuCell {
    fn is_solved(&self) -> bool {
        match *self {
            SudokuCell::Solved(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for SudokuCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SudokuCell::Solved(value) => write!(f, "{}", value),
            SudokuCell::Unsolved(_) => write!(f, "0")
        }
    }
}

struct SudokuResult(Result<bool, Error>);

impl SudokuResult {
    fn is_changed(&self) -> bool {
        match self.0 {
            Ok(true) => true,
            _ => false,
        }
    }
}

fn check_grid(grid: &mut SudokuGrid) -> SudokuResult {
    let mut changed = false;

    for row in 0..9 {
        for column in 0..9 {
            let mut update_to = 0;
            let item = &grid[row][column];
            let mut possibilities;

            match *item {
                SudokuCell::Solved(_) => continue,
                SudokuCell::Unsolved(p) => {
                    possibilities = p;

                    for r in 0..9 {
                        if let SudokuCell::Solved(value) = grid[r][column] {
                            let v = value - 1;

                            let bit = (possibilities << (15 - v)) >> 15;
                            if bit == 1 {
                                possibilities -= 1 << v;
                                changed = true;
                            }

                        }
                    }

                    for c in 0..9 {
                        if let SudokuCell::Solved(value) = grid[row][c] {
                            let v = value - 1;

                            let bit = (possibilities << (15 - v)) >> 15;
                            if bit == 1 {
                                possibilities -= 1 << v;
                                changed = true;
                            }
                        }
                    }

                    let row_offset = row / 3;
                    let column_offset = column / 3;

                    for r in (row_offset * 3)..(row_offset * 3 + 3) {
                        for c in (column_offset * 3)..(column_offset * 3 + 3) {
                            if let SudokuCell::Solved(value) = grid[r][c] {
                                let v = value - 1;
    
                                let bit = (possibilities << (15 - v)) >> 15;
                                if bit == 1 {
                                    possibilities -= 1 << v;
                                    changed = true;
                                }
                            }
                        }
                    }

                    if possibilities.count_ones() == 1 {
                        let mut index = 0;
                        while possibilities > 1 {
                            possibilities = possibilities >> 1;
                            index += 1;
                        }

                        update_to = index + 1;
                    } else if possibilities.count_ones() == 0 {
                        return SudokuResult(Err(Error::InvalidGrid));
                    }
                }
            }

            if update_to != 0 {
                grid[row][column] = SudokuCell::Solved(update_to);
            } else {
                grid[row][column] = SudokuCell::Unsolved(possibilities);
            }
        }
    }

    SudokuResult(Ok(changed))
}

fn clone_grid(grid: &SudokuGrid) -> SudokuGrid {
    let mut new_grid = Vec::new();

    for row in 0..9 {
        let mut new_row = Vec::new();
        for column in 0..9 {
            new_row.push(grid[row][column]);
        }
        new_grid.push(new_row);
    }

    new_grid
}

fn check_solved(grid: &SudokuGrid) -> bool {
    for row in 0..9 {
        for column in 0..9 {
            if !grid[row][column].is_solved() {
                return false;
            }
        }
    }

    true
}

fn suggest_guess(grid: &SudokuGrid) -> (u16, usize, usize) {
    for row in 0..9 {
        for column in 0..9 {
            if let SudokuCell::Unsolved(possibilities) = grid[row][column] {
                let mut index = 0;
                while (possibilities << (15 - index)) >> 15 != 1 {
                    index += 1;
                }

                return (index + 1, row, column);
            }
        }
    }

    (10, 10, 10)
}

fn print_grid(grid: &SudokuGrid) {
    for row in grid {
        for item in row {
            print!("{} ", item);
        }
        println!();
    }
    println!();
}

fn remove_possibility(grid: &mut SudokuGrid, guess: (u16, usize, usize)) {
    let mut update_to = 0;
    let mut possibilities;

    match grid[guess.1][guess.2] {
        SudokuCell::Solved(_) => return,
        SudokuCell::Unsolved(p) => {
            possibilities = p;

            if possibilities.count_ones() > 1 {
                let v = guess.0 - 1;
                possibilities -= 1 << v;
            } else {
                let mut index = 0;
                while possibilities > 1 {
                    possibilities = possibilities >> 1;
                    index += 1;
                }

                update_to = index + 1;
            }
        }
    }

    if update_to != 0 {
        grid[guess.1][guess.2] = SudokuCell::Solved(update_to);
    } else {
        grid[guess.1][guess.2] = SudokuCell::Unsolved(possibilities);
    }
}

fn main() {
    // let grid: [[u16; 9]; 9] = [
    //     [0, 0, 0, 1, 0, 0, 3, 9, 4],
    //     [2, 0, 0, 0, 9, 0, 0, 0, 6],
    //     [5, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 2, 0, 8, 0, 7, 0, 0, 0],
    //     [4, 0, 0, 0, 5, 0, 0, 3, 0],
    //     [8, 6, 3, 0, 0, 0, 0, 7, 0],
    //     [0, 0, 0, 0, 0, 6, 2, 8, 3],
    //     [1, 0, 0, 0, 0, 3, 0, 5, 9],
    //     [0, 0, 0, 0, 8, 0, 0, 0, 0],
    // ];

    let grid: [[u16; 9]; 9] = [
        [8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 6, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 9, 0, 2, 0, 0],
        [0, 5, 0, 0, 0, 7, 0, 0, 0],
        [0, 0, 0, 0, 4, 5, 7, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 3, 0],
        [0, 0, 1, 0, 0, 0, 0, 6, 8],
        [0, 0, 8, 5, 0, 0, 0, 1, 0],
        [0, 9, 0, 0, 0, 0, 4, 0, 0],
    ];

    let default_possibilities = 0b111111111;

    let mut sudoku_grid: SudokuGrid = grid.iter().map(|row| {
        row.iter().map(|value| {
            if *value == 0 {
                SudokuCell::Unsolved(default_possibilities)
            } else {
                SudokuCell::Solved(*value)
            }
        }).collect()
    }).collect();

    while check_grid(&mut sudoku_grid).is_changed() { }

    if check_solved(&sudoku_grid) {
        print_grid(&sudoku_grid);
        return;
    }

    let mut queue = VecDeque::new();
    let mut next_grid = clone_grid(&sudoku_grid);
    let mut guess_details = suggest_guess(&next_grid);
    queue.push_front((sudoku_grid, guess_details));

    loop {
        next_grid[guess_details.1][guess_details.2] = SudokuCell::Solved(guess_details.0);

        let mut valid = true;
        loop {
            let status = check_grid(&mut next_grid);

            match status.0 {
                Ok(false) => {
                    break;
                },
                Ok(true) => {
                    continue;
                },
                Err(_) => {
                    valid = false;
                    break;
                }
            }
        }

        if valid && check_solved(&next_grid) {
            print_grid(&next_grid);
            return;
        }

        if !valid {
            let mut top_grid = queue.pop_front().unwrap();
            remove_possibility(&mut top_grid.0, top_grid.1);

            loop {
                let mut inner_valid = true;
                loop {
                    let status = check_grid(&mut top_grid.0);
        
                    match status.0 {
                        Ok(false) => {
                            break;
                        },
                        Ok(true) => {
                            continue;
                        },
                        Err(_) => {
                            inner_valid = false;
                            break;
                        }
                    }
                }

                if inner_valid {
                    break;
                } else {
                    top_grid = queue.pop_front().unwrap();
                    remove_possibility(&mut top_grid.0, top_grid.1);
                }
            }

            next_grid = clone_grid(&top_grid.0);
            guess_details = suggest_guess(&next_grid);
            queue.push_front((top_grid.0, guess_details));
        } else {
            let next_next_grid = clone_grid(&next_grid);
            queue.push_front((next_grid, guess_details));
            next_grid = next_next_grid;
            guess_details = suggest_guess(&next_grid);
        }
    }

}
