use std::collections::HashMap;

use crate::structure::*;
use ggez::mint::Point2;
use ggez::graphics::{Image, Rect};
use ggez::{Context, GameResult};
use multimap::MultiMap;

#[derive(Debug, Clone)]
pub struct Led {
    pub id: usize,
    pub input: Pin,
    pub position: Point2<f32>,
    pub image: Option<Image>, 
    pub hitbox: Hitbox,
    pub ref_pin_pos: Point2<f32>,  
}
impl Led {
    pub fn new() -> Self{
        Self {
            id: 0,
            input: Pin { 
                value: PinValue::Single(Signal::Undefined),
                cid: 0, pid: 1, ioc: 1,
                hitbox:  Hitbox { 
                        rect: Rect { x: 22.5, y: 27.5, w: 5.0, h: 5.0 },
                        r#type: HitboxType::Pin(0, 1, 1),
                    },
                },
            position: Point2 { x: 0.0, y: 0.0 },
            image: None,
            hitbox: Hitbox {
                    rect: Rect { x: 32.0, y: 20.0, w: 20.0, h: 20.0 },
                    r#type: HitboxType::Component
                },
            ref_pin_pos: Point2{ x: 25.0, y: 30.0},
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    
        // Assign the cid of the input pin to match self.id
        self.input.cid = self.id;
        // Asign the cid for the hitbox as well
        if let HitboxType::Pin(a,_ ,_ ) = &mut self.input.hitbox.r#type{
            *a = id;
        }
    }

    pub fn load_led_image(&mut self, ctx: &mut Context) -> GameResult<()> {
        let path = "/leds/normal/led_undefined.png";
        let image = Image::from_path(ctx, path)?; 
        self.image = Some(image);
        Ok(())
    }

    pub fn get_pin(&mut self, pid: usize, ioc: usize) -> &mut Pin {
        match ioc {
            1 => {
                // For input pins (ioc == 1), the `id` should be checked against the `output` pin's `pid`.
                if self.input.pid == pid {
                    &mut self.input 
                } else {
                    panic!("input pin with id {} not found", pid);
                }
            }
            _ => panic!("Invalid ioc value: {}. Only 0 (output) is valid.", ioc),
        }
    }

    pub fn update_led_position(&mut self, position: Point2<f32>){
        let dx = position.x - self.position.x;
        let dy = position.y - self.position.y;

        // Maintain the local offset of the hitbox relative to the source
        let hitbox_offset = Point2{x: self.hitbox.rect.x - self.position.x, y: self.hitbox.rect.y - self.position.y};
        self.position = position;

        // Apply the offset correctly in global coordinates
        self.hitbox.rect.x = position.x + hitbox_offset.x;
        self.hitbox.rect.y = position.y + hitbox_offset.y;

        // Update the pin's hitbox position as well
        self.input.hitbox.rect.x += dx ;
        self.input.hitbox.rect.y += dy ;

        // Update the ref_pin position
        self.ref_pin_pos.x += dx;
        self.ref_pin_pos.y += dy;
    }

    pub fn led_pin_hitbox(&self) -> Vec<Hitbox>{
        let pin = &self.input;
        vec![pin.hitbox.clone()]
    }

    pub fn update_led_image(&mut self, ctx: &mut Context) -> Option<Image> {
        match self.input.value {
            PinValue::Single(signal) => {
                let path = match signal {
                    Signal::On => "/leds/normal/led_high_green.png",
                    Signal::Off => "/leds/normal/led_low.png",
                    Signal::Undefined => "/leds/normal/led_undefined.png",
                };
                
                // Try loading the image and return Some(image) or None if it fails
                match Image::from_path(ctx, path) {
                    Ok(image) => {
                        self.image = Some(image.clone());  // Store the image in self if needed
                        Some(image)
                    }
                    Err(_) => {
                        println!("Failed to load LED image from path: {}", path);
                        None
                    }
                }
            },
            _ => {
                println!("Normal LED can't compute a bus input");
                None
            }
        }
    }

    /*pub fn store_pin_pos(&self, map: &mut MultiMap<(i32, i32), (usize, usize, usize)>){
        map.insert((self.ref_pin_pos.x as i32, self.ref_pin_pos.y as i32), (self.id, 1, 1));
    }*/

}