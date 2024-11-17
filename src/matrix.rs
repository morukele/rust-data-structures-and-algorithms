pub fn spiral_matrix(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    // early exit for empty matrix
    let rows = matrix.len();
    if rows == 0 {
        return vec![];
    }
    let cols = matrix[0].len();
    if cols == 0 {
        return vec![];
    }

    let size = rows * cols;

    // current position
    let mut x: i8 = 0; // column coordinate
    let mut y: i8 = 0; // row coordinate
    let mut dx: i8 = 1; // column direction
    let mut dy: i8 = 0; // row direction
    let mut res = Vec::with_capacity(size);

    // get boundaries
    let mut right = (cols - 1) as i8;
    let mut left = 0;
    let mut top = 0;
    let mut bottom = (rows - 1) as i8;

    // possible directions (dx, dy), clockwise
    // right ( 1,  0)
    // down  ( 0,  1)
    // left  (-1,  0)
    // up    ( 0, -1)
    // loop until we fill the array
    for _ in 0..size {
        // add element in matrix to result
        res.push(matrix[y as usize][x as usize]);

        // check boundaries and change direction
        if dx == 1 && x == right {
            // moving right, hit the right boundary
            // change direction to down
            dx = 0;
            dy = 1;
            // shrink the matrix from the top
            top += 1;
        } else if dy == 1 && y == bottom {
            // moving down, hit the bottom boundary
            // change direction to left
            dx = -1;
            dy = 0;
            // shrink the matrix from the right
            right -= 1;
        } else if dx == -1 && x == left {
            // moving left, and hit the left boundary
            // change direction up
            dx = 0;
            dy = -1;
            // shrink the matrix from the bottom
            bottom -= 1;
        } else if dy == -1 && y == top {
            // moving up, and hit the top boundary
            // change direction right
            dx = 1;
            dy = 0;
            // shrink the matrix from the left
            left += 1;
        }

        // update directions
        x += dx;
        y += dy;
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spiral_matrix_test_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = spiral_matrix(matrix);

        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn spiral_matrix_test_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let result = spiral_matrix(matrix);

        assert_eq!(result, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }

    #[test]
    fn spiral_matrix_test_3() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];
        let result = spiral_matrix(matrix);

        assert_eq!(result, vec![1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8]);
    }
}
