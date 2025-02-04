mod circuit;
mod logic_gates;
mod source;
mod structure;

use std::vec;
use source::Source;
use structure::*;
use logic_gates::*;
use circuit::*;

use ggegui::egui::{vec2, Align, Layout, Vec2};
use ggegui::{egui, Gui};
use ggez::event::{self, EventHandler};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use ggez::{Context, ContextBuilder, GameResult, input, mint::Point2, conf::{Conf, WindowSetup}};

 
const UI_BUTTON_SIZE: Vec2 = vec2(150.0, 30.0);
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
	wire_start: Option<Point2<f32>>
}

impl State {
	pub fn new(ctx: &mut Context) -> Self {
		// The grid image
		let canvas_grid = Image::from_path(ctx, "/utils/grid1.png").unwrap();

		Self { 
			gui: Gui::new(ctx),
			circuit: Circuit::new(), 
			add_element: vec![false, false, false, false, false],
			selected_gate: None,
			selected_source: None,
			input_number: 2,
			dragging_index: None,
			drag_offset: None,
			grid_image: canvas_grid,
			wire_start: None,
		}
	}

}

impl EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		let gui_ctx = self.gui.ctx();

		// The menu window (main window)
		egui::Window::new("Menu").show(&gui_ctx, |ui| {
			ui.label("Logic Components");

			ui.with_layout(Layout::top_down(Align::Min), |ui| {			
				// Button to create components
				if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("Logic Gates")).clicked() {
                	self.add_element[0] = !self.add_element[0];
				}

				// Button for the constants
				if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("Sources")).clicked() {
					self.add_element[1] = !self.add_element[1];
				}

				// Button for the wires
				if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("Wire tool")).clicked() {
					self.add_element[2] = !self.add_element[2];
				}

				// Button for the Muxes
				if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("Multiplexers")).clicked() {
					self.add_element[3] = !self.add_element[3];
				}

				// Edit tool button
				if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("Edit tool")).clicked() {
					self.add_element[4] = !self.add_element[4];
				}

				if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("Quit")).clicked() {
					ctx.request_quit();
				}
			});
		
			//* Window to choose the logic gates
			if self.add_element[0] {
				egui::Window::new("Logic Gate Selector")
					.resizable(false)
					.default_width(100.0)
					.show(&gui_ctx, |ui| {
						ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
						// Gate selection section
							ui.label("Select a logic gate:");
							ui.separator();
							let button_size = egui::vec2(150.0, 30.0); // Uniform button size
							let gates = vec![
								"AND Gate", "OR Gate", "NOT Gate", 
								"NAND Gate", "NOR Gate", "XOR Gate", "XNOR Gate",
							];
							for gate in &gates {
								if ui.add_sized(button_size, egui::Button::new(*gate)).clicked() {
									self.selected_gate = Some(gate.to_string());
								}
							}
			
							ui.separator();
			
							// Number of inputs section
							ui.label("Select the number of inputs:");
							if let Some(gate) = &self.selected_gate {
								match gate.as_str() {
									"NOT Gate" => {
										self.input_number = 1;
										ui.label("NOT Gate only allows 1 input.");
									}
									_ => {
										ui.add(egui::DragValue::new(&mut self.input_number)
											.clamp_range(2..=16)
											.speed(1));
									}
								}
							}
			
							ui.separator();
			
							// Display selected gate and input number
							if let Some(selected_gate) = &self.selected_gate {
								ui.label(format!("Selected Gate: {}", selected_gate));
								ui.label(format!("Number of Inputs: {}", self.input_number));
							}
			
							ui.separator();
			
							// Generate button
							if ui.add_sized(button_size, egui::Button::new("Generate")).clicked() {
								if let Some(selected_gate) = &self.selected_gate {
									let gate_type: u32 = match selected_gate.as_str() {
										"AND Gate" => 0,
										"OR Gate" => 1,
										"NOT Gate" => 2,
										"NAND Gate" => 3,
										"NOR Gate" => 4,
										"XOR Gate" => 5,
										"XNOR Gate" => 6,
										_ => panic!("Invalid Gate"),
									};
									let mut gate = LogicElements::Gates(
										LogicGate::new_gate(gate_type, self.input_number as usize, false, 1),
									);
									let _ = gate.load_image(ctx); // Load the gate image
									self.circuit.add_element(gate); // Add the gate to the circuit
								}
							}
			
							// Close button
							if ui.add_sized(button_size, egui::Button::new("Close")).clicked() {
								self.add_element[0] = false;
							}
						});
					});
				}

			//* Window to choose the source element
			if self.add_element[1] {
				egui::Window::new("Sources")
					.resizable(false) 
					.default_width(100.0)
					.show(&gui_ctx, |ui| {
						ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {		
							if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("High Source")).clicked() {
								self.selected_source = Some("High".to_string());
							}
							if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("Low Source")).clicked() {
								self.selected_source = Some("Low".to_string());
							}
			
							ui.separator(); // Add separator between buttons and other sections
			
							// If a source is selected, show the "Generate" button
							if let Some(selected_source) = &self.selected_source {
								ui.label(format!("Selected Source: {}", selected_source));
			
								//ui.separator();
			
								if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("Generate")).clicked() {
									// Generate the selected source
									let value = match selected_source.as_str() {
										"High" => 1,
										"Low" => 0,
										_ => panic!("Invalid source type"),
									};
			
									let mut source = LogicElements::Source(Source::new(value));
									let _ = source.load_image(ctx);
									self.circuit.add_element(source);
								}
							}
			
							//Close button for this window, if necessary
							if ui.add_sized(UI_BUTTON_SIZE, egui::Button::new("Close")).clicked() {
								self.add_element[1] = false;
							}
						});
					});
			}

			//* Wire logic
			if self.add_element[2] {
				// The grid size indicates the distance between 2 points in the grid
				let grid_size = 10.0;
				let mouse_pos = ctx.mouse.position();
				let snapped_mouse_pos = Point2 { 
					x: (mouse_pos.x / grid_size).round() * grid_size, 
					y: (mouse_pos.y / grid_size).round() * grid_size 
				};
			
				if ctx.mouse.button_pressed(input::mouse::MouseButton::Left) {
					// Start drawing if not already started
					if self.wire_start.is_none() {
						self.wire_start = Some(snapped_mouse_pos);
					}
				} else if ctx.mouse.button_just_released(input::mouse::MouseButton::Left) {
					if let Some(start_point) = self.wire_start.take() {
						if start_point != snapped_mouse_pos {
							// Calculate the greater deviation
							let dx = (snapped_mouse_pos.x - start_point.x).abs();
							let dy = (snapped_mouse_pos.y - start_point.y).abs();
			
							let segment = if dx >= dy {
								// Horizontal segment
								WireSegment {
									start: start_point,
									end: Point2 { x: snapped_mouse_pos.x, y: start_point.y },
									hitbox: Rect {
										x: start_point.x.min(snapped_mouse_pos.x),
										y: start_point.y - 5.0,
										w: dx,
										h: 10.0,
									},
								}
							} else {
								// Vertical segment
								WireSegment {
									start: start_point,
									end: Point2 { x: start_point.x, y: snapped_mouse_pos.y },
									hitbox: Rect {
										x: start_point.x - 5.0,
										y: start_point.y.min(snapped_mouse_pos.y),
										w: 10.0,
										h: dy,
									},
								}
							};
			
							// Store the wire with a single segment
							self.circuit.wires.push(Wire {
								pins: vec![],
								segments: vec![segment],
							});
						}
					}
				}
			}
			
			//* Logic to drag the component
			if ctx.mouse.button_pressed(input::mouse::MouseButton::Left) {
    			let mouse_pos = ctx.mouse.position();

    			// Initiate dragging
    			if self.dragging_index.is_none() && !self.add_element[2] {
        			for (i, component) in self.circuit.components.iter().enumerate() {
            			let hitbox = component.get_hitbox();
            			if hitbox.contains(mouse_pos) {
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

    			// Update position while dragging
    			if let Some(index) = self.dragging_index {
        			if let Some(offset) = self.drag_offset {
            			let new_position = Point2 {
                			x: mouse_pos.x - offset.x,
                			y: mouse_pos.y - offset.y,
            			};
            		self.circuit.components[index].update_postion(new_position);
        			}
    			}
			}

			// Snap to grid on mouse release
			if ctx.mouse.button_just_released(input::mouse::MouseButton::Left) {
    			if let Some(index) = self.dragging_index {
        			let component = &self.circuit.components[index];

        			// Get the current position of the reference pin
        			let reference_pin = component.get_refpin_pos();

        			// Find the nearest grid point
        			let snapped_x = (reference_pin.x / 10.0).round() * 10.0;
        			let snapped_y = (reference_pin.y / 10.0).round() * 10.0;

        			// Calculate the displacement 
        			let displacement = Point2 {
            			x: snapped_x - reference_pin.x,
            			y: snapped_y - reference_pin.y,
        			};

        			// Apply the displacement to the component
        			let new_position = Point2 {
            			x: component.get_position().x + displacement.x,
            			y: component.get_position().y + displacement.y,
        			};

        			self.circuit.components[index].update_postion(new_position);
    			}

    			// Clear drag state
    			self.dragging_index = None;
    			self.drag_offset = None;
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
    	// DRAW WIRES
    	///////////////////////////////////////////////////////////
    	for wire in &self.circuit.wires {
        	for segment in &wire.segments {
				// Segments of the wire
            	let line = vec![
                	Point2 { x: segment.start.x, y: segment.start.y},
                	Point2 { x: segment.end.x, y: segment.end.y},
            	];
            	let line_mesh = Mesh::new_line(ctx, &line, 4.0, Color::BLUE)?;
            	canvas.draw(&line_mesh, DrawParam::default());

				// Hitbox of the wire
				/*let hitbox_mesh = Mesh::new_rectangle(
					ctx,
					DrawMode::stroke(1.0),
					segment.hitbox,
					Color::RED, 
				)?;
				canvas.draw(&hitbox_mesh, DrawParam::default());*/
        	}
    	}
    	///////////////////////////////////////////////////////////
		
		///////////////////////////////////////////////////////////
		// HITBOXES
		///////////////////////////////////////////////////////////
		for component in &self.circuit.components {
			// Get the component's main hitbox and pin hitboxes
			let hitbox = component.get_hitbox();
			let pins = component.get_pins_hitbox();
		
			// Draw the component's main hitbox
			let component_hitbox = Mesh::new_rectangle(
				ctx,
				DrawMode::stroke(2.0),
				hitbox,
				Color::RED,
			)?;
			//canvas.draw(&component_hitbox, DrawParam::default());
		
			// Draw each pin's hitbox
			for pin_hitbox in pins {
				let pin_mesh = Mesh::new_rectangle(
					ctx,
					DrawMode::stroke(2.0),
					pin_hitbox,
					Color::BLUE, 
				)?;
				canvas.draw(&pin_mesh, DrawParam::default());
			}
		}
		///////////////////////////////////////////////////////////
		
        // Draw the GUI
        canvas.draw(&self.gui, DrawParam::default());
        canvas.finish(ctx)
	}
}


fn main() {
	let window_setup = WindowSetup {
        title: "Rusty Simulator".to_string(), 
        ..Default::default()
    };

    let conf = Conf {
        window_setup,
        ..Default::default()
    };
	let (mut ctx, event_loop) = ContextBuilder::new("Rusty Emulator", "author").default_conf(conf).build().unwrap();
	let state = State::new(&mut ctx);
	event::run(ctx, event_loop, state);
	
}