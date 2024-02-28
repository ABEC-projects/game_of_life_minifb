use std::ops::{Add, AddAssign, Mul, Div, Sub, SubAssign};
#[derive(Clone, Copy)]
pub struct Vec2D{
    pub x: f32,
    pub y: f32,
}
impl Vec2D{
    pub const ZERO: Self = Self{x: 0., y: 0.};
    pub fn new (cords: (f32, f32)) -> Vec2D{
        Vec2D{ x: cords.0, y: cords.1 }
    }
    pub fn get_tuple(&self) -> (f32, f32){
        (self.x, self.y)
    }
    pub fn div(&self, rhs: f32) -> Vec2D{
        Self::new((self.x/rhs, self.y/rhs))
    }
    pub fn floor(&self) -> Self{
        Self::new((self.x.floor(), self.y.floor()))
    }
}

impl Add for Vec2D{
    type Output = Vec2D;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2D::new((self.x+rhs.x, self.y+rhs.y))
    }
}
impl Sub for Vec2D{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new((self.x-rhs.x, self.y-rhs.y))
    }
}
impl SubAssign for Vec2D{
    fn sub_assign(&mut self, rhs: Self){
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl Mul for Vec2D{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output{
        Self::new((self.x*rhs.x, self.y*rhs.y))
    }
}

