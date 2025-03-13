extern crate sdl2;


use sdl2::image::{InitFlag, LoadTexture};
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::path::Path;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;



mod board;

struct Sprite {

}


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let height = 800;
    let width = 800;

    let _image_context =sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem.window("rust-sdl2 demo", height, width)
        .position_centered()
        .build()
        .unwrap();

    
    let mut canvas = window.into_canvas().build().unwrap();

 
    let texture_creator = canvas.texture_creator();
    let texture = match texture_creator.load_texture(Path::new("src/media/bishop_b.png")) {
        Ok(texture) => texture,
        Err(err) => panic!("Failed to load texture: {}", err),
    };

    let target_rect = Rect::new(100,100,100,100);
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut mx: i32;
    let mut my: i32;
    'running: loop{
    
        

        canvas.clear();
        board::draw(&mut canvas);
        canvas.copy(&texture, None, target_rect);
        canvas.present();
        
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn,x,y,..} =>{
                    match mouse_btn{
                        MouseButton::Left => {
                            mx = ((x as f64).ceil()/100.0) as i32;
                            my = ((y as f64).ceil()/100.0) as i32;
                            println!("{mx}, {my}");
                        }, _=> {}
                    }
                    
                }
                |
                _=>{}
            }
        }
    }

}
