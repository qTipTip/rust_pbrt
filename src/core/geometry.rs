use std::fmt::Debug;
use std::ops;

use num_traits::Float;

pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<f32>;

// We derive from PartialEq in order to use assert_eqs in tests.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

// This NaN-check is only implemented for Float-types.
impl<T> Vector2<T> where T: Float {
    pub fn new(x: T, y: T) -> Self {
        assert!(
            !x.is_nan() && !y.is_nan()
        );
        Vector2 { x: x, y: y }
    }
}

impl<T> Vector2<T> {
    pub fn length_squared(self) -> T where T: ops::Mul<T, Output=T> + ops::Add<T, Output=T> + Copy {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> T where T: num_traits::Float {
        self.length_squared().sqrt()
    }

    pub fn has_nans(self) -> bool where T: num_traits::Float {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn abs(self) -> Vector2<T> where T: num_traits::sign::Signed {
        Vector2 { x: self.x.abs(), y: self.y.abs() }
    }

    pub fn dot(self, rhs: Vector2<T>) -> T where T: ops::Mul<Output=T> + ops::Add<Output=T> {
        self.x * rhs.x + self.y * rhs.y
    }
}


// the where clause can alternatively be written as impl<T: Copy + Add<T, Output=T>
// I.e, this is implemented only for generic types implementing Copy and Add:T->T.
impl<T> ops::Add for Vector2<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vector2<T>;
    fn add(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

// Note that we require self to be mutable in order to alter its internal state.
impl<T> ops::AddAssign for Vector2<T> where T: ops::AddAssign {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> ops::Sub for Vector2<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vector2<T>;
    fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

// Note that we require self to be mutable in order to alter its internal state.
impl<T> ops::SubAssign for Vector2<T> where T: ops::SubAssign {
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> ops::Mul<T> for Vector2<T> where T: Float {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T> ops::MulAssign<T> for Vector2<T> where T: Copy + ops::MulAssign {
    fn mul_assign(&mut self, constant: T) {
        self.x *= constant;
        self.y *= constant;
    }
}

impl<T> ops::Div<T> for Vector2<T> where T: Float {
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Vector2<T> {
        self * rhs.recip()
    }
}

impl<T> ops::Neg for Vector2<T> where T: Copy + ops::Neg<Output=T> {
    type Output = Vector2<T>;

    fn neg(self) -> Self::Output {
        Vector2 { x: -self.x, y: -self.y }
    }
}

impl<T> ops::Index<u32> for Vector2<T> {
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds for Vector2"),
        }
    }
}

pub fn vec2_dot<T>(a: Vector2<T>, b: Vector2<T>) -> T where T: ops::Mul<Output=T> + ops::Add<Output=T> {
    a.x * b.x + a.y * b.y
}

#[cfg(test)]
mod vector2_tests {
    use std::panic;

    use num_traits::Float;

    use crate::core::geometry::{vec2_dot, Vector2};

    #[test]
    fn test_constructor_nan_float() {
        let result_should_pass = panic::catch_unwind(||
            Vector2::new(3.2f32, 3.1f32)
        );
        let result_should_panic = panic::catch_unwind(||
            Vector2::new(Float::nan(), 3.1f32)
        );


        assert!(result_should_pass.is_ok());
        assert!(result_should_panic.is_err());
    }

    #[test]
    fn test_add() {
        let v1 = Vector2 { x: 2.0f32, y: 3.0f32 };
        let v2 = Vector2 { x: 1.0f32, y: -1.0f32 };
        let v3 = Vector2 { x: 3.0f32, y: 2.0f32 };
        let v4 = Vector2 { x: 0.0f32, y: 0.0f32 };

        assert_eq!(v1 + v2, v3);
        assert_ne!(v1 + v2, v4);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vector2 { x: 2.0f32, y: 3.0f32 };
        let v2 = Vector2 { x: 1.0f32, y: -1.0f32 };
        let v3 = Vector2 { x: 3.0f32, y: 2.0f32 };
        let v4 = Vector2 { x: 0.0f32, y: 0.0f32 };

        v1 += v2;

        assert_eq!(v1, v3);
        assert_ne!(v1, v4);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector2 { x: 2.0f32, y: 3.0f32 };
        let v2 = Vector2 { x: 1.0f32, y: -1.0f32 };
        let v3 = Vector2 { x: 1.0f32, y: 4.0f32 };
        let v4 = Vector2 { x: 0.0f32, y: 0.0f32 };

        assert_eq!(v1 - v2, v3);
        assert_ne!(v1 - v2, v4);
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = Vector2 { x: 2.0f32, y: 3.0f32 };
        let v2 = Vector2 { x: 1.0f32, y: -1.0f32 };
        let v3 = Vector2 { x: 1.0f32, y: 4.0f32 };
        let v4 = Vector2 { x: 0.0f32, y: 0.0f32 };

        v1 -= v2;

        assert_eq!(v1, v3);
        assert_ne!(v1, v4);
    }


    #[test]
    fn test_mul() {
        let v1 = Vector2 { x: 1.0f32, y: 3.0f32 };
        let constant: f32 = 2.0f32;
        let v2 = Vector2 { x: 2.0f32, y: 6.0f32 };
        let v3 = Vector2 { x: 1.9f32, y: 6.0f32 };

        assert_eq!(v1 * constant, v2);
        assert_ne!(v1 * constant, v3);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vector2 { x: 1.0f32, y: 3.0f32 };
        let constant: f32 = 2.0f32;
        let v2 = Vector2 { x: 2.0f32, y: 6.0f32 };
        let v3 = Vector2 { x: 1.9f32, y: 6.0f32 };

        v1 *= constant;
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_div() {
        let v1 = Vector2 { x: 1.0f32, y: 1.0f32 };
        let v2 = Vector2 { x: 0.5f32, y: 0.5f32 };

        assert_eq!(v1 / 2.0, v2);
        assert_ne!(v1 / 2.0, v1);
    }

    #[test]
    fn test_neg() {
        let v1 = Vector2 { x: 1.0f32, y: 1.0f32 };

        assert_eq!(-v1, v1 * (-1.0));
        assert_eq!(-(-v1), v1);
        assert_ne!(-v1, v1);
    }

    #[test]
    fn test_length() {
        let v1 = Vector2 { x: 0.0f32, y: 3.0f32 };

        assert_eq!(v1.length_squared(), 9.0);
        assert_eq!(v1.length(), 3.0);
        assert_ne!(v1.length(), 2.0);
    }

    #[test]
    fn test_is_nan() {
        let v1 = Vector2 { x: 0.0f32, y: Float::nan() };
        let v2 = Vector2 { x: 0.0f32, y: 0.0f32 };

        assert!(v1.has_nans());
        assert!(!v2.has_nans());
    }

    #[test]
    fn test_index() {
        let v1 = Vector2 { x: 1, y: 2 };

        assert_eq!(v1[0], v1.x);
        assert_eq!(v1[1], v1.y);

        let result_should_panic = panic::catch_unwind(||
            v1[3]);
        assert!(result_should_panic.is_err());
    }

    #[test]
    fn test_abs() {
        let v1 = Vector2 { x: -1, y: 2 };
        let v2 = Vector2 { x: 1, y: 2 };

        assert_eq!(v1.abs(), v2);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector2 { x: -1.0, y: 2.0 };
        assert_eq!(vec2_dot(v1, v1), 5.0);
        assert_eq!(v1.dot(v1), 5.0);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// This NaN-check is only implemented for Float-types.
impl<T> Vector3<T> where T: Float {
    pub fn new(x: T, y: T, z: T) -> Self {
        assert!(
            !x.is_nan() && !y.is_nan() && !z.is_nan()
        );
        Vector3 { x: x, y: y, z: z }
    }
}

impl<T> Vector3<T> {
    pub fn length_squared(self) -> T where T: ops::Mul<T, Output=T> + ops::Add<T, Output=T> + Copy {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> T where T: num_traits::Float {
        self.length_squared().sqrt()
    }

    pub fn has_nans(self) -> bool where T: num_traits::Float {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn abs(self) -> Vector3<T> where T: num_traits::sign::Signed {
        Vector3 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    pub fn dot(self, rhs: Vector3<T>) -> T where T: ops::Mul<Output=T> + ops::Add<Output=T> {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}


// the where clause can alternatively be written as impl<T: Copy + Add<T, Output=T>
// I.e, this is implemented only for generic types implementing Copy and Add:T->T.
impl<T> ops::Add for Vector3<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vector3<T>;
    fn add(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

// Note that we require self to be mutable in order to alter its internal state.
impl<T> ops::AddAssign for Vector3<T> where T: ops::AddAssign {
    fn add_assign(&mut self, rhs: Vector3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> ops::Sub for Vector3<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

// Note that we require self to be mutable in order to alter its internal state.
impl<T> ops::SubAssign for Vector3<T> where T: ops::SubAssign {
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> ops::Mul<T> for Vector3<T> where T: Float {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl<T> ops::MulAssign<T> for Vector3<T> where T: Copy + ops::MulAssign {
    fn mul_assign(&mut self, constant: T) {
        self.x *= constant;
        self.y *= constant;
        self.z *= constant;
    }
}

impl<T> ops::Div<T> for Vector3<T> where T: Float {
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Vector3<T> {
        self * rhs.recip()
    }
}

impl<T> ops::Neg for Vector3<T> where T: Copy + ops::Neg<Output=T> {
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl<T> ops::Index<u32> for Vector3<T> {
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector2"),
        }
    }
}

pub fn vec3_dot<T>(a: Vector3<T>, b: Vector3<T>) -> T where T: ops::Mul<Output=T> + ops::Add<Output=T> {
    a.x * b.x + a.y * b.y + a.z + b.z
}

#[cfg(test)]
mod vector3_tests {
    use std::panic;

    use num_traits::Float;

    use crate::core::geometry::{vec2_dot, vec3_dot, Vector2, Vector3};

    #[test]
    fn test_constructor_nan_float() {
        let result_should_pass = panic::catch_unwind(||
            Vector3::new(3.2f32, 3.1f32, 1.0f32)
        );
        let result_should_panic = panic::catch_unwind(||
            Vector3::new(Float::nan(), 3.1f32, 1.0f32)
        );


        assert!(result_should_pass.is_ok());
        assert!(result_should_panic.is_err());
    }

    #[test]
    fn test_add() {
        let v1 = Vector3 { x: 2.0f32, y: 3.0f32, z: 1.0f32 };
        let v2 = Vector3 { x: 1.0f32, y: -1.0f32, z: 1.0f32 };
        let v3 = Vector3 { x: 3.0f32, y: 2.0f32, z: 2.0f32 };
        let v4 = Vector3 { x: 0.0f32, y: 0.0f32, z: 1.0f32 };

        assert_eq!(v1 + v2, v3);
        assert_ne!(v1 + v2, v4);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vector3 { x: 2.0f32, y: 3.0f32, z: 1.0f32 };
        let v2 = Vector3 { x: 1.0f32, y: -1.0f32, z: 1.0f32 };
        let v3 = Vector3 { x: 3.0f32, y: 2.0f32, z: 2.0f32 };
        let v4 = Vector3 { x: 0.0f32, y: 0.0f32, z: 1.0f32 };

        v1 += v2;

        assert_eq!(v1, v3);
        assert_ne!(v1, v4);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector3 { x: 2.0f32, y: 3.0f32, z: 1.0f32 };
        let v2 = Vector3 { x: 1.0f32, y: -1.0f32, z: 1.0f32 };
        let v3 = Vector3 { x: 1.0f32, y: 4.0f32, z: 0.0f32 };
        let v4 = Vector3 { x: 0.0f32, y: 0.0f32, z: 1.0f32 };

        assert_eq!(v1 - v2, v3);
        assert_ne!(v1 - v2, v4);
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = Vector3 { x: 2.0f32, y: 3.0f32, z: 1.0f32 };
        let v2 = Vector3 { x: 1.0f32, y: -1.0f32, z: 1.0f32 };
        let v3 = Vector3 { x: 1.0f32, y: 4.0f32, z: 0.0f32 };
        let v4 = Vector3 { x: 0.0f32, y: 0.0f32, z: 1.0f32 };

        v1 -= v2;

        assert_eq!(v1, v3);
        assert_ne!(v1, v4);
    }


    #[test]
    fn test_mul() {
        let v1 = Vector3 { x: 1.0f32, y: 3.0f32, z: 1.0f32 };
        let constant: f32 = 2.0f32;
        let v2 = Vector3 { x: 2.0f32, y: 6.0f32, z: 2.0f32 };
        let v3 = Vector3 { x: 1.9f32, y: 6.0f32, z: 1.0f32 };

        assert_eq!(v1 * constant, v2);
        assert_ne!(v1 * constant, v3);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vector3 { x: 1.0f32, y: 3.0f32, z: 1.0f32 };
        let constant: f32 = 2.0f32;
        let v2 = Vector3 { x: 2.0f32, y: 6.0f32, z: 2.0f32 };
        let v3 = Vector3 { x: 1.9f32, y: 6.0f32, z: 1.0f32 };

        v1 *= constant;
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_div() {
        let v1 = Vector3 { x: 1.0f32, y: 1.0f32, z: 1.0f32 };
        let v2 = Vector3 { x: 0.5f32, y: 0.5f32, z: 0.5f32 };

        assert_eq!(v1 / 2.0, v2);
        assert_ne!(v1 / 2.0, v1);
    }

    #[test]
    fn test_neg() {
        let v1 = Vector3 { x: 1.0f32, y: 1.0f32, z: 1.0f32 };
        assert_eq!(-v1, v1 * (-1.0));
        assert_eq!(-(-v1), v1);
        assert_ne!(-v1, v1);
    }

    #[test]
    fn test_length() {
        let v1 = Vector3 { x: 0.0f32, y: 3.0f32, z: 0.0f32 };

        assert_eq!(v1.length_squared(), 9.0);
        assert_eq!(v1.length(), 3.0);
        assert_ne!(v1.length(), 2.0);
    }

    #[test]
    fn test_is_nan() {
        let v1 = Vector3 { x: 0.0f32, y: Float::nan(), z: 1.0f32 };
        let v2 = Vector3 { x: 0.0f32, y: 0.0f32, z: 1.0f32 };

        assert!(v1.has_nans());
        assert!(!v2.has_nans());
    }

    #[test]
    fn test_index() {
        let v1 = Vector3 { x: 1, y: 2, z: 3 };

        assert_eq!(v1[0], v1.x);
        assert_eq!(v1[1], v1.y);
        assert_eq!(v1[2], v1.z);

        let result_should_panic = panic::catch_unwind(||
            v1[4]);
        assert!(result_should_panic.is_err());
    }

    #[test]
    fn test_abs() {
        let v1 = Vector3 { x: -1, y: 2, z: 1 };
        let v2 = Vector3 { x: 1, y: 2, z: 1 };

        assert_eq!(v1.abs(), v2);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector3 { x: -1.0, y: 2.0, z: 0.0 };
        assert_eq!(vec3_dot(v1, v1), 5.0);
        assert_eq!(v1.dot(v1), 5.0);
    }
}