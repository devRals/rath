use crate::{RowVector, number::Number};

pub fn dot<T, const N: usize>(arr1: [T; N], arr2: [T; N]) -> T
where
    T: Number,
{
    arr1.into_iter()
        .zip(arr2.into_iter())
        .map(|(a, b)| a * b)
        .sum()
}

pub const fn vec2<N: Number>(x: N, y: N) -> RowVector<2, N> {
    RowVector::from_array([[x, y]])
}
pub const fn vec3<N: Number>(x: N, y: N, z: N) -> RowVector<3, N> {
    RowVector::from_array([[x, y, z]])
}
pub const fn vec4<N: Number>(x: N, y: N, z: N, w: N) -> RowVector<4, N> {
    RowVector::from_array([[x, y, z, w]])
}
