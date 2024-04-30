use std::hash::Hash;

pub trait Point: Eq + Hash + Copy + Clone {
    fn generate(x: i32, y: i32, z: i32) -> Self;
    fn coordinates(&self) -> (i32, i32, i32);

    fn x(&self) -> i32 {
        self.coordinates().0
    }

    fn y(&self) -> i32 {
        self.coordinates().1
    }

    fn z(&self) -> i32 {
        self.coordinates().2
    }
}
