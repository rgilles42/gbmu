use egui::{ClippedPrimitive, Context, TexturesDelta};
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use pixels::{wgpu, PixelsContext};
use winit::event::WindowEvent;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

pub(crate) struct Framework {
	// State for egui.
	egui_ctx: Context,
	egui_state: egui_winit::State,
	screen_descriptor: ScreenDescriptor,
	renderer: Renderer,
	paint_jobs: Vec<ClippedPrimitive>,
	textures: TexturesDelta,

	// State for the GUI
	pub gui: Gui,
}

impl Framework {
	/// Create egui.
	pub(crate) fn new<T>(
		event_loop: &EventLoopWindowTarget<T>,
		width: u32,
		height: u32,
		scale_factor: f32,
		pixels: &pixels::Pixels,
	) -> Self {
		let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

		let egui_ctx = Context::default();
		let mut egui_state = egui_winit::State::new(event_loop);
		egui_state.set_max_texture_side(max_texture_size);
		egui_state.set_pixels_per_point(scale_factor);
		let screen_descriptor = ScreenDescriptor {
			size_in_pixels: [width, height],
			pixels_per_point: scale_factor,
		};
		let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
		let textures = TexturesDelta::default();
		let gui = Gui::new();

		Self {
			egui_ctx,
			egui_state,
			screen_descriptor,
			renderer,
			paint_jobs: Vec::new(),
			textures,
			gui,
		}
	}

	/// Handle input events from the window manager.
	pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
		let _ = self.egui_state.on_event(&self.egui_ctx, event);
		if let WindowEvent::ScaleFactorChanged { scale_factor, .. } = event {
			self.screen_descriptor.pixels_per_point = *scale_factor as f32;
		}
		if let WindowEvent::Resized(size) = event {
			if size.width > 0 && size.height > 0 {
				self.screen_descriptor.size_in_pixels = [size.width, size.height];
			}
		} 
	}

	/// Prepare egui.
	pub(crate) fn prepare(&mut self, window: &Window) {
		// Run the egui frame and create all paint jobs to prepare for rendering.
		let raw_input = self.egui_state.take_egui_input(window);
		let output = self.egui_ctx.run(raw_input, |egui_ctx| {
			// Draw the demo application.
			self.gui.ui(egui_ctx);
		});

		self.textures.append(output.textures_delta);
		self.egui_state
			.handle_platform_output(window, &self.egui_ctx, output.platform_output);
		self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
	}

	/// Render egui.
	pub(crate) fn render(
		&mut self,
		encoder: &mut wgpu::CommandEncoder,
		render_target: &wgpu::TextureView,
		context: &PixelsContext,
	) {
		// Upload all resources to the GPU.
		for (id, image_delta) in &self.textures.set {
			self.renderer
				.update_texture(&context.device, &context.queue, *id, image_delta);
		}
		self.renderer.update_buffers(
			&context.device,
			&context.queue,
			encoder,
			&self.paint_jobs,
			&self.screen_descriptor,
		);

		// Render egui with WGPU
		{
			let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("egui"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: render_target,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Load,
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});

			self.renderer
				.render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
		}

		// Cleanup
		let textures = std::mem::take(&mut self.textures);
		for id in &textures.free {
			self.renderer.free_texture(id);
		}
	}
}

/// Example application state. A real application will need a lot more state than this.
pub struct Gui {
	pub disp_tileset: bool,
	pub disp_tilemap: bool,
	/// Only show the egui window when true.
	window_open: bool,
}

impl Gui {
	/// Create a `Gui`.
	fn new() -> Self {
		Self {
			disp_tileset: false,
			disp_tilemap: false,
			window_open: false
		}
	}

	/// Create the UI using egui.
	fn ui(&mut self, ctx: &Context) {
		egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("About...").clicked() {
						self.window_open = true;
						ui.close_menu();
					}
				});
				ui.menu_button("Debug", |ui| {
					if ui.button("Open tileset viewer").clicked() {
						self.disp_tileset = true;
						ui.close_menu();
					}
					if ui.button("Open tilemap viewer").clicked() {
						self.disp_tilemap = true;
						ui.close_menu();
					}
				});
			});
		});

		egui::Window::new("About GBMU")
			.open(&mut self.window_open)
			.show(ctx, |ui| {
				ui.label("Thank you for using GBMU!");
				ui.label("A quick and dirty, yet featureful GameBoy emulator written in Rust for educational purposes.");

				ui.separator();

				ui.horizontal(|ui| {
					ui.spacing_mut().item_spacing.x /= 2.0;
					ui.label("© 2023 - rgilles");
					ui.hyperlink("https://github.com/rgilles42/gbmu");
				});
			});
	}
}