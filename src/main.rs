use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const WINDOW_SCALE: f32 = 2.;
const ZOOM_SPEED: f32 = 0.2;
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
fn gray_scale(br: f32) -> u32{
    from_u8_rgb((u8::MAX as f32 * br) as u8, (u8::MAX as f32 * br) as u8, (u8::MAX as f32 * br) as u8)
}
fn main() {
    let mut game = game_of_life::game_of_life::GameInstance::new(
        game_of_life::field_presets::r_pentomino(),
        (WIDTH, HEIGHT),
        game_of_life::game_of_life::Options::default(),
    );

    let mut buffer: Vec<u32>;

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
    let mut  scale_factor = 1.;
    let mut prev_mouse_pos = (0., 0.);
    let mut offset = (0., 0.);
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
        let scaled_buffer = scale::zoom_nearest(game.get_field(), scale_factor.into(), offset);
        buffer = scaled_buffer
            .iter()
            .map(|x| gray_scale(*x))
            .collect();
        
        game.new_generation();
        prev_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    }
}


mod scale{
    use game_of_life::game_of_life::Field;
    use std::cmp::min;

    pub fn _crop (field: &Field, factor: f64, offset: (usize, usize)) -> Field{
        let mut vector: Vec<bool> = Vec::new();
        let width_bound = min(
            (field.width as f64/factor+offset.0 as f64) as usize,
            field.width);
        let hight_bound = min(
            (field.hight as f64/factor+offset.1 as f64) as usize, 
            field.hight);

        for y in (offset.1)..hight_bound{

            for x in (offset.0)..width_bound{
                
                vector.push(field[(x, y)]);
            }
        }
        Field{
            vec: vector,
            width: width_bound-offset.0,
            hight: hight_bound-offset.1,
        }
    }
    pub fn zoom_bilin (field: &Field, factor: f32, offset: (f32, f32)) -> Vec<f32>{
        let mut vector: Vec<f32> = Vec::new();
        let width_bound = (field.width as f32/factor+offset.0).min(field.width as f32);
        let hight_bound = (field.hight as f32/factor+offset.1).min(field.hight as f32);
        let mut x: f32 = offset.0;
        let mut y: f32 = offset.1;
        while y < hight_bound{
            while x < width_bound - factor.recip()/2.{
                vector.push(field.get_bilin((x, y)));
                x += factor.recip();
            }
            x = 0.0;
            y += factor.recip();
        }
        vector
    }
    pub fn zoom_nearest(field: &Field, factor: f32, offset: (f32, f32)) -> Vec<f32>{
        let mut vector: Vec<f32> = Vec::new();
        let width_bound = (field.width as f32/factor+offset.0).min(field.width as f32);
        let hight_bound = (field.hight as f32/factor+offset.1).min(field.hight as f32);
        let mut x: f32 = offset.0;
        let mut y: f32 = offset.1;
        while y < hight_bound{
            while x < width_bound - factor.recip()/2.{
                vector.push(if field[(x.round().clamp(0., width_bound-1.) as usize, y.round().clamp(0., hight_bound-1.) as usize)] {1.} else {0.});
                x += factor.recip();
            }
            x = offset.0;
            y += factor.recip();
        }
        vector

    }
}
