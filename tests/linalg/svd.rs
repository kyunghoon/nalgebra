use na::{DMatrix, Matrix6};

#[cfg(feature = "proptest-support")]
mod proptest_tests {
    macro_rules! gen_tests(
        ($module: ident, $scalar: expr, $scalar_type: ty) => {
            mod $module {
                use na::{
                    DMatrix, DVector, Matrix2, Matrix3, Matrix4,
                    ComplexField
                };
                use std::cmp;
                #[allow(unused_imports)]
                use crate::core::helper::{RandScalar, RandComplex};
                use crate::proptest::*;
                use proptest::{prop_assert, proptest};

                proptest! {
                    #[test]
                    fn svd(m in dmatrix_($scalar)) {
                        let svd = m.clone().svd(true, true);
                        let recomp_m = svd.clone().recompose().unwrap();
                        let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
                        let ds = DMatrix::from_diagonal(&s.map(|e| ComplexField::from_real(e)));

                        prop_assert!(s.iter().all(|e| *e >= 0.0));
                        prop_assert!(relative_eq!(&u * ds * &v_t, recomp_m, epsilon = 1.0e-5));
                        prop_assert!(relative_eq!(m, recomp_m, epsilon = 1.0e-5));
                    }

                    #[test]
                    fn svd_static_5_3(m in matrix5x3_($scalar)) {
                        let svd = m.svd(true, true);
                        let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
                        let ds = Matrix3::from_diagonal(&s.map(|e| ComplexField::from_real(e)));

                        prop_assert!(s.iter().all(|e| *e >= 0.0));
                        prop_assert!(relative_eq!(m, &u * ds * &v_t, epsilon = 1.0e-5));
                        prop_assert!(u.is_orthogonal(1.0e-5));
                        prop_assert!(v_t.is_orthogonal(1.0e-5));
                    }

                    #[test]
                    fn svd_static_5_2(m in matrix5x2_($scalar)) {
                        let svd = m.svd(true, true);
                        let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
                        let ds = Matrix2::from_diagonal(&s.map(|e| ComplexField::from_real(e)));

                        prop_assert!(s.iter().all(|e| *e >= 0.0));
                        prop_assert!(relative_eq!(m, &u * ds * &v_t, epsilon = 1.0e-5));
                        prop_assert!(u.is_orthogonal(1.0e-5));
                        prop_assert!(v_t.is_orthogonal(1.0e-5));
                    }

                    #[test]
                    fn svd_static_3_5(m in matrix3x5_($scalar)) {
                        let svd = m.svd(true, true);
                        let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());

                        let ds = Matrix3::from_diagonal(&s.map(|e| ComplexField::from_real(e)));

                        prop_assert!(s.iter().all(|e| *e >= 0.0));
                        prop_assert!(relative_eq!(m, u * ds * v_t, epsilon = 1.0e-5));
                    }

                    #[test]
                    fn svd_static_2_5(m in matrix2x5_($scalar)) {
                        let svd = m.svd(true, true);
                        let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
                        let ds = Matrix2::from_diagonal(&s.map(|e| ComplexField::from_real(e)));

                        prop_assert!(s.iter().all(|e| *e >= 0.0));
                        prop_assert!(relative_eq!(m, u * ds * v_t, epsilon = 1.0e-5));
                    }

                    #[test]
                    fn svd_static_square(m in matrix4_($scalar)) {
                        let svd = m.svd(true, true);
                        let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
                        let ds = Matrix4::from_diagonal(&s.map(|e| ComplexField::from_real(e)));

                        prop_assert!(s.iter().all(|e| *e >= 0.0));
                        prop_assert!(relative_eq!(m, u * ds * v_t, epsilon = 1.0e-5));
                        prop_assert!(u.is_orthogonal(1.0e-5));
                        prop_assert!(v_t.is_orthogonal(1.0e-5));
                    }

                    #[test]
                    fn svd_static_square_2x2(m in matrix2_($scalar)) {
                        let svd = m.svd(true, true);
                        let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
                        let ds = Matrix2::from_diagonal(&s.map(|e| ComplexField::from_real(e)));

                        prop_assert!(s.iter().all(|e| *e >= 0.0));
                        prop_assert!(relative_eq!(m, u * ds * v_t, epsilon = 1.0e-5));
                        prop_assert!(u.is_orthogonal(1.0e-5));
                        prop_assert!(v_t.is_orthogonal(1.0e-5));
                    }

                    #[test]
                    fn svd_pseudo_inverse(m in dmatrix_($scalar)) {
                        let svd = m.clone().svd(true, true);
                        let pinv = svd.pseudo_inverse(1.0e-10).unwrap();

                        if m.nrows() > m.ncols() {
                            prop_assert!((pinv * m).is_identity(1.0e-5))
                        } else {
                            prop_assert!((m * pinv).is_identity(1.0e-5))
                        }
                    }

                    #[test]
                    fn svd_solve(n in PROPTEST_MATRIX_DIM, nb in PROPTEST_MATRIX_DIM) {
                        let n = cmp::max(1, cmp::min(n, 10));
                        let nb = cmp::min(nb, 10);
                        let m  = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0);

                        let svd = m.clone().svd(true, true);

                        if svd.rank(1.0e-7) == n {
                            let b1 = DVector::<$scalar_type>::new_random(n).map(|e| e.0);
                            let b2 = DMatrix::<$scalar_type>::new_random(n, nb).map(|e| e.0);

                            let sol1 = svd.solve(&b1, 1.0e-7).unwrap();
                            let sol2 = svd.solve(&b2, 1.0e-7).unwrap();

                            let recomp = svd.recompose().unwrap();

                            prop_assert!(relative_eq!(m, recomp, epsilon = 1.0e-6));
                            prop_assert!(relative_eq!(&m * &sol1, b1, epsilon = 1.0e-6));
                            prop_assert!(relative_eq!(&m * &sol2, b2, epsilon = 1.0e-6));
                        }
                    }
                }
            }
        }
    );

    gen_tests!(complex, complex_f64(), RandComplex<f64>);
    gen_tests!(f64, PROPTEST_F64, RandScalar<f64>);
}

// Test proposed on the issue #176 of rulinalg.
#[test]
#[rustfmt::skip]
fn svd_singular() {
    let m = DMatrix::from_row_slice(24, 24, &[
        1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  1.0,  1.0,
        0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0]);

    let svd = m.clone().svd(true, true);
    let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
    let ds = DMatrix::from_diagonal(&s);

    assert!(s.iter().all(|e| *e >= 0.0));
    assert!(u.is_orthogonal(1.0e-5));
    assert!(v_t.is_orthogonal(1.0e-5));
    assert_relative_eq!(m, &u * ds * &v_t, epsilon = 1.0e-5);
}

// Same as the previous test but with one additional row.
#[test]
#[rustfmt::skip]
fn svd_singular_vertical() {
    let m = DMatrix::from_row_slice(25, 24, &[
        1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  1.0,  1.0,
        0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0]);


    let svd = m.clone().svd(true, true);
    let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
    let ds = DMatrix::from_diagonal(&s);

    assert!(s.iter().all(|e| *e >= 0.0));
    assert_relative_eq!(m, &u * ds * &v_t, epsilon = 1.0e-5);
}

// Same as the previous test but with one additional column.
#[test]
#[rustfmt::skip]
fn svd_singular_horizontal() {
    let m = DMatrix::from_row_slice(24, 25, &[
        1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  1.0,  1.0,   1.0,
        0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,   0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, 0.0]);

    let svd = m.clone().svd(true, true);
    let (u, s, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());
    let ds = DMatrix::from_diagonal(&s);

    assert!(s.iter().all(|e| *e >= 0.0));
    assert_relative_eq!(m, &u * ds * &v_t, epsilon = 1.0e-5);
}

#[test]
fn svd_zeros() {
    let m = DMatrix::from_element(10, 10, 0.0);
    let svd = m.clone().svd(true, true);
    assert_eq!(Ok(m), svd.recompose());
}

#[test]
fn svd_identity() {
    let m = DMatrix::<f64>::identity(10, 10);
    let svd = m.clone().svd(true, true);
    assert_eq!(Ok(m), svd.recompose());

    let m = DMatrix::<f64>::identity(10, 15);
    let svd = m.clone().svd(true, true);
    assert_eq!(Ok(m), svd.recompose());

    let m = DMatrix::<f64>::identity(15, 10);
    let svd = m.clone().svd(true, true);
    assert_eq!(Ok(m), svd.recompose());
}

#[test]
#[rustfmt::skip]
fn svd_with_delimited_subproblem() {
    let mut m = DMatrix::<f64>::from_element(10, 10, 0.0);
    m[(0, 0)] = 1.0;  m[(0, 1)] = 2.0;
    m[(1, 1)] = 0.0;  m[(1, 2)] = 3.0;
    m[(2, 2)] = 4.0;  m[(2, 3)] = 5.0;
    m[(3, 3)] = 6.0;  m[(3, 4)] = 0.0;
    m[(4, 4)] = 8.0;  m[(3, 5)] = 9.0;
    m[(5, 5)] = 10.0; m[(3, 6)] = 11.0;
    m[(6, 6)] = 12.0; m[(3, 7)] = 12.0;
    m[(7, 7)] = 14.0; m[(3, 8)] = 13.0;
    m[(8, 8)] = 16.0; m[(3, 9)] = 17.0;
    m[(9, 9)] = 18.0;
    let svd = m.clone().svd(true, true);
    assert_relative_eq!(m, svd.recompose().unwrap(), epsilon = 1.0e-7);

    // Rectangular versions.
    let mut m = DMatrix::<f64>::from_element(15, 10, 0.0);
    m[(0, 0)] = 1.0;  m[(0, 1)] = 2.0;
    m[(1, 1)] = 0.0;  m[(1, 2)] = 3.0;
    m[(2, 2)] = 4.0;  m[(2, 3)] = 5.0;
    m[(3, 3)] = 6.0;  m[(3, 4)] = 0.0;
    m[(4, 4)] = 8.0;  m[(3, 5)] = 9.0;
    m[(5, 5)] = 10.0; m[(3, 6)] = 11.0;
    m[(6, 6)] = 12.0; m[(3, 7)] = 12.0;
    m[(7, 7)] = 14.0; m[(3, 8)] = 13.0;
    m[(8, 8)] = 16.0; m[(3, 9)] = 17.0;
    m[(9, 9)] = 18.0;
    let svd = m.clone().svd(true, true);
    assert_relative_eq!(m, svd.recompose().unwrap(), epsilon = 1.0e-7);

    let svd = m.transpose().svd(true, true);
    assert_relative_eq!(m.transpose(), svd.recompose().unwrap(), epsilon = 1.0e-7);
}

#[test]
#[rustfmt::skip]
fn svd_fail() {
    let m = Matrix6::new(
        0.9299319121545955,   0.9955870335651049,   0.8824725266413644,  0.28966880207132295,  0.06102723649846409,   0.9311880746048009,
        0.5938395242304351,   0.8398522876024204,  0.06672831951963198,   0.9941213119963099,   0.9431846038057834,   0.8159885168706427,
        0.9121962883152357,   0.6471119669367571,   0.4823309702814407,   0.6420516076705516,   0.7731203925207113,   0.7424069470756647,
        0.07311092531259344,   0.5579247949052946,  0.14518764691585773,  0.03502980663114896,   0.7991329455957719,   0.4929930019965745,
        0.12293810556077789,   0.6617084679545999,   0.9002240700227326, 0.027153062135304884,   0.3630189466989524,  0.18207502727558866,
        0.843196731466686,  0.08951878746549924,   0.7533450877576973, 0.009558876499740077,   0.9429679490873482,   0.9355764454129878);
    
    // Check unordered ...
    let svd = m.clone().svd_unordered(true, true);
    let recomp = svd.recompose().unwrap();
    assert_relative_eq!(m, recomp, epsilon = 1.0e-5);

    // ... and ordered SVD.
    let svd = m.clone().svd(true, true);
    let recomp = svd.recompose().unwrap();
    assert_relative_eq!(m, recomp, epsilon = 1.0e-5);
}

#[test]
fn svd_err() {
    let m = DMatrix::from_element(10, 10, 0.0);
    let svd = m.clone().svd(false, false);
    assert_eq!(
        Err("SVD recomposition: U and V^t have not been computed."),
        svd.clone().recompose()
    );
    assert_eq!(
        Err("SVD pseudo inverse: the epsilon must be non-negative."),
        svd.clone().pseudo_inverse(-1.0)
    );
}

#[test]
#[rustfmt::skip]
fn svd_sorted() {
    let reference = nalgebra::matrix![
        1.0, 2.0, 3.0, 4.0;
        5.0, 6.0, 7.0, 8.0;
        9.0, 10.0, 11.0, 12.0
    ];

    let mut svd = nalgebra::SVD {
        singular_values: nalgebra::matrix![1.72261225; 2.54368356e+01; 5.14037515e-16],
        u: Some(nalgebra::matrix![
            -0.88915331, -0.20673589, 0.40824829;
            -0.25438183, -0.51828874, -0.81649658;
            0.38038964, -0.82984158, 0.40824829
        ]),
        v_t: Some(nalgebra::matrix![
            0.73286619,  0.28984978, -0.15316664, -0.59618305;
            -0.40361757, -0.46474413, -0.52587069, -0.58699725;
            0.44527162, -0.83143156,  0.32704826,  0.05911168
        ]),
    };

    assert_relative_eq!(
        svd.recompose().expect("valid SVD"),
        reference,
        epsilon = 1.0e-5
    );

    svd.sort_by_singular_values();

    // Ensure successful sorting
    assert_relative_eq!(svd.singular_values.x, 2.54368356e+01, epsilon = 1.0e-5);

    // Ensure that the sorted components represent the same decomposition
    assert_relative_eq!(
        svd.recompose().expect("valid SVD"),
        reference,
        epsilon = 1.0e-5
    );
}
