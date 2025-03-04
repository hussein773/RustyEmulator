
use crate::structure::*;
use ggez::mint::Point2;
use ggez::graphics::{Image, Rect};
use ggez::{Context, GameResult};
use multimap::MultiMap;

#[derive(Debug, Clone)]
pub struct Source {
    pub id: usize,
    pub output: Pin,
    pub position: Point2<f32>,
    pub image: Option<Image>, 
    pub hitbox: Hitbox,
    pub ref_pin_pos: Point2<f32>,  
}
impl Source {
    pub fn new(value: usize) -> Self{
        Self {
            id: 0,
            output: Pin { 
                value: match value {
                    0 => PinValue::Single(Signal::Off),
                    1 => PinValue::Single(Signal::On),
                    _ => panic!("Invalid source type")
                },
                cid: 0, pid: 1, ioc: 0,
                hitbox:  Hitbox { 
                        rect: Rect { x: 70.5, y: 34.5, w: 5.0, h: 5.0 },
                        r#type: HitboxType::Pin,
                    },
                },
            position: Point2 { x: 0.0, y: 0.0 },
            image: None,
            hitbox: Hitbox { 
                    rect: Rect { x: 46.0, y: 27.0, w: 20.0, h: 20.0 },
                    r#type: HitboxType::Component,
                },
            ref_pin_pos: Point2{ x: 73.0, y: 37.0},
        }
    }

    pub fn get_pin(&mut self, ioc: usize, pid: usize) -> &mut Pin {
        match ioc {
            0 => {
                // For output pins (ioc == 0), the `id` should be checked against the `output` pin's `pid`.
                if self.output.pid == pid {
                    &mut self.output 
                } else {
                    panic!("Output pin with id {} not found", pid);
                }
            }
            _ => panic!("Invalid ioc value: {}. Only 0 (output) is valid.", ioc),
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    
        // Assign the `cid` of the `output` pin to match `self.id`
        self.output.cid = self.id;
    }

    pub fn load_source_image(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.output.value {
            PinValue::Single(signal) => {
                let path = match signal {
                    Signal::On => "/sources/normal/source_high.png",
                    Signal::Off | Signal::Undefined => "/sources/normal/source_low.png",
                };
                let image = Image::from_path(ctx, path)?; 
                self.image = Some(image);
            }
            _ => panic!("Nomral source can't generate a bus output"),
        }
        Ok(())
    }

    pub fn update_source_position(&mut self, position: Point2<f32>){
        let dx = position.x - self.position.x;
        let dy = position.y - self.position.y;

        // Maintain the local offset of the hitbox relative to the source
        let hitbox_offset = Point2{x: self.hitbox.rect.x - self.position.x, y: self.hitbox.rect.y - self.position.y};
        self.position = position;

        // Apply the offset correctly in global coordinates
        self.hitbox.rect.x = position.x + hitbox_offset.x;
        self.hitbox.rect.y = position.y + hitbox_offset.y;

        // Update the pin's hitbox position as well
        self.output.hitbox.rect.x += dx ;
        self.output.hitbox.rect.y += dy ;

        // Update the ref_pin position
        self.ref_pin_pos.x += dx;
        self.ref_pin_pos.y += dy;
    }

    pub fn source_pin_hitbox(&self) -> Vec<&Hitbox>{
        let pin = &self.output;
        vec![&pin.hitbox]
    }

    // Stores the the position of the pin and the ioc/pid
    pub fn store_pin_pos(&self, map: &mut MultiMap<(i32, i32), (usize, usize, usize)>){
        map.insert((self.ref_pin_pos.x as i32, self.ref_pin_pos.y as i32), (self.id, 1, 0));
    }
}