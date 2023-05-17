use std::path::PathBuf;

use egui::{ClippedPrimitive, Context, TexturesDelta, ColorImage, TextureOptions};
use egui_file::FileDialog;
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
		program_icon_rgba: Option<(Vec<u8>, u32, u32)>
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
		let gui = Gui::new(program_icon_rgba);

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

pub struct Gui {
	pub disp_tileset: bool,
	pub disp_tilemap: bool,
	window_open: bool,
	program_icon_image: Option<egui::ColorImage>,
	program_icon: Option<egui::TextureHandle>,
	pub opened_file: Option<PathBuf>,
  	open_file_dialog: Option<FileDialog>,
	pub reset_requested: bool
}

impl Gui {
	fn new(program_icon_rgba: Option<(Vec<u8>, u32, u32)>) -> Self {
		Self {
			disp_tileset: false,
			disp_tilemap: false,
			window_open: false,
			program_icon_image: program_icon_rgba.map(|program_icon_rgba| ColorImage::from_rgba_unmultiplied([program_icon_rgba.1 as usize, program_icon_rgba.2 as usize], &program_icon_rgba.0)),
			program_icon: None,
			opened_file: None,
			open_file_dialog: None,
			reset_requested: false
		}
	}

	/// Create the UI using egui.
	fn ui(&mut self, ctx: &Context) {
		egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Load ROM").clicked() {
						let mut dialog = FileDialog::open_file(self.opened_file.clone());
						dialog.open();
						self.open_file_dialog = Some(dialog);
						ui.close_menu();
					}
					if ui.button("Reset").clicked() {
						self.reset_requested = true;
						ui.close_menu();
					}
					if ui.button("About GBMU").clicked() {
						if let Some(image) = self.program_icon_image.clone() {
							self.program_icon.get_or_insert_with(|| {
								ui.ctx().load_texture("program_logo", image, TextureOptions { magnification: egui::TextureFilter::Nearest, minification: egui::TextureFilter::Nearest })
							});
						}
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
		if let Some(dialog) = &mut self.open_file_dialog {
			if dialog.show(ctx).selected() {
				if let Some(file) = dialog.path() {
					self.opened_file = Some(file);
					self.reset_requested = true;
				};
			}
		}
		egui::Window::new("About GBMU")
		.open(&mut self.window_open)
		.auto_sized()
		.show(ctx, |ui| {
			ui.vertical_centered(|ui| {
				if let Some(texture) = &self.program_icon {
					ui.image(texture, texture.size_vec2() * 2.0);
				}
			});
			ui.add_space(5.0);
			ui.label("A quick and dirty, yet featureful GameBoy emulator written in Rust for educational purposes, as part of a 42 School project.");
			ui.horizontal(|ui| {
				ui.spacing_mut().item_spacing.x /= 2.0;
				ui.label("This software is licensed under the GPL-3.0 License. See");
				ui.hyperlink("www.gnu.org/licenses/gpl-3.0.html");
			});
			ui.separator();
			ui.horizontal(|ui| {
				ui.spacing_mut().item_spacing.x /= 2.0;
				ui.label("By Raphaël Gilles (rgilles) & Mederic Martinet (memartin) -");
				ui.hyperlink("github.com/rgilles42/gbmu");
			});
			ui.horizontal(|ui| {
				ui.spacing_mut().item_spacing.x /= 2.0;
				ui.label("Original logo art by RetroPunkZ -");
				ui.hyperlink("twitter.com/RetroPunkZ1");
			});
		});
	}
}