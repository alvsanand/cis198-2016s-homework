/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let mut mat_result: Matrix = vec![];

    let size_mat1 = (mat1.len() as i32, mat1[0].len() as i32);
    let size_mat2 = (mat2.len() as i32, mat2[0].len() as i32);

    for i in 0..size_mat1.0 {
        let mut new_vec: Vec<f32> = vec![];

        for j in 0..size_mat2.1 {
            let mut x: f32 = 0.0;

            for k in 0..size_mat1.1 {
                x = x + mat1[i as usize][k as usize] * mat2[k as usize][j as usize];
            }

            new_vec.push(x);
        }
        mat_result.push(new_vec);
    }

    mat_result
}
