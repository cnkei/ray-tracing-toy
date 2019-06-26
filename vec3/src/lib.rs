use std::ops::Neg;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3([f32; 3]);

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3([x, y, z])
    }

    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self[2]
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self[0]
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self[1]
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self[2]
    }

    #[inline]
    pub fn squared_length(&self) -> f32 {
        self.0.iter().map(|v| (*v).powi(2)).sum()
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        let k = 1.0 / self.length();
        self * k
    }
}

macro_rules! impl_bin_op {
    (impl $imp:ident, $method:ident) => {
        impl $imp<Vec3> for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(mut self, rhs: Vec3) -> Self::Output {
                for i in 0..3 {
                    self[i] = self[i].$method(rhs[i]);
                }
                self
            }
        }
    };
}

macro_rules! forward_val_ref_bin_op {
    (impl $imp:ident, $method:ident) => {
        impl $imp<&Vec3> for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: &Vec3) -> Self::Output {
                self.$method(*rhs)
            }
        }
    };
}

macro_rules! forward_ref_val_bin_op {
    (impl $imp:ident, $method:ident) => {
        impl $imp<Vec3> for &Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: Vec3) -> Self::Output {
                (*self).$method(rhs)
            }
        }
    };
}

macro_rules! forward_ref_ref_bin_op {
    (impl $imp:ident, $method:ident) => {
        impl $imp<&Vec3> for &Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: &Vec3) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
    };
}

macro_rules! forward_all_bin_op {
    (impl $imp:ident, $method:ident) => {
        forward_val_ref_bin_op!(impl $imp, $method);
        forward_ref_val_bin_op!(impl $imp, $method);
        forward_ref_ref_bin_op!(impl $imp, $method);
    };
}

impl_bin_op!(impl Add, add);
forward_all_bin_op!(impl Add, add);

impl_bin_op!(impl Sub, sub);
forward_all_bin_op!(impl Sub, sub);

impl_bin_op!(impl Mul, mul);
forward_all_bin_op!(impl Mul, mul);

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(mut self, rhs: f32) -> Self::Output {
        for i in 0..3 {
            self[i] *= rhs;
        }
        self
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        (*self) * rhs
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl_bin_op!(impl Div, div);
forward_all_bin_op!(impl Div, div);

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        (*self) / rhs
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut v = self;
        for i in 0..3 {
            v[i] = -v[i];
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_indexing() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, v[0]);
        assert_eq!(2.0, v[1]);
        v[1] = 1.0;
        assert_eq!(1.0, v[1]);
    }

    #[test]
    #[should_panic]
    fn test_indexing_out_of_bound() {
        let v = Vec3::zero();
        let _ = v[3];
    }

    #[test]
    fn test_negate() {
        assert_eq!(Vec3::new(-1.0, -2.0, -3.0), -Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_arithmatic() {
        assert_eq!(
            Vec3::new(2.0, -4.0, 6.0),
            Vec3::new(1.0, -2.0, 3.0) + Vec3::new(1.0, -2.0, 3.0)
        );
        assert_eq!(
            Vec3::new(2.0, -4.0, 6.0),
            Vec3::new(1.0, -2.0, 3.0) + &Vec3::new(1.0, -2.0, 3.0)
        );
        assert_eq!(
            Vec3::new(2.0, -4.0, 6.0),
            &Vec3::new(1.0, -2.0, 3.0) + Vec3::new(1.0, -2.0, 3.0)
        );
        assert_eq!(
            Vec3::new(2.0, -4.0, 6.0),
            &Vec3::new(1.0, -2.0, 3.0) + &Vec3::new(1.0, -2.0, 3.0)
        );

        assert_eq!(
            Vec3::new(2.0, -4.0, 6.0),
            Vec3::new(3.0, -6.0, 9.0) - Vec3::new(1.0, -2.0, 3.0)
        );
        assert_eq!(
            Vec3::new(2.0, -4.0, 6.0),
            Vec3::new(2.0, 2.0, 2.0) * Vec3::new(1.0, -2.0, 3.0)
        );
        assert_eq!(
            Vec3::new(2.0, -4.0, 6.0),
            Vec3::new(2.0, 8.0, 18.0) / Vec3::new(1.0, -2.0, 3.0)
        );

        assert_eq!(Vec3::new(6.0, -12.0, 18.0), Vec3::new(3.0, -6.0, 9.0) * 2.0);
        assert_eq!(
            Vec3::new(6.0, -12.0, 18.0),
            &Vec3::new(3.0, -6.0, 9.0) * 2.0
        );
        assert_eq!(Vec3::new(6.0, -12.0, 18.0), 2.0 * Vec3::new(3.0, -6.0, 9.0));
        assert_eq!(
            Vec3::new(6.0, -12.0, 18.0),
            2.0 * &Vec3::new(3.0, -6.0, 9.0)
        );
        assert_eq!(Vec3::new(1.0, -2.0, 3.0), Vec3::new(3.0, -6.0, 9.0) / 3.0);
        assert_eq!(Vec3::new(1.0, -2.0, 3.0), &Vec3::new(3.0, -6.0, 9.0) / 3.0);

        assert_eq!(50.0, Vec3::new(3.0, 4.0, -5.0).squared_length());
        assert_eq!(5.0, Vec3::new(3.0, -4.0, 0.0).length());
        assert_eq!(
            Vec3::new(0.6, -0.8, 0.0),
            Vec3::new(3.0, -4.0, 0.0).unit_vector()
        );
    }
}
