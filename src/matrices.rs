use core::ops::*;

use crate::functions::dot;
use crate::number::Number;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Matrix<const R: usize, const C: usize, T>
where
    T: Number,
{
    data: [[T; C]; R],
}

impl<const S: usize, T: Number> Matrix<S, S, T> {
    /// Creates a new square matrix with the main diagonal values are equal to one and the rest is
    /// zero
    pub fn identity() -> Self {
        let mut matrix = Self::zeros();

        let mut i = 0;
        for rows in &mut matrix.data {
            let mut j = 0;
            for val in rows {
                if i == j {
                    *val = T::ONE;
                }

                j += 1;
            }

            i += 1;
        }

        matrix
    }
}

impl<const R: usize, const C: usize, T: Number> Matrix<R, C, T> {
    pub const fn from_array(data: [[T; C]; R]) -> Matrix<R, C, T> {
        Matrix { data }
    }

    /// Creates a new matrix with all entries equals to one
    pub const fn ones() -> Matrix<R, C, T> {
        Matrix {
            data: [[T::ONE; C]; R],
        }
    }

    /// Creates a new matrix with all entries equals to zero
    pub const fn zeros() -> Matrix<R, C, T> {
        Matrix {
            data: [[T::ZERO; C]; R],
        }
    }

    /// Returns the zero indexed `n`'th row
    pub fn nth_row(&self, n: usize) -> [T; C] {
        assert!(n >= R, "Index is out of bounds: `n > RowCount` : {n} > {R}");
        self.data[n - 1]
    }

    /// Returns zero indexed `n`'th column
    pub fn nth_column(&self, n: usize) -> [T; R] {
        assert!(
            n >= C,
            "Index is out of bounds: `n > ColumnCount` : {n} > {C}"
        );
        let mut col = [T::ZERO; R];

        for (i, x) in self.data.iter().enumerate() {
            col[i] = x[n - 1]
        }

        col
    }

    pub const fn as_flat_slice(&self) -> &[T] {
        self.data.as_flattened()
    }
    pub const fn as_flat_mut_slice(&mut self) -> &[T] {
        self.data.as_flattened_mut()
    }

    pub const fn as_array(&self) -> [[T; C]; R] {
        self.data
    }
    pub const fn as_array_mut(&mut self) -> &mut [[T; C]; R] {
        &mut self.data
    }
}

impl<const R: usize, const C: usize, N: Number> Add for Matrix<R, C, N> {
    type Output = Matrix<R, C, N>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut new_matrix = Matrix::zeros();
        for (i, (a, b)) in self.into_iter().zip(rhs).enumerate() {
            new_matrix[i] = a + b;
        }

        new_matrix
    }
}
impl<const R: usize, const C: usize, N: Number> AddAssign for Matrix<R, C, N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<const R: usize, const C: usize, N: Number> Sub for Matrix<R, C, N> {
    type Output = Matrix<R, C, N>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut new_matrix = Matrix::zeros();
        for (i, (a, b)) in self.into_iter().zip(rhs).enumerate() {
            new_matrix[i] = a - b;
        }

        new_matrix
    }
}
impl<const R: usize, const C: usize, N: Number> SubAssign for Matrix<R, C, N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<const R: usize, const C: usize, N: Number> Mul<N> for Matrix<R, C, N> {
    type Output = Matrix<R, C, N>;
    fn mul(self, rhs: N) -> Self::Output {
        let mut new_matrix = self;
        for x in &mut new_matrix {
            *x *= rhs;
        }

        new_matrix
    }
}

impl<const M: usize, const N: usize, const P: usize, T> Mul<Matrix<N, P, T>> for Matrix<M, N, T>
where
    T: Number,
{
    type Output = Matrix<M, P, T>;
    fn mul(self, rhs: Matrix<N, P, T>) -> Self::Output {
        let mut new_matrix = Matrix::zeros();
        let mut index = 0;

        for i in 1..=M {
            let lhs_row = self.nth_row(i);
            for j in 1..=P {
                let rhs_column = rhs.nth_column(j);

                new_matrix[index] = dot(lhs_row, rhs_column);
                index += 1;
            }
        }

        new_matrix
    }
}

impl<const R: usize, const C: usize, N: Number> MulAssign<N> for Matrix<R, C, N> {
    fn mul_assign(&mut self, rhs: N) {
        *self = *self * rhs
    }
}

impl<const R: usize, const C: usize, N: Number> Div<N> for Matrix<R, C, N> {
    type Output = Matrix<R, C, N>;
    fn div(self, rhs: N) -> Self::Output {
        let mut new_matrix = self;
        for x in &mut new_matrix {
            *x /= rhs;
        }

        new_matrix
    }
}
impl<const R: usize, const C: usize, N: Number> DivAssign<N> for Matrix<R, C, N> {
    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs
    }
}

impl<const R: usize, const C: usize, N: Number> Index<(usize, usize)> for Matrix<R, C, N> {
    type Output = N;
    /// 0 indexed as (row, column)
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<const R: usize, const C: usize, N: Number> Index<usize> for Matrix<R, C, N> {
    type Output = N;
    fn index(&self, index: usize) -> &Self::Output {
        let i = index / C;
        let j = index % C;
        &self.data[i][j]
    }
}

impl<const R: usize, const C: usize, N: Number> IndexMut<(usize, usize)> for Matrix<R, C, N> {
    /// 0 indexed as (row, column)
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<const R: usize, const C: usize, N: Number> IndexMut<usize> for Matrix<R, C, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = index / C;
        let j = index % C;
        &mut self.data[i][j]
    }
}

impl<const R: usize, const C: usize, N: Number> IntoIterator for Matrix<R, C, N> {
    type Item = N;
    type IntoIter = core::iter::Flatten<core::array::IntoIter<[N; C], R>>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().flatten()
    }
}

impl<'a, const R: usize, const C: usize, N: Number> IntoIterator for &'a Matrix<R, C, N> {
    type Item = &'a N;
    type IntoIter = core::iter::Flatten<core::slice::Iter<'a, [N; C]>>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter().flatten()
    }
}

impl<'a, const R: usize, const C: usize, N: Number> IntoIterator for &'a mut Matrix<R, C, N> {
    type Item = &'a mut N;
    type IntoIter = core::iter::Flatten<core::slice::IterMut<'a, [N; C]>>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut().flatten()
    }
}

pub type FMatrix<const R: usize, const C: usize> = Matrix<R, C, f32>;
pub type DMatrix<const R: usize, const C: usize> = Matrix<R, C, f64>;
pub type IMatrix<const R: usize, const C: usize> = Matrix<R, C, i64>;
pub type UMatrix<const R: usize, const C: usize> = Matrix<R, C, u64>;
pub type ByteMatrix<const R: usize, const C: usize> = Matrix<R, C, u8>;

pub type SquareMatrix<const N: usize, T> = Matrix<N, N, T>;

pub type RowVector<const S: usize, T> = Matrix<1, S, T>;
pub type ColumnVector<const S: usize, T> = Matrix<S, 1, T>;

pub type FRVector<const S: usize> = RowVector<S, f32>;
pub type DRVector<const S: usize> = RowVector<S, f64>;

pub type Vector2 = RowVector<2, f64>;
pub type Vector3 = RowVector<3, f64>;
pub type Vector4 = RowVector<4, f64>;

pub type FVector2 = RowVector<2, f32>;
pub type FVector3 = RowVector<3, f32>;
pub type FVector4 = RowVector<4, f32>;
