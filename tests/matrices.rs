use rath::*;

#[test]
fn test_matrix_identity() {
    let mut a = DMatrix::<4, 4>::ones();
    let b = DMatrix::<4, 4>::ones();

    a[(3, 2)] = 3.0;
}
