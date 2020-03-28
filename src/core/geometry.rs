use std::ops;

pub struct Vector2<T> {
    x: T,
    y: T,
}

pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

// the where clause can alternatively be written as impl<T: Copy + Add<T, Output=T>
// I.e, this is implemented only for generic types implementing Copy and Add:T->T.
impl<T> ops::Add for Vector2<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vector2<T>;
    fn add(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
