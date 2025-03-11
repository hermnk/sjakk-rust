extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::{Canvas};

struct Rectangle {
    x: u32,
    y: u32,
    height: i32,
    width: i32,
}

impl Rectangle {
    fn draw(& self, canvas: &mut Canvas<Window>) {
        canvas.fill_rect(Rect::new(self.x as i32, self.y as i32, self.width as u32, self.height as u32))
            .unwrap();

    }
}

fn board(canvas: &mut Canvas<Window>) {
    let mut color_picker = 0;
    for i in 0..8 {
        color_picker += 1;
        for j in 0..8 {
            

            if color_picker%2 == 0 {
                canvas.set_draw_color(Color::RGB(53, 47, 72)); 
                
            } else{
                canvas.set_draw_color(Color::RGB(240, 240, 240)); 
            }

            let mut rect = Rectangle {
                x: j * 100,   
                y: i * 100,    
                height: 100,
                width: 100,
            };
            rect.draw(canvas);
            color_picker+= 1;
        }
    }
}
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let height = 800;
    let width = 600;

    let window = video_subsystem.window("rust-sdl2 demo", height, width)
        .position_centered()
        .build()
        .unwrap();

    
    let mut canvas = window.into_canvas().build().unwrap();


    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'running: loop{


        canvas.clear();
        board(&mut canvas);
        canvas.present();
        
        
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _=>{}
            }
        }
    }

}
