use std::clone;
use crate::structure::*;
use ggegui::{egui::output, Input};
use ggez::mint::Point2;
use ggez::graphics::{Image, Rect};

#[derive(Debug, Clone)]
pub struct Source {
    id: usize,
    output: Pin,
    pub position: Point2<f32>,
    image: Option<Image>, 
    hitbox: Rect  
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
                cid: 0, pid: 1, ioc: 0 
                },
            position: Point2 { x: 0.0, y: 0.0 },
            image: None,
            hitbox: Rect { x: 0.0, y: 0.0, w: 80.0, h: 80.0 }
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
}