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
    let mut xor = LogicGate::new_gate(5,3, false, 1);
    xor.set_input(ConnectionType::Simple(vec![Signal::Off, Signal::On, Signal::On]));
    println!("{:?}", xor.input.connection);
    xor.get_output();
    println!("{:?}", xor.output.connection);
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Signal{
    Off, 
    On, 
    Undefined, 
}

enum LogicGates {
    And,
    Or,
    Not,
    Nand, 
    Nor,
    Xor, 
    Xnor,
}

#[derive(Debug)]
enum ConnectionType {
    Simple(Vec<Signal>),
    Bus(Vec<Vec<Signal>>)
}

#[derive(Debug)]
enum PinType {
    Source,
    NotSource
}

struct Pin {
    connection: ConnectionType,
    r#type: PinType
}
// Structure of the logic gate
struct LogicGate {
    input: Pin,
    output: Pin,
    num_input: usize,
    r#type: LogicGates
}

impl LogicGate {
    // Constructor to build a new logic gate
    fn new_gate(gate_type: u32, num_inputs: usize, bus: bool, bits:usize) -> LogicGate {

        let gate = if !bus {
            // Create a simple logic gate
            LogicGate{
                // Check if number bits is > 1 and return an error
                //...

                // As many inputs vec elements as inputs
                input: Pin { connection: ConnectionType::Simple(vec![Signal::Undefined; num_inputs]), r#type: (PinType::NotSource) },
                output: Pin { connection: ConnectionType::Simple(vec![Signal::Undefined; num_inputs]), r#type: (PinType::Source) },
                num_input: num_inputs,
                r#type: match gate_type {
                    0 => LogicGates::And,
                    1 => LogicGates::Or,
                    2 => LogicGates::Not, 
                    3 => LogicGates::Nand,
                    4 => LogicGates::Nor,
                    5 => LogicGates::Xor,
                    6 => LogicGates::Xnor,
                    _ => panic!("Input non valido per la porta logica"),
                }
            }
            
        } else {
            // Create a bus logic gate
            LogicGate{
                // Check if number of bits are %2 
                //...
                
                // Inner vec elements represent the bit numbers, while outer
                // vec elements represent the number of bus
                input: Pin { connection: ConnectionType::Bus(vec![vec![Signal::Undefined; bits]; num_inputs]), r#type: PinType::NotSource },
                output: Pin { connection: ConnectionType::Simple(vec![Signal::Undefined; bits]), r#type: PinType::Source },
                num_input: num_inputs,
                r#type: match gate_type {
                    0 => LogicGates::And,
                    1 => LogicGates::Or,
                    2 => LogicGates::Not, 
                    3 => LogicGates::Nand,
                    4 => LogicGates::Nor,
                    5 => LogicGates::Xor,
                    6 => LogicGates::Xnor,
                    _ => panic!("Input non valido per la porta logica"),
                }
            }
        };
        gate
    }

    // Update the input of the gate
    fn set_input(&mut self, new_inputs: ConnectionType) {
        match (new_inputs, &mut self.input.connection) {
            // Both the current input and new input are Simple
            (ConnectionType::Simple(mut new_simple_vec), ConnectionType::Simple(ref mut simple_vec)) => {
                new_simple_vec.resize(self.num_input, Signal::Undefined);
                *simple_vec = new_simple_vec;
            }
    
            // Both the current input and new input are Bus
            (ConnectionType::Bus(new_bus_vec), ConnectionType::Bus(ref mut bus_vec)) => {
                *bus_vec = new_bus_vec;
            }
    
            // Mismatch between current input type and new input type
            (ConnectionType::Simple(_), ConnectionType::Bus(_)) | (ConnectionType::Bus(_), ConnectionType::Simple(_)) => {
                panic!("Input type mismatch: cannot assign a Simple input to a Bus gate, or vice versa");
            }
        }
    }
    

    // Get the output of a gate
    fn get_output(&mut self) {
        
        // Check the type of the logic gate (bus or simple)
        match &self.input.connection {
            ConnectionType::Simple(simple_vec) => {
                // input into Vec<Signal>
                match self.r#type {
                    LogicGates::And => {
                        if simple_vec.iter().all(|&signal| signal == Signal::On) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::On; 1]);
                        } else if simple_vec.iter().any(|&signal| signal == Signal::Undefined) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Undefined; 1]);
                        } else {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Off; 1]);
                        }
                    }
                    LogicGates::Or => {
                        if simple_vec.iter().any(|&signal| signal == Signal::On) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::On; 1]);
                        } else if simple_vec.iter().any(|&signal| signal == Signal::Undefined) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Undefined; 1]);
                        } else {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Off; 1]);
                        }
                    }
                    LogicGates::Not => {
                        // La porta Not prende solo un input
                        if simple_vec.len() != 1 {
                            panic!("La porta Not richiede esattamente un input");
                        }
                        match simple_vec[0] {
                            Signal::On => self.output.connection = ConnectionType::Simple(vec![Signal::Off; 1]),
                            Signal::Off => self.output.connection = ConnectionType::Simple(vec![Signal::On; 1]),
                            Signal::Undefined => self.output.connection = ConnectionType::Simple(vec![Signal::Undefined; 1]),
                        }
                    }
                    LogicGates::Nand => {
                        if simple_vec.iter().all(|&signal| signal == Signal::On) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Off; 1]);
                        } else if simple_vec.iter().any(|&signal| signal == Signal::Undefined) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Undefined; 1]);
                        } else {
                            self.output.connection = ConnectionType::Simple(vec![Signal::On; 1]);
                        }
                    }
                    LogicGates::Nor => {
                        if simple_vec.iter().all(|&signal| signal == Signal::Off) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::On; 1]);
                        } else if simple_vec.iter().any(|&signal| signal == Signal::Undefined) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Undefined; 1]);
                        } else {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Off; 1]);
                        }
                    }
                    LogicGates::Xor => {
                        let on_count = simple_vec.iter().filter(|&&signal| signal == Signal::On).count();
                        if on_count % 2 == 1 {
                            self.output.connection = ConnectionType::Simple(vec![Signal::On; 1]);
                        } else if simple_vec.iter().any(|&signal| signal == Signal::Undefined) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Undefined; 1]);
                        } else {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Off; 1]);
                        }
                    }
                    LogicGates::Xnor => {
                        let on_count = simple_vec.iter().filter(|&&signal| signal == Signal::On).count();
                        if on_count % 2 == 0 {
                            self.output.connection = ConnectionType::Simple(vec![Signal::On; 1]);
                        } else if simple_vec.iter().any(|&signal| signal == Signal::Undefined) {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Undefined; 1]);
                        } else {
                            self.output.connection = ConnectionType::Simple(vec![Signal::Off; 1]);
                        }
                    }
                } 
                
            }
            ConnectionType::Bus(bus_vec) => {
                match self.r#type {
                    LogicGates::And => {
                        todo!();
                    }
                    LogicGates::Nand => {
                        todo!();
                    }
                    LogicGates::Nor => {
                        todo!();
                    }
                    LogicGates::Not => {
                        todo!();
                    }
                    LogicGates::Or => {
                        todo!();
                    }
                    LogicGates::Xor => {
                        todo!();
                    }
                    LogicGates::Xnor => {
                        todo!();
                    }
                }
            },
        }
 
    }
}

enum Components {
    Adder,
    Buffer,
    Button,
    Clock,
    Constant,
    Decoder,
    Led,
    LogicGates,
    Multiplexer,
    Multiplier,
    Register,
    Subtractor,
    Switch,
    Wire
}

impl Components {
    fn new_component () -> Components {
        todo!();
    }
    fn set_input(){
        todo!();
    }
    fn get_output() -> Vec<Signal> {
        todo!();
    }
}

struct Circuit {
    element: Components,
    
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