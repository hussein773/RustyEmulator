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
    pub hitbox: Rect,
    pub ref_pin_pos: Point2<f32>,  
}
impl Led {
    pub fn new() -> Self{
        Self {
            id: 0,
            input: Pin { 
                value: PinValue::Single(Signal::Undefined),
                cid: 0, pid: 1, ioc: 0,
                hitbox:  Rect { x: 22.5, y: 27.5, w: 5.0, h: 5.0 },
                },
            position: Point2 { x: 0.0, y: 0.0 },
            image: None,
            hitbox: Rect { x: 32.0, y: 20.0, w: 20.0, h: 20.0 },
            ref_pin_pos: Point2{ x: 25.0, y: 30.0},
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    
        // Assign the `cid` of the `output` pin to match `self.id`
        self.input.cid = self.id;
    }

    pub fn load_led_image(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.input.value {
            PinValue::Single(signal) => {
                let path = match signal {
                    Signal::On => "/leds/normal/led_high_green.png",
                    Signal::Off=> "/leds/normal/led_low.png",
                    Signal::Undefined => "/leds/normal/led_undefined.png",
                };
                let image = Image::from_path(ctx, path)?; 
                self.image = Some(image);
            }
            _ => panic!("Nomral led can't compute a bus input"),
        }
        Ok(())
    }

    pub fn update_led_position(&mut self, position: Point2<f32>){
        let dx = position.x - self.position.x;
        let dy = position.y - self.position.y;

        // Maintain the local offset of the hitbox relative to the source
        let hitbox_offset = Point2{x: self.hitbox.x - self.position.x, y: self.hitbox.y - self.position.y};
        self.position = position;

        // Apply the offset correctly in global coordinates
        self.hitbox.x = position.x + hitbox_offset.x;
        self.hitbox.y = position.y + hitbox_offset.y;

        // Update the pin's hitbox position as well
        self.input.hitbox.x += dx ;
        self.input.hitbox.y += dy ;

        // Update the ref_pin position
        self.ref_pin_pos.x += dx;
        self.ref_pin_pos.y += dy;
    }

    pub fn led_pin_hitbox(&self) -> Vec<Rect>{
        let pin = &self.input;
        vec![pin.hitbox]
    }

    pub fn store_pin_pos(&self, map: &mut MultiMap<(i32, i32), (usize, usize, usize)>){
        map.insert((self.ref_pin_pos.x as i32, self.ref_pin_pos.y as i32), (self.id, 1, 1));
    }
}