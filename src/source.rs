use std::clone;
use crate::structure::*;
use ggegui::{egui::output, Input};
use ggez::glam::{vec2, Vec2};

#[derive(Debug, Clone)]
pub struct Source {
    id: usize,
    output: Pin,
    pub position: ggez::glam::Vec2,   
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
            position: vec2(0.0, 0.0),
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