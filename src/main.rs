mod structure;
mod logic_gates;

use structure::*;
use logic_gates::*;

use std::path::Component;

use ggez::{input, Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};


fn main() {
    /*/ Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
    */
    let mut xor = LogicGate::new_gate(5,2, false, 1);
    xor.set_input(vec![Signal::Off, Signal::Off]);
    xor.get_output();
    let xor_out = match xor.output.connection {
        ConnectionType::Simple(signal) => signal,
        ConnectionType::Bus(signal) => signal.get(0).cloned().unwrap_or(Signal::Undefined),
    };
    println!("{:?}", xor_out);

    let mut and = LogicGate::new_gate(0, 2, false, 1);
    and.set_input(vec![Signal::On, xor_out]);
    and.get_output();
    let and_out = match and.output.connection {
        ConnectionType::Simple(signal) => signal,
        ConnectionType::Bus(signal) => signal.get(0).cloned().unwrap_or(Signal::Undefined),
    };
    println!("{:?}", and_out);

    let mut not = LogicGate::new_gate(2, 1, false, 1);
    not.set_input(vec![and_out]);
    not.get_output();
    println!("{:?}", not.output);

    /*let circ = Circuit::new()
                        .add(Components::LogicGates(xor))
                        .add(Components::LogicGates(and)) 
                        .connect(xor.output, and.input[0]);
                        //.connect(xor.output.connection, and.input.connection)
                        //.simulate()
*/  
}

// Structure of the logic gate
#[derive(Debug, Clone)]

struct Circuit {
    components: Vec<Box<i32>>,     
    connections: Vec<Vec<Pin>>,   // Store the connections
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            components: Vec::new(),
            connections: Vec::new(),
        }
    }

    // Method to add components to the circuit
    fn add(mut self, component: i32) -> Self {
        todo!()
    }

    // Method to connect components
    fn connect(mut self, from: Pin,  to: Pin) -> Self {
        // Check if both the pins are of the same type (simple / bus)

        // Check if the pins are both source types and panic if true
        todo!()
    }

    // Placeholder for simulate method
    fn simulate(&self) {
        todo!()
    }
}
/* 
struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(ctx)
    }
}
*/