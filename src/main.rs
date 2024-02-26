use std::usize;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const WINDOW_SCALE: f32 = 2.;
const ZOOM_SPEED: f32 = 0.2;
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
fn _gray_scale(br: f32) -> u32{
    from_u8_rgb((u8::MAX as f32 * br) as u8, (u8::MAX as f32 * br) as u8, (u8::MAX as f32 * br) as u8)
}

fn to_rgb_tuple (c: u32) -> (u8, u8, u8){
    ( (c & 255) as u8, (c >> 8 & 255) as u8, (c >> 16 & 255) as u8)
}

fn main() {
    let tt = to_rgb_tuple(from_u8_rgb(255, 20, 255));
    println!("{} {} {}", tt.0, tt.1, tt.2);
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
    let mut prev_mouse_pos = (0., 0.);
    let mut offset = (0., 0.);
    let mut screen = ScreenImage::new(vec![0; WIDTH*HEIGHT], *game.get_field().get_width(), *game.get_field().get_height());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let cur_mouse_pos: (f32, f32) =  window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
        let wheel_movement = window.get_scroll_wheel().unwrap_or_else(|| -> (f32, f32) {(0., 0.)});
        let old_size = (WIDTH as f32*WINDOW_SCALE / scale_factor, HEIGHT as f32*WINDOW_SCALE / scale_factor);
        scale_factor += wheel_movement.1*ZOOM_SPEED;
        scale_factor = scale_factor.clamp(1., 20.);
        let new_size = (WIDTH as f32*WINDOW_SCALE / scale_factor, HEIGHT as f32*WINDOW_SCALE / scale_factor);

        offset.0 -= (new_size.0-old_size.0)*cur_mouse_pos.0/(WIDTH as f32*WINDOW_SCALE)/2.;
        offset.1 -= (new_size.1-old_size.1)*cur_mouse_pos.1/(HEIGHT as f32*WINDOW_SCALE)/2.;

        if window.get_mouse_down(minifb::MouseButton::Left){
            offset.0 -= (cur_mouse_pos.0 - prev_mouse_pos.0)/scale_factor/WINDOW_SCALE;
            offset.1 -=  (cur_mouse_pos.1 - prev_mouse_pos.1)/scale_factor/WINDOW_SCALE;
        }

        offset.0 = offset.0.clamp(0., WIDTH as f32 - WIDTH as f32 / scale_factor);
        offset.1 = offset.1.clamp(0., HEIGHT as f32 - HEIGHT as f32 / scale_factor);
        screen.set_vector(game.get_field().get_vec().iter().map(|x: &bool| if *x {u32::MAX} else {0}).collect());
        scale::zoom_bilin_self(&mut screen, scale_factor.into(), offset);
        game.new_generation();
        prev_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
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

        fn to_rgba(color: &u32) -> u32 {
            *color
        }
        fn from_rgba(color: u32) -> u32 {
            color
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
        fn to_rgba(color: &Self::Stored) -> u32;
        fn from_rgba(color: u32) -> Self::Stored;
        fn copy(input: &Self::Stored) -> Self::Stored;
        fn get_vector(&self) -> &Vec<Self::Stored>;
        fn new_clear(&self) -> Self;

        fn get_bilin(&self, x: f32, y: f32) -> Self::Stored{
            let lin_inter = |v1: &Self::Stored, v2: &Self::Stored, factor: f32| -> Self::Stored{
                let v1 = Self::to_rgba(v1);
                let v2 = Self::to_rgba(v2);
                let (r1, g1, b1, a1) = ( (v1 & 255) as u8, (v1 >> 8 & 255) as u8, (v1 >> 16 & 255) as u8, (v1 >> 24 & 255) as u8);
                let (r2, g2, b2, a2) = ( (v2 & 255) as u8, (v2 >> 8 & 255) as u8, (v2 >> 16 & 255) as u8, (v1 >> 24 & 255) as u8);
                let (r, g, b, a) = ((r1 as f32 * (1.-factor) + r2 as f32 * factor) as u32,
                (g1 as f32 * (1.-factor) + g2 as f32 * factor) as u32,
                (b1 as f32 * (1.-factor) + b2 as f32 * factor) as u32,
                (a1 as f32 * (1.-factor) + a2 as f32 * factor) as u32);
                Self::from_rgba((a << 24) | (r << 16) | (g << 8) | b)
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
            Self::copy(self.index(x.round() as usize, y.round() as usize))
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
        let width_bound = (field.width() as f32/factor+offset.0).min(field.width() as f32);
        let height_bound = (field.height() as f32/factor+offset.1).min(field.height() as f32);
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
            println!("{} {}", i, j);
            i = offset.0;
            x = 0;
            j += factor.recip();
            y += 1;
        }
        *field = buff;
    }
}
