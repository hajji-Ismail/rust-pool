#[derive(Debug)]
pub struct Matrix {
    row: [i32; 2],
    col: [i32; 2],
}

pub fn transpose(m: Matrix) -> Matrix {
    let mut matrix = Matrix {
        row : [0,0],
        col : [0,0]

    }; 
    let length = matrix.col.len();
    for i in 0..length-1 {
        matrix.row[i] = m.col[i];
        matrix.col[i]= m.row[i];


    }
    matrix
}
