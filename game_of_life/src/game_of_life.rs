use std::{
    isize, ops::{Index, IndexMut}};
#[derive (Clone)]
pub struct Field{
    pub vec: Vec<bool>,
    width: usize,
    height: usize
}
impl Field{
    pub fn new(vec: Vec<bool>, width: usize, height: usize) -> Self{
        Field {vec, width, height}
    }

    pub fn get_vec(&self) -> &Vec<bool> {
        &self.vec
    }
    pub fn get_vec_mut(&mut self) -> &mut Vec<bool> {
        &mut self.vec
    }
    pub fn get_width(&self) -> &usize {
        &self.width
    }
    pub fn get_width_mut(&mut self) -> &mut usize {
        &mut self.width
    }
    pub fn get_height(&self) -> &usize {
        &self.height
    }
    pub fn get_height_mut(&mut self) -> &mut usize {
        &mut self.height
    }

    //pub fn get_bilin(&self, cords: (f32, f32)) -> f32{
    //    let (x, y) = cords;
    //    let to_f32 = |a: bool| -> f32{
    //        if a {1.0} else {0.0}
    //    };
    //    let x1 = x.floor() as usize;
    //    let x2: Option<usize> = if x+1. < self.width as f32 {Option::Some((x.floor()+1.) as usize)} else {Option::None};
    //    let y1 = y.floor() as usize;
    //    let y2: Option<usize> = if y+1. < self.height as f32 {Option::Some((y.floor()+1.) as usize)} else {Option::None};
    //    match x2{
    //        Some(x2) => {
    //            let v1 = to_f32(self[(x1, y1)]) * (1.-x.fract()) +  to_f32(self[(x2, y1)]) as f32 * (x.fract());
    //            match y2 {
    //                Some(y2) => {
    //                    let v2 = to_f32(self[(x1, y2)]) * (1.-x.fract()) +  to_f32(self[(x2, y2)]) * (x.fract());
    //                    v1 * (1.-y.fract()) + v2 * (y.fract())
    //                },
    //                None => {
    //                    v1
    //                },
    //            }
    //        },
    //        None => {
    //            let v1 = to_f32(self[(x1, y1)]);
    //            
    //            match y2 {
    //                Some(y2) => {
    //                    let v2 = to_f32(self[(x1, y2)]);
    //                    v1 * (1.-y.fract()) + v2 * (y.fract())
    //                },
    //                None => {
    //                    v1
    //                },
    //            }
    //        },
    //    }
    //    
    //}
}
impl Index<(usize, usize)> for Field{
    type Output = bool;

    fn index(&self, cords: (usize, usize)) -> &Self::Output{
        let index: usize = cords.1 * self.width + cords.0;  
        &self.vec[index]
    }
}
impl IndexMut<(usize, usize)> for Field{
    fn index_mut(&mut self, cords: (usize, usize)) -> &mut Self::Output{
        let index: usize = cords.1 * self.width + cords.0;  
        &mut self.vec[index]
    }
}

pub struct GameInstance {
    field: Field,
    options: Options,
}
impl GameInstance {
    pub fn new(field: Vec<bool>, dimentions: (usize, usize), options: Options) -> Self {
        GameInstance {
            field: Field::new(field, dimentions.0, dimentions.1),
            options,
        }
    }

    pub fn new_generation(&mut self) {
        let mut new_field = self.field.clone();
        let len_x = self.width();
        for x in 0..len_x {
            let len_y = self.height();
            for y in 0..len_y {
                let mut counter: i8 = 0;
                for offset in [
                    (-1, -1),
                    (1, 0),
                    (0, 1),
                    (1, -1),
                    (-1, 1),
                    (0, -1),
                    (-1, 0),
                    (1, 1),
                ] {
                    let (x_t, y_t) = (x as isize + offset.0, y as isize + offset.1);
                    if x_t >= 0
                        && y_t >= 0
                        && x_t < len_x as isize
                        && y_t < len_y as isize
                        && self.field[(x_t as usize, y_t as usize)]
                    {
                        counter += 1;
                    }
                }
                if self.field[(x, y)] {
                    if counter != 2 && counter != 3 {
                        new_field[(x, y)] = false;
                    }
                } else if counter == 3 {
                    new_field[(x, y)] = true;
                }
            }
        }
        self.field.vec = new_field.vec;
    }


    pub fn get_field(&self) -> &Field {
        &self.field
    }

    pub fn get_field_mut(&mut self) -> &mut Field{
        &mut self.field
    }

    pub fn width(&self) -> usize {
        self.field.width
    }

    pub fn height(&self) -> usize {
        self.field.height
    }


}

pub struct Options {
    pub boundaries_rule: BoundariesRule,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            boundaries_rule: crate::game_of_life::BoundariesRule::Finite,
        }
    }
}
pub enum BoundariesRule {
    _Infinite,
    Finite,
    _Cyclic,
}
