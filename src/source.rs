use std::clone;
use crate::structure::*;
use ggegui::{egui::output, Input};
use ggez::mint::Point2;
use ggez::graphics::{Image, Rect};
use ggez::{Context, GameResult};

#[derive(Debug, Clone)]
pub struct Source {
    id: usize,
    output: Pin,
    pub position: Point2<f32>,
    image: Option<Image>, 
    hitbox: Rect,
    //pin_hitbox: Rect,  
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
                hitbox:  Rect { x: 10.0, y: 10.0, w: 5.0, h: 5.0 },
                },
            position: Point2 { x: 0.0, y: 0.0 },
            image: None,
            hitbox: Rect { x: 0.0, y: 0.0, w: 80.0, h: 80.0 },
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

    pub fn get_source_hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn update_source_position(&mut self, position: Point2<f32>){
        let dx = position.x - self.position.x;
        let dy = position.y - self.position.y;

        self.position = position;
        self.hitbox.x = position.x;
        self.hitbox.y = position.y;
        // Update the pin's hitbox position as well
        self.output.hitbox.x += dx ;
        self.output.hitbox.y += dy ;
    }

    pub fn get_source_image(&self) -> Option<Image>{
        self.image.clone()
    }

    pub fn get_source_position(&self) -> Point2<f32>{
        self.position
    }

    pub fn source_pins_hitbox(&self) -> Vec<Rect>{
        let pin = &self.output;
        vec![pin.hitbox]
    }
}