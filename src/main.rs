mod circuit;
mod logic_gates;
mod source;
mod structure;

use source::Source;
use structure::*;
use logic_gates::*;
use circuit::*;


use ggegui::{egui, Gui};
use ggez::event::{self, EventHandler};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, Image, Mesh};
use ggez::{Context, ContextBuilder, GameResult, input};
use ggez::mint::Point2;

 
struct State {
	gui: Gui,
	circuit: Circuit,
    add_element: Vec<bool>,
	selected_gate: Option<String>,
	selected_source: Option<String>,
	input_number: u32,
	dragging_index: Option<usize>,
	drag_offset: Option<Point2<f32>>,
	grid_image: Image,
	grid_offset: Point2<f32>,
}

impl State {
	pub fn new(ctx: &mut Context) -> Self {
		// The grid image
		let canvas_grid = Image::from_path(ctx, "/utils/grid1.png").unwrap();

		Self { 
			gui: Gui::new(ctx),
			circuit: Circuit::new(), 
			add_element: vec![false, false, false, false],
			selected_gate: None,
			selected_source: None,
			input_number: 2,
			dragging_index: None,
			drag_offset: None,
			grid_image: canvas_grid,
			grid_offset: Point2 { x: 0.0, y: 0.0 },
		}
	}

}

impl EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		let gui_ctx = self.gui.ctx();

		// The menu window (main window)
		egui::Window::new("Menu").show(&gui_ctx, |ui| {
			ui.label("Logic Components");

			// Button to create components
			if ui.button("Logic Gates").clicked() {
                self.add_element[0] = !self.add_element[0];
			}

			// Button for the constants
			if ui.button("Constants").clicked() {
				self.add_element[1] = !self.add_element[1];
			}

			// Button for the wires
			if ui.button("Wire").clicked() {
				self.add_element[2] = !self.add_element[2];
			}

			// Button for the Muxes
			if ui.button("Multiplexers").clicked() {
				self.add_element[3] = !self.add_element[3];
			}

			if ui.button("quit").clicked() {
				ctx.request_quit();
			}

			// Window to choose the logic gates
			if self.add_element[0] {
				egui::Window::new("Logic Gate Selector").show(&gui_ctx, |ui| {
					// Available gates
					let gates = vec!["AND Gate", "OR Gate", "NOT Gate", "NAND Gate", "NOR Gate", "XOR Gate", "XNOR Gate"];
	
					// Iterate over gates and create buttons
					ui.label("Select a gate:");
					for gate in &gates {
						if ui.button(*gate).clicked() {
							// Flag that indicates which gate is selected
							self.selected_gate = Some(gate.to_string());
						}
					}
	
					// Input number selection (2 to 16)
					ui.label("Select the number of inputs:");
					ui.add(egui::DragValue::new(&mut self.input_number)
						.clamp_range(2..=16)
						.speed(1));
	
					// Display selected gate and input number
					if let Some(selected_gate) = &self.selected_gate {
						ui.label(format!("Selected Gate: {}", selected_gate));
						ui.label(format!("Number of Inputs: {}", self.input_number));
					}

					// Button to generate the gate
					if ui.button("Generate").clicked() {
                        if let Some(selected_gate) = &self.selected_gate {
							// Get the gate variant
							let gate_type: u32;
							match selected_gate.as_str() {
								"AND Gate" => gate_type = 0,
								"OR Gate" => gate_type = 1,
								"NOT Gate" => gate_type = 2,
								"NAND Gate" => gate_type = 3,
								"NOR Gate" => gate_type = 4,
								"XOR Gate" => gate_type = 5,
								"XNOR Gate" => gate_type = 6,
								_ => panic!("Invalid Gate")
							}
                            // Build the Gate component in the circuit
							let mut gate = LogicElements::Gates(LogicGate::new_gate(gate_type, self.input_number as usize, false, 1));
							let _ = gate.load_image(ctx);
							self.circuit.add_element(gate);
                        }
					}

					// Button to close the window
					if ui.button("Close").clicked() {
						self.add_element[0] = false;
					}
						
				});
			}

			// Window to choose the source element
			if self.add_element[1] {
				egui::Window::new("Sources").show(&gui_ctx, |ui| {
					// Two buttons for selecting high or low source
					if ui.button("High Source").clicked() {
						self.selected_source = Some("High".to_string());
					}
					if ui.button("Low Source").clicked() {
						self.selected_source = Some("Low".to_string());
					}
			
					// If a source is selected, show the "Generate" button
					if let Some(selected_source) = &self.selected_source {
						ui.label(format!("Selected Source: {}", selected_source));
						let value;
						if ui.button("Generate").clicked() {
							// Generate the selected source
							match selected_source.as_str() {
								"High" => value = 1,
								"Low" => value = 0,
								_ => panic!("Invalid source type")
							}
							let mut source = LogicElements::Source(Source::new(value));
							let _ = source.load_image(ctx);
							self.circuit.add_element(source);
						}
					}
				});
			}
			
			// Drag the component
			if ctx.mouse.button_pressed(input::mouse::MouseButton::Left) {
				let mouse_pos = ctx.mouse.position();
			
				// Check if we are initiating a drag
				if self.dragging_index.is_none() {
					for (i, component) in self.circuit.components.iter().enumerate() {
						let hitbox = component.get_hitbox();
						if hitbox.contains(mouse_pos) {
							// Set the drag offset and index
							let component_pos = component.get_position();
							self.drag_offset = Some(Point2 {
								x: mouse_pos.x - component_pos.x,
								y: mouse_pos.y - component_pos.y,
							});
							self.dragging_index = Some(i);
							break;
						}
					}
				}
			}
			
			// If dragging then update the position of the selected component with the offset
			if let Some(index) = self.dragging_index {
				let mouse_pos = ctx.mouse.position();
				if let Some(offset) = self.drag_offset {
					let new_position = Point2 {
						x: mouse_pos.x - offset.x,
						y: mouse_pos.y - offset.y,
					};
					self.circuit.components[index].update_postion(new_position);
				}
			}

    		if ctx.mouse.button_just_released(input::mouse::MouseButton::Left) {
        		self.dragging_index = None;
    		}
		});
		self.gui.update(ctx);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = Canvas::from_frame(ctx, None);
		// Draw the grid
		canvas.draw(&self.grid_image, DrawParam::default());
        // Draw all the components's images by iterating over the components vec
		for component in self.circuit.components.iter() {
			if let Some(image) = component.get_image() {
				let draw_params = DrawParam::default()
					.dest(component.get_position())
					.scale(ggez::glam::Vec2::new(0.5, 0.5)); 
				canvas.draw(&image, draw_params);
			}
		}
		
		///////////////////////////////////////////////////////////
		// HITBOXES
		///////////////////////////////////////////////////////////
		for component in &self.circuit.components {
			let hitbox = component.get_hitbox();
			let rect_mesh = Mesh::new_rectangle(
				ctx,
				DrawMode::stroke(2.0),
				hitbox,
				Color::RED,
			)?;
			canvas.draw(&rect_mesh, DrawParam::default());
		}
		///////////////////////////////////////////////////////////
		
        // Draw the GUI
        canvas.draw(&self.gui, DrawParam::default());
        canvas.finish(ctx)
	}
}


fn main() {
	
	let (mut ctx, event_loop) = ContextBuilder::new("Rusty Emulator", "author").build().unwrap();
	let state = State::new(&mut ctx);
	event::run(ctx, event_loop, state);
	
}