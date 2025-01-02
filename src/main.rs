mod circuit;
mod logic_gates;
mod source;
mod structure;

use std::collections::HashMap;
use ggegui::egui::{vec2, Vec2};
use ggez::mint::Point2;
use source::Source;
use structure::*;
use logic_gates::*;
use circuit::*;


use ggegui::{egui, Gui};
use ggez::event::{self, EventHandler};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use ggez::{Context, ContextBuilder, GameResult, input};

 
struct State {
	gui: Gui,
    add_gates: bool,
	selected_gate: Option<String>,
	input_number: u32,
	logicelement_image: Vec<Image>,
	images_paths: HashMap<String, String>,
	logicelement_position: Vec<Point2<f32>>,
	logicelement_hitbox: Vec<Rect>,
	dragging_index: Option<usize>,
	grid_image: Image,
}

impl State {
	pub fn new(ctx: &mut Context) -> Self {

		let mut path = HashMap::new();
        path.insert("AND Gate".to_string(), "/normal/input_2/source_low1.png".to_string());
        path.insert("OR Gate".to_string(), "/normal/input_2/or.png".to_string());
        path.insert("NOT Gate".to_string(), "/normal/input_2/not.png".to_string());
        path.insert("XOR Gate".to_string(), "/normal/input_2/xor.png".to_string());
        path.insert("NAND Gate".to_string(), "/normal/input_2/nand.png".to_string());
        path.insert("NOR Gate".to_string(), "/normal/input_2/nor.png".to_string());
        path.insert("XNOR Gate".to_string(), "/normal/input_2/xnor.png".to_string());

		// The grid image
		let canvas_grid = Image::from_path(ctx, "/utils/grid.png").unwrap();

		Self { 
			gui: Gui::new(ctx), 
			add_gates: false,
			selected_gate: None,
			input_number: 2,
			logicelement_image: Vec::new(),
            images_paths: path,
			logicelement_position: Vec::new(),
			logicelement_hitbox: Vec::new(),
			dragging_index: None,
			grid_image: canvas_grid,
		}
	}

	fn load_gate_image(&mut self, ctx: &mut Context, gate: &str) -> GameResult<()> {
		// Get the path of the image for the selected gate
		if let Some(image_path) = self.images_paths.get(gate) {
			// Load the new image and set it
			let image = Image::from_path(ctx, image_path)?;
			self.logicelement_image.push(image);
	
			// Set a default position for the new image
			let position = Point2 { x: 100.0, y: 100.0 };
			self.logicelement_position.push(position);
	
			// Calculate and store the hitbox
			let rect = ggez::graphics::Rect::new(
				position.x,
				position.y,
				180.0 * 0.5, // Adjust for scaling if necessary
				150.0 as f32 * 0.5, // Adjust for scaling if necessary
			);
			self.logicelement_hitbox.push(rect);
		}
	
		Ok(())
	}

}

impl EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		let gui_ctx = self.gui.ctx();

		// The menu window (main windo)
		egui::Window::new("Menu").show(&gui_ctx, |ui| {
			ui.label("Logic Components");

			// Button to create components
			if ui.button("Logic Gates").clicked() {
                self.add_gates = !self.add_gates;
			}

			// Button for the constants
			if ui.button("Constants").clicked() {
			}

			// Button for the Muxes
			if ui.button("Multiplexers").clicked() {

			}

			if ui.button("quit").clicked() {
				ctx.request_quit();
			}

			// Window to choose the component
			if self.add_gates {
				egui::Window::new("Component Selector").show(&gui_ctx, |ui| {
					// Available gates
					let gates = vec!["AND Gate", "OR Gate", "NOT Gate", "XOR Gate", "NAND Gate", "NOR Gate", "XNOR Gate"];
	
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
                            // Load the appropriate image for the selected gate
							let selected_gate_clone = selected_gate.clone();
							self.load_gate_image(ctx, &selected_gate_clone).expect("Failed to load gate image");
							let gate = LogicGate::new_gate(0, 1, false, 1);
                        }
					}

					// Button to close the window
					if ui.button("Close").clicked() {
						self.add_gates = false;
					}
						
				});
			}
			
			// Drag the component
    		if ctx.mouse.button_pressed(input::mouse::MouseButton::Left) {
        		let mouse_pos = ctx.mouse.position();

        		for (i, hitbox) in self.logicelement_hitbox.iter().enumerate() {
            		if hitbox.contains(mouse_pos) {
                		self.dragging_index = Some(i);
                		break;
            		}
        		}
    		}

    		if let Some(index) = self.dragging_index {
        		let mouse_pos = ctx.mouse.position();
        		self.logicelement_position[index] = mouse_pos;
        		self.logicelement_hitbox[index].x = mouse_pos.x;
        		self.logicelement_hitbox[index].y = mouse_pos.y;
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
        // Draw all gate images by iterating over the `gate_images` vector
        for (i, image) in self.logicelement_image.iter().enumerate() {
			let position = self.logicelement_position[i];
            let draw_params = DrawParam::default()
                .dest(position) // Position the images with some spacing
                .scale(ggez::glam::Vec2::new(0.5, 0.5));   // Adjust the scale as needed
            canvas.draw(image, draw_params);
        }
		
		///////////////////////////////////////////////////////////
		// HITBOXES
		///////////////////////////////////////////////////////////
		for hitbox in &self.logicelement_hitbox {
			let rect_mesh = Mesh::new_rectangle(
				ctx,
				DrawMode::stroke(2.0), // Stroke with 2.0 thickness
				*hitbox,
				Color::RED,           // Use red for visibility
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
	
	let (mut ctx, event_loop) = ContextBuilder::new("game_id", "author").build().unwrap();
	let state = State::new(&mut ctx);
	event::run(ctx, event_loop, state);
	

}
	// Example of a circuit
	/*let mut and = LogicElements::Gates(LogicGate::new_gate(0, 2, false, 1));
	let s1 = LogicElements::Source(Source::new(1));
	let s2 = LogicElements::Source(Source::new(0));
	let mut circ = Circuit::new();
	circ.add_element(and);
	circ.add_element(s1);
	circ.add_element(s2);
	circ.connect(2, 0, 1, 1, 1, 1);
	circ.connect(3, 0, 1, 1, 1, 2);
	circ.simulate();
	circ.display_outputs();
	*/