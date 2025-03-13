extern crate sdl2;

use crate::Color;

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


pub fn draw(canvas: &mut Canvas<Window>) {
    let mut color_picker = 0;
    for i in 0..=9 {
        color_picker += 1;
        for j in 0..=9 {
            

            if color_picker%2 == 0 {
                canvas.set_draw_color(Color::RGB(53, 47, 72)); 
                
            } else{
                canvas.set_draw_color(Color::RGB(240, 240, 240)); 
            }

            let rect = Rectangle {
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