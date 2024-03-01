mod vector_geom;


use minifb::{Key, Window, WindowOptions};
use vector_geom::Vec2D;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const WINDOW_SCALE: f32 = 2.;
const ZOOM_SPEED: f32 = 0.2;

fn from_local(cords: Vec2D, offset: Vec2D, scale: f32 )-> Vec2D{
    Vec2D::new(((cords.x + offset.x)/(WINDOW_SCALE*scale), (cords.y + offset.y)/(WINDOW_SCALE*scale)))
}

fn main() {
    let mut game = game_of_life::game_of_life::GameInstance::new(
        game_of_life::field_presets::r_pentomino(),
        (WIDTH, HEIGHT),
        game_of_life::game_of_life::Options::default(),
    );
    

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH * WINDOW_SCALE as usize,
        HEIGHT * WINDOW_SCALE as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });


    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut scale_factor = 1.;
    let mut prev_mouse_pos: Vec2D = Vec2D::new((0., 0.));
    let mut offset: Vec2D = Vec2D::new((0., 0.));
    let mut screen = ScreenImage::new(vec![0; WIDTH*HEIGHT], *game.get_field().get_width(), *game.get_field().get_height());
    let mut pause = true;


    while window.is_open() && !window.is_key_down(Key::Escape) {
        //Pausing
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No){
            pause = !pause;
        }
        
        let cur_mouse_pos: Vec2D =  Vec2D::new(window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap());
        let wheel_movement = window.get_scroll_wheel().unwrap_or_else(|| -> (f32, f32) {(0., 0.)});
        let old_size = Vec2D::new((WIDTH as f32*WINDOW_SCALE / scale_factor, HEIGHT as f32*WINDOW_SCALE / scale_factor));
        scale_factor += wheel_movement.1*ZOOM_SPEED;
        scale_factor = scale_factor.clamp(1., 20.);
        let new_size = Vec2D::new((WIDTH as f32*WINDOW_SCALE / scale_factor, HEIGHT as f32*WINDOW_SCALE / scale_factor));

        offset.x -= (new_size.x-old_size.x)*cur_mouse_pos.x/(WIDTH as f32*WINDOW_SCALE)/2.;
        offset.y -= (new_size.y-old_size.y)*cur_mouse_pos.y/(HEIGHT as f32*WINDOW_SCALE)/2.;
        
        // Camera movement
        if window.get_mouse_down(minifb::MouseButton::Middle){
            offset -= (cur_mouse_pos - prev_mouse_pos).div(scale_factor*WINDOW_SCALE);
        }
        // Pixel Drawing
        if window.get_mouse_down(minifb::MouseButton::Left){
            let from: Vec2D = from_local(prev_mouse_pos, Vec2D::ZERO, scale_factor);
            let mut to: Vec2D = from_local(cur_mouse_pos, Vec2D::ZERO, scale_factor);
            let x_inverted = if to.x < from.x {
                to.x = 2.*from.x-to.x;
                true
            }else{
                false
            };
            let y_inverted = if to.y < from.y {
                to.y = 2.*from.y-to.y;
                true
            }else{
                false
            };
            let invert_x_y= (to.x-from.x).abs() < (to.y - from.y).abs();

            if !invert_x_y{
            let  speed = (to.y-from.y)/(to.x-from.x);
                let mut x = from.x.clamp(0., WIDTH as f32 -1.);
                while x <= to.x {
                    let mut  y_ = speed*(x-from.x)+from.y;
                    let mut x_ = x;
                    if speed.is_nan() {
                        game.get_field_mut()[((to.x + offset.x) as usize, (to.y + offset.y) as usize)] = true;
                        break;
                    }
                    if x_inverted {x_ = 2.*from.x-x_;}
                    if y_inverted {y_ = 2.*from.y-y_;}
                    y_ = y_.clamp(0., HEIGHT as f32 - 1. - offset.y);
                    x_ = x_.clamp(0., WIDTH as f32 - 1. - offset.x);
                    game.get_field_mut()[((x_+offset.x)as usize, (y_+offset.y)as usize)] = true;
                    x += 1.;
                };
            }else{
                let speed = (to.x-from.x)/(to.y-from.y);
                let mut y = from.y.clamp(0., HEIGHT as f32 - 1.);
                while y <= to.y {
                    let mut x_ = speed*(y-from.y)+from.x;
                    let mut y_ = y;
                    if speed.is_nan() {
                        game.get_field_mut()[((to.x + offset.x) as usize, (to.y + offset.y) as usize)] = true;
                        break;
                    }
                    if x_inverted {x_ = 2.*from.x-x_;}
                    if y_inverted {y_ = 2.*from.y-y_;}
                    x_ = x_.clamp(0., WIDTH as f32 - 1. - offset.x);
                    y_ = y_.clamp(0., HEIGHT as f32 - 1. - offset.y);
                    game.get_field_mut()[((x_+offset.x)as usize, (y_+offset.y)as usize)] = true;
                    y += 1.;
                };
            }
        }

        offset.x = offset.x.clamp(0., WIDTH as f32 - WIDTH as f32 / scale_factor);
        offset.y = offset.y.clamp(0., HEIGHT as f32 - HEIGHT as f32 / scale_factor);

        screen.set_vector(game.get_field().get_vec().iter().map(|x: &bool| if *x {u32::MAX} else {0}).collect());
        scale::zoom_nearest_self(&mut screen, scale_factor.into(), offset.floor().get_tuple());
        if !pause{
            game.new_generation();
        }
        prev_mouse_pos = Vec2D::new(window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap());
        window.update_with_buffer(&screen.get_vector().to_vec(), WIDTH, HEIGHT).unwrap();

    }
    

    struct ScreenImage{
        vector: Vec<u32>,
        width: usize,
        height: usize
    }
    impl ScreenImage {
        fn new (vector: Vec<u32>, width: usize, height: usize) -> ScreenImage{
            ScreenImage{
                vector,
                width,
                height
            }
        }
        fn get_vector(&self) -> &Vec<u32>{
            &self.vector
        }
        fn set_vector(&mut self, new: Vec<u32>){
            self.vector = new;
        }

    }
    impl std::ops::Index<(usize, usize)> for ScreenImage{
        type Output = u32;
        fn index(&self, index: (usize, usize)) -> &Self::Output {
            &self.vector[index.0 + index.1*self.width]
        }
    }
    impl std::ops::IndexMut<(usize, usize)> for ScreenImage{
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            &mut self.vector[index.0 + index.1*self.width]
        }
    }

    impl scale::ScalableImage for ScreenImage{
        type Stored = u32;
        fn index(&self, x: usize, y: usize) -> &u32 {
            &self[(x, y)]
        }

        fn index_mut(&mut self, x: usize, y: usize) -> &mut u32 {
            &mut self[(x, y)]
        }

        fn width(&self) -> usize {
            self.width
        }

        fn height(&self) -> usize {
            self.height
        }

        fn to_rgba(color: &u32) -> u128 {
            let color = *color as u128;
            let (r, g, b, a) = ( (color & 255) as u128, (color >> 8 & 255) as u128, (color >> 16 & 255) as u128, (color >> 24 & 255) as u128);
            (a << 96) | (r << 64) | (g << 32) | b
        }
        fn from_rgba(color: u128) -> u32 {
            let (r, g, b, a) = ( (color & 255) as u32, (color >> 32 & 255) as u32, (color >> 64 & 255) as u32, (color >> 96 & 255) as u32);
            (a << 24) | (r << 16) | (g << 8) | b
        }

        fn copy(input: &Self::Stored) -> Self::Stored {
            *input
        }
        fn get_vector(&self) -> &Vec<u32>{
            &self.vector
        }
        fn new_clear(&self) -> Self {
            Self::new(vec![0; self.width()*self.height()], 
                self.width, 
                self.height)
        }


    }

}


mod scale{
    use game_of_life::game_of_life::Field;
    use std::cmp::min;

    
    pub trait ScalableImage{
        type Stored;
        fn index(&self, x: usize, y: usize) -> &Self::Stored;
        fn index_mut(&mut self, x: usize, y: usize) -> &mut Self::Stored;
        fn width(&self) -> usize;
        fn height(&self) -> usize;
        fn to_rgba(color: &Self::Stored) -> u128;
        fn from_rgba(color: u128) -> Self::Stored;
        fn copy(input: &Self::Stored) -> Self::Stored;
        fn get_vector(&self) -> &Vec<Self::Stored>;
        fn new_clear(&self) -> Self;

        fn get_bilin(&self, x: f32, y: f32) -> Self::Stored{
            let lin_inter = |v1: &Self::Stored, v2: &Self::Stored, factor: f32| -> Self::Stored{
                let v1 = Self::to_rgba(v1);
                let v2 = Self::to_rgba(v2);
                let (r1, g1, b1, a1) = ( (v1 & 255) as u32, (v1 >> 32 & 255) as u32, (v1 >> 64 & 255) as u32, (v1 >> 96 & 255) as u32);   
                let (r2, g2, b2, a2) = ( (v2 & 255) as u32, (v2 >> 32 & 255) as u32, (v2 >> 64 & 255) as u32, (v1 >> 96 & 255) as u32);   
                let (r, g, b, a) = (
                    (r1 as f64 * (1.-factor) as f64 + r2 as f64 * factor as f64) as u128,
                    (g1 as f64 * (1.-factor) as f64 + g2 as f64 * factor as f64) as u128,
                    (b1 as f64 * (1.-factor) as f64 + b2 as f64 * factor as f64) as u128,
                    (a1 as f64 * (1.-factor) as f64 + a2 as f64 * factor as f64) as u128);
                Self::from_rgba((a << 96) | (r << 64) | (g << 32) | b)
            };

            let x1 = x.floor() as usize;
            let x2: Option<usize> = if x+1. < self.width() as f32 {Option::Some((x.floor()+1.) as usize)} else {Option::None};
            let y1 = y.floor() as usize;
            let y2: Option<usize> = if y+1. < self.height() as f32 {Option::Some((y.floor()+1.) as usize)} else {Option::None};
            match x2{
                Some(x2) => {
                    let v1 = lin_inter(self.index(x1, y1), self.index(x2, y1), x.fract());
                    match y2 {
                        Some(y2) => {
                            let v2 = lin_inter(self.index(x1, y2), self.index(x2, y2), x.fract());
                            lin_inter (&v1, &v2, y.fract())
                        },
                        None => {
                            v1
                        },
                    }
                },
                None => {
                    let v1 = self.index(x1, y1);
                    
                    match y2 {
                        Some(y2) => {
                            let v2 = self.index(x1, y2);
                            lin_inter(v1, v2, y.fract())
                        },
                        None => {
                            Self::copy(v1)
                        },
                    }
                },
            }
        }
        fn get_nearest(&self, x: f32, y: f32) -> Self::Stored{
            Self::copy(self.index(x.round().clamp(0., self.width() as f32 - 1.) as usize, y.round().clamp(0., self.height() as f32 - 1.) as usize))
        }
    }

    pub fn _crop (field: &Field, factor: f64, offset: (usize, usize)) -> Field{
        let mut vector: Vec<bool> = Vec::new();
      let width_bound = min(
            (*field.get_width() as f64/factor+offset.0 as f64) as usize,
            *field.get_width());
        let height_bound = min(
            (*field.get_height() as f64/factor+offset.1 as f64) as usize, 
            *field.get_height());

        for y in (offset.1)..height_bound{

            for x in (offset.0)..width_bound{
                
                vector.push(field[(x, y)]);
            }
        }
        Field::new(vector, width_bound-offset.0, height_bound-offset.1)
    }
    pub fn zoom_bilin_self (field: &mut impl ScalableImage , factor: f32, offset: (f32, f32)){
        let mut i: f32 = offset.0;
        let mut j: f32 = offset.1;
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut buff = field.new_clear();

        while y < field.height(){
            while x < field.width(){
                *buff.index_mut(x, y) = field.get_bilin(i, j);
                i += factor.recip();
                x += 1;
            }
            i = offset.0;
            x = 0;
            j += factor.recip();
            y += 1;
        }
        *field = buff;
    }
     pub fn zoom_nearest_self (field: &mut impl ScalableImage , factor: f32, offset: (f32, f32)){
        let mut i: f32 = offset.0;
        let mut j: f32 = offset.1;
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut buff = field.new_clear();

        while y < field.height(){
            while x < field.width(){
                *buff.index_mut(x, y) = field.get_nearest(i, j);
                i += factor.recip();
                x += 1;
            }
            i = offset.0;
            x = 0;
            j += factor.recip();
            y += 1;
        }
        *field = buff;
    }

}
