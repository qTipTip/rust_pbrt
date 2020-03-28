use std::ops;

// We derive from PartialEq in order to use assert_eqs in tests.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
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

impl<T> ops::MulAssign<T> for Vector2<T> where T: Copy + ops::MulAssign {
    fn mul_assign(&mut self, constant: T) {
        self.x *= constant;
        self.y *= constant;
    }
}

#[cfg(test)]
mod vector2_tests {
    use crate::core::geometry::Vector2;

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
    fn test_mul_assign() {
        let mut v1 = Vector2 { x: 1.0f32, y: 3.0f32 };
        let constant: f32 = 2.0f32;
        let v2 = Vector2 { x: 2.0f32, y: 6.0f32 };
        let v3 = Vector2 { x: 1.9f32, y: 6.0f32 };

        v1 *= constant;
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}