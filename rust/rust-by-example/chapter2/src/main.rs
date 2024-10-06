use core::fmt;
use std::mem;

fn main() {
    struct Matrix(f32, f32, f32, f32);
    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }
    fn transpose(matrix: &Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(&matrix));

    let xs = [1, 2, 3, 4, 5];
    println!("{}", mem::size_of_val(&xs));
    println!("{:?}", &xs[1..4])
}
