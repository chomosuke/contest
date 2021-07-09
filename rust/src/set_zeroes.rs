
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {

    let mut firstRowZero = false;
    for i in 0..matrix[0].len() {
        if matrix[0][i] == 0 {
            firstRowZero = true;
            break;
        }
    }
    let mut firstColumnZero = false;
    for i in 0..matrix.len() {
        if matrix[i][0] == 0 {
            firstColumnZero = true;
        }
    }

    for i in 1..matrix.len() {
        for j in 1..matrix[0].len() {
            if matrix[i][j] == 0 {
                matrix[0][j] = 0;
                matrix[i][0] = 0;
            }
        }
    }

    for i in 1..matrix.len() {
        for j in 1..matrix[0].len() {
            if matrix[0][j] == 0 || matrix[i][0] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    if firstRowZero {
        for i in 0..matrix[0].len() {
            matrix[0][i] = 0;
        }
    }
    if firstColumnZero {
        for i in 0..matrix.len() {
            matrix[i][0] = 0;
        }
    }
}