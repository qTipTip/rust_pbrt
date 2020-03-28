use std::ops;

pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// the where clause can alternatively be written as impl<T: Copy + Add<T, Output=T>
// I.e, this is implemented only for generic types implementing Copy and Add:T->T.
impl<T> ops::Add for Vector2<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vector2<T>;
    fn add(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
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

        assert_eq!(v1 + v2, v3);
    }
}