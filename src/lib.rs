#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // If size is 0, we return an empty vector
    if size == 0 {
        return Vec::new();
    }

    // zero matrix
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    
    // initial direction and starting position
    let mut direction = Direction::Right;
    let mut row = 0;
    let mut col = 0;

    // matrix boundaries
    let (mut top, mut bottom, mut left, mut right) = (0, size as usize - 1, 0, size as usize - 1);

    // Filling the matrix with numbers in spiral order
    for num in 1..=size*size {
        matrix[row][col] = num as u32;

        // Moving to the next position based on the current direction
        match direction {
            Direction::Right => {
                if col == right {
                    row += 1;
                    direction = Direction::Down;
                    top += 1;
                } else {
                    col += 1;
                }
            }
            Direction::Down => {
                if row == bottom {
                    col -= 1;
                    direction = Direction::Left;
                    right -= 1;
                } else {
                    row += 1;
                }
            }
            Direction::Left => {
                if col == left {
                    row -= 1;
                    direction = Direction::Up;
                    bottom -= 1;
                } else {
                    col -= 1;
                }
            }
            Direction::Up => {
                if row == top {
                    col += 1;
                    direction = Direction::Right;
                    left += 1;
                } else {
                    row -= 1;
                }
            }
        }
    }
    
    matrix
}
