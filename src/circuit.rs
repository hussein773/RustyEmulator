use ggez::graphics::Image;
use ggez::mint::Point2;
use ggez::{Context, graphics::Rect};
use ggez::GameResult;
use multimap::MultiMap;
use std::path::Component;

use crate::logic_gates::*;
use crate::source::*;
use crate::structure::*;
use crate::led::*;

#[derive(Debug)]
pub enum LogicElements {
    Gates(LogicGate),
    Source(Source),
    Leds(Led),
    Clock,
    Adders,
    Multiplexers,
    Demultiplexers,
    ShiftRegisters,
    FlipFlops,
    LatchRegisters,
}

impl LogicElements {
    pub fn set_input(&mut self, signal: Vec<Signal>) {
        match self {
            // Handle the case when it's a single input gate
            LogicElements::Gates(logic_gate) => {
                logic_gate.set_input(signal);  // Call the method for a single input gate
            }

            // Handle cases where the logic element is a flip flop or latch register
            LogicElements::FlipFlops | LogicElements::LatchRegisters => {
                todo!()
            }

            // Fallback case: for any other variant of LogicElements
            _ => {
                panic!("set_input is not supported for this LogicElement variant");
            }
        }
    }

    // Function to elaborate the output
    pub fn get_output(&mut self) {
        match self {
            LogicElements::Gates(logic_gate) => {
                logic_gate.get_output();
            }
            _ => todo!(),
        }
    }

    pub fn get_pin(&mut self, ioc:usize, pid:usize) -> &mut Pin{
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.get_pin(ioc, pid),
            LogicElements::Source(source) => source.get_pin(ioc, pid),
            _ => todo!("rest of the componens"),
        }
    }

    pub fn load_image(&mut self, ctx: &mut Context) -> GameResult<()>{
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.load_gate_image(ctx),
            LogicElements::Source(source) => source.load_source_image(ctx),
            LogicElements::Leds(led) => led.load_led_image(ctx),
            _ => todo!(),
        }
    }

    pub fn get_hitbox(&self) -> Rect{
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.hitbox,
            LogicElements::Source(source) => source.hitbox,
            LogicElements::Leds(led) => led.hitbox,
            _ => todo!()
        }
    }

    pub fn get_pins_hitbox(&self) -> Vec<Rect>{
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.gate_pins_hitbox(),
            LogicElements::Source(source) => source.source_pin_hitbox(),
            LogicElements::Leds(led) => led.led_pin_hitbox(),
            _ => todo!()
        }
    }

    pub fn update_postion(&mut self, new_position:Point2<f32>){
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.update_gate_position(new_position),
            LogicElements::Source(source) => source.update_source_position(new_position),
            LogicElements::Leds(led) => led.update_led_position(new_position),
            _ => todo!(),
        }
    }

    pub fn get_image(&self) -> Option<Image>{
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.image.clone(),
            LogicElements::Source(source) => source.image.clone(),
            LogicElements::Leds(led) => led.image.clone(),
            _ => todo!(),
        }
    }

    pub fn get_position(&self) -> Point2<f32>{
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.position,
            LogicElements::Source(source) => source.position,
            LogicElements::Leds(led) => led.position,
            _ => todo!(),
        }
    }

    pub fn get_refpin_pos(&self) -> Point2<f32>{
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.ref_pin_pos,
            LogicElements::Source(source) => source.ref_pin_pos,
            LogicElements::Leds(led) => led.ref_pin_pos,
            _ => todo!(),
        }
    }

    pub fn store_pin_pos(&self, pin_map: &mut MultiMap<(i32, i32), (usize, usize, usize)>){
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.store_pin_pos(pin_map),
            LogicElements::Source(source) => source.store_pin_pos(pin_map),
            LogicElements::Leds(led) => led.store_pin_pos(pin_map),
            _ => todo!(),
        }
    }

}

impl Clone for LogicElements {
    fn clone(&self) -> Self {
        match self {
            LogicElements::Gates(logic_gate) => LogicElements::Gates(logic_gate.clone()),
            LogicElements::Source(source) => LogicElements::Source(source.clone()),
            LogicElements::Leds(led) => LogicElements::Leds(led.clone()),
            _ => todo!(),
        }
    }
}


pub struct Circuit {
    pub components: Vec<LogicElements>,
    pub wires: Vec<Wire>,
    pub component_id: usize,
    pub pin_grid: MultiMap<(i32, i32), (usize, usize, usize)>   
}
impl Circuit {
    // Create a new circuit
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            wires: Vec::new(),
            component_id: 1,
            pin_grid: MultiMap::new(),
        }
    }
    

    //* Function to connect pins of different components together
    pub fn connect(
        &mut self,
        from_cid: usize,
        from_ioc: usize,
        from_pid: usize,
        to_cid: usize,
        to_ioc: usize,
        to_pid: usize,
    ) {
        // Create tuples for both pins
        let from_pin_id = (from_cid, from_ioc, from_pid);
        let to_pin_id = (to_cid, to_ioc, to_pid);
    
        let mut from_wire_index = None;
        let mut to_wire_index = None;
    
        // Find the wires that contain the from_pin and to_pin
        for (index, wire) in self.wires.iter().enumerate() {
            if wire.pins.contains(&from_pin_id) {
                from_wire_index = Some(index);
            }
            if wire.pins.contains(&to_pin_id) {
                to_wire_index = Some(index);
            }
        }
    
        match (from_wire_index, to_wire_index) {
            // Differen indexes = diffeent wire connections => merge the wire connections
            (Some(from_index), Some(to_index)) if from_index != to_index => {
                // Merge the two wires into one
                let mut from_wire = self.wires.remove(from_index);
                let to_wire = self.wires.remove(to_index - (from_index < to_index) as usize);
    
                // Combine pins and segments
                from_wire.pins.extend(to_wire.pins);
                from_wire.pins.push(from_pin_id);
                from_wire.pins.push(to_pin_id);
                from_wire.pins.dedup();
                from_wire.segments.extend(to_wire.segments);
                self.wires.push(from_wire);
            }
            (Some(wire_index), None) | (None, Some(wire_index)) => {
                // Case where only one pin is found in a wire connection
                // Add the pin outside of the wire connection into the connection
                let wire = &mut self.wires[wire_index];
                wire.pins.push(from_pin_id);
                wire.pins.push(to_pin_id);
                wire.pins.dedup();
            }
            (None, None) => {
                // Create a new wire with both pins
                self.wires.push(Wire {
                    pins: vec![from_pin_id, to_pin_id],
                    segments: Vec::new(),
                });
            }
            _ => {}
        }
    }

    // Function to add logic elements to the circuit
    pub fn add_element(&mut self, mut component: LogicElements) {
        // Set the id of the component
        match &mut component {
            LogicElements::Gates(logic_gate) => logic_gate.set_gate_id(self.component_id),
            LogicElements::Source(source) => source.set_id(self.component_id),
            LogicElements:: Leds(led) => led.set_id(self.component_id),
            _ => todo!(),
        }
        self.component_id += 1;
        self.components.push(component);
    }

    // Function to delete an element from the circuit
    pub fn remove_element(&mut self, id: usize) {
        self.components.remove(id);
    }

    pub fn simulate(&mut self) {
        // save the position of all the pins in the pin_grid
        self.pin_grid.clear();
        for component in &self.components {
            component.store_pin_pos(&mut self.pin_grid)
        }

        //----------------------------------------------------------------------------
        println!("Pin Grid Contents:");
        println!("------------------");
        
        for (pos, values) in &self.pin_grid {
            print!("Position: ({}, {}) -> ", pos.0, pos.1);
            for (cid, pid, ioc) in values {
                print!("[CID: {}, PID: {}, IOC: {}] ", cid, pid, ioc);
            }
            println!(); 
        }
        //----------------------------------------------------------------------------

        // Check if all the connections are correct (contain a single source pin)
        /* 
        for (index, wire) in self.wires.iter().enumerate() {
            // Buffer to hold the source pins
            let mut source_pins: Vec<&(usize, usize, usize)> = wire.pins.iter().filter(|&&(_, ioc, _)| ioc == 0).collect();
            if source_pins.len() > 1 {
                panic!(
                    "Short circuit detected in connection {}: {:?}. More than one source pin (ioc = 0) found.",
                    index, wire
                );
            }
    
            // We have a single source pin
            // Propagate the signal of the source pin to the other pin in the connection
            let source = source_pins.pop();
    
            if let Some(&(cid, ioc, pid)) = source {
                // Access the source pin using the get_pin method
                let source_value = self.components[cid-1].get_pin(ioc, pid).value.clone();
                
                /*println!(
                    "Propagating signal from source pin: cid = {}, ioc = {}, pid = {}, value = {:?}",
                    cid, ioc, pid, source_value
                );*/
    
                // Propagate the source signal to all other pins in the connection
                for &(target_cid, target_ioc, target_pid) in &wire.pins {
                    if target_cid == cid && target_ioc == ioc && target_pid == pid {
                        // Skip the source pin itself
                        continue;
                    }
    
                    // Get the target pin and set its value
                    self.components[target_cid-1]
                        .get_pin(target_ioc, target_pid)
                        .value = source_value.clone(); 
                }
            } else {
                // If no source pin exists then set all pins to undefined
                println!(
                    "No source pin found in connection {}: {:?}. Setting all pins to undefined.",
                    index, wire
                );
                for &(target_cid, target_ioc, target_pid) in &wire.pins {
                    self.components[target_cid-1]
                        .get_pin(target_ioc, target_pid)
                        .value = PinValue::Single(Signal::Undefined);
                }
            }
            // After propagating the signals get the output of each component if the componenet
            // needs to evaluate the output
            for component in &mut self.components {
                match component {
                    LogicElements::Source(_) => {
                        // Doesn't need to evaluate the inputs (no input component)
                        continue;
                    }
                    _ => {
                        // Call get_output for all other components
                        component.get_output();
                    }
                }
            }
        }*/
    }

    // TODO: function need to be modified to account for possible components with more than 1 output
    pub fn display_outputs(&self){
        for (index, component) in self.components.iter().enumerate(){
            let mut out = self.components[index].clone();
            println!("output is {:?}", out.get_pin(0, 1).value)
        }
    }
}

