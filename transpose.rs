use std::fmt;

// transpose function to swap second and third indices
fn transpose (matrix:Matrix) -> Matrix {
        return Matrix {
                first: matrix.first,
                second: matrix.third,
                third: matrix.second,
                fourth: matrix.fourth
        };
}

// matrix struct
struct Matrix {
        first: f32,
        second: f32,
        third: f32,
        fourth: f32
}

// implementation for matrix output
impl std::fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})\n({}, {})\n", self.first, self.second, self.third, self.fourth)
        }
}

fn main() {
        let matrix = Matrix {
                first : 1.1,
                second : 1.2,
                third : 2.1,
                fourth : 2.2
        };

        println!("Matrix:\n{}", matrix);
        println!("Transposed:\n{}", transpose(matrix));
}
