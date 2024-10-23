mod logic_gates;
mod structure;

use std::collections::HashMap;

use structure::*;
use logic_gates::*;

use ggegui::{egui, Gui};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh, Rect, Text, Canvas, Drawable, draw, DrawMode, Image};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::input::mouse::MouseButton;


struct State {
	gui: Gui,
    add_component: bool,
	selected_gate: Option<String>,
	input_number: u32,
	gate_image: Option<Image>,
	gate_paths: HashMap<String, String>,
}

impl State {
	pub fn new(ctx: &mut Context) -> Self {

		let mut gate_paths = HashMap::new();
        gate_paths.insert("AND Gate".to_string(), "/normal/input_2/and.png".to_string());
        gate_paths.insert("OR Gate".to_string(), "/normal/input_2/or.png".to_string());
        gate_paths.insert("NOT Gate".to_string(), "/normal/input_2/not.png".to_string());
        gate_paths.insert("XOR Gate".to_string(), "/normal/input_2/xor.png".to_string());
        gate_paths.insert("NAND Gate".to_string(), "/normal/input_2/nand.png".to_string());
        gate_paths.insert("NOR Gate".to_string(), "/normal/input_2/nor.png".to_string());
        gate_paths.insert("XNOR Gate".to_string(), "/normal/input_2/xnor.png".to_string());

		Self { 
			gui: Gui::new(ctx), 
			add_component: false,
			selected_gate: None,
			input_number: 2,
			gate_image: None,
            gate_paths: gate_paths,
		}
	}

	// The function to load the gate image
	fn load_gate_image(&mut self, ctx: &mut Context, gate: &str) -> GameResult<()> {
        // Clear the existing image
        self.gate_image = None;

        // Get the path of the selected gate image
        if let Some(image_path) = self.gate_paths.get(gate) {
            // Load the new image and set it
            let image = Image::from_path(ctx, image_path)?;
            self.gate_image = Some(image);
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
                self.add_component = !self.add_component;
			}

			// Button for the constants
			if ui.button("Constants").clicked() {
                /*self.add_or = true;
                if self.or_image.is_none() {
					let image = Image::from_path(ctx,"/or.png");
					self.or_image = Some(image.unwrap());
                }*/
			}

			// Button for the Muxes
			if ui.button("Multiplexers").clicked() {

			}

			if ui.button("quit").clicked() {
				ctx.request_quit();
			}

			// Window to choose the component
			if self.add_component {
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

					// Button 
					if ui.button("Generate").clicked() {
                        if let Some(selected_gate) = &self.selected_gate {
                            // Load the appropriate image for the selected gate
							let selected_gate_clone = selected_gate.clone();
							self.load_gate_image(ctx, &selected_gate_clone).expect("Failed to load gate image");
                        }
					}

					// Button to close the window
					if ui.button("Close").clicked() {
						self.add_component = false;
					}
						
				});
			}
		});
		self.gui.update(ctx);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(217, 217, 217));

        if let Some(image) = &self.gate_image {
            let draw_params = DrawParam::default()
                .dest(ggez::glam::Vec2::new(100.0, 100.0)) // Position the image
                .scale(ggez::glam::Vec2::new(0.5, 0.5));   // Adjust the scale as needed
            canvas.draw(image, draw_params);
        }

		canvas.draw(&self.gui, DrawParam::default().dest(ggez::glam::Vec2::ZERO));
		canvas.finish(ctx)
	}
}

fn main() {
	let (mut ctx, event_loop) = ContextBuilder::new("game_id", "author").build().unwrap();
	let state = State::new(&mut ctx);
	event::run(ctx, event_loop, state);
}