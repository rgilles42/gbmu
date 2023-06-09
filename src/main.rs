mod memory_bus;
mod cpu;
mod ppu;
mod input;
mod timer;

mod gui;

use std::collections::HashMap;
use std::time::{Instant, Duration};
use gui::Framework;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Icon};
use winit_input_helper::WinitInputHelper;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::{Ppu, VIEWPORT_PX_WIDTH, VIEWPORT_PX_HEIGHT, TILEMAP_PX_HEIGHT, TILEMAP_PX_WIDTH, TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT};
use timer::Timer;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum WindowTypes {
	Main, Tileset, Tilemap
}

fn main() -> Result<(), Error> {
	let project_icon = include_bytes!("../assets/gbmu.bmp");
	let program_icon_rgba = {
		let image = image::load_from_memory_with_format(project_icon, image::ImageFormat::Bmp);
		if let Ok(image_contents) = image {
			let image = image_contents.into_rgba8();
			let image_dims = image.dimensions();
			Some((image.into_raw(), image_dims.0, image_dims.1))
		} else {
			None
		}
	};
	let event_loop = EventLoop::new();
	let mut windows = HashMap::new();
	let mut pixels = HashMap::new();
	let mut main_input = WinitInputHelper::new();
	let window_icon = program_icon_rgba.clone().map(|image| Icon::from_rgba(image.0, image.1, image.2));
	let window_icon = if let Some(Ok(icon)) = window_icon {Some(icon)} else {None};
	windows.insert(WindowTypes::Main, 
		{
			WindowBuilder::new()
				.with_title("GBMU")
				.with_inner_size(LogicalSize::new(VIEWPORT_PX_WIDTH as f64 * 4.0, VIEWPORT_PX_HEIGHT as f64 * 4.0 + 50.0))
				.with_min_inner_size(LogicalSize::new(VIEWPORT_PX_WIDTH as f64, VIEWPORT_PX_HEIGHT as f64 + 50.0))
				.with_window_icon(window_icon.clone())
				.build(&event_loop)
				.unwrap()
		}
	);
	pixels.insert(windows[&WindowTypes::Main].id(),
		{
			let window_size = windows[&WindowTypes::Main].inner_size();
			let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &windows[&WindowTypes::Main]);
			Pixels::new(VIEWPORT_PX_WIDTH as u32, VIEWPORT_PX_HEIGHT as u32, surface_texture)?
		}
	);
	let mut framework = {
		let window_size = windows[&WindowTypes::Main].inner_size();
		let scale_factor = windows[&WindowTypes::Main].scale_factor() as f32;
		Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels[&windows[&WindowTypes::Main].id()],
			program_icon_rgba
        )
	};
	let mut memory_bus: Option<MemoryBus> = None;
	let mut cpu = Cpu::new();
	let mut ppu = Ppu::new();
	let mut timer = Timer::new();

	let mut next_redraw = Instant::now() + Duration::from_micros(16665);
	let mut frame_completed = false;
									// "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	event_loop.run(move |event, event_loop, control_flow| {
		match &event {
			Event::WindowEvent { window_id, event } => {
				if window_id == &windows[&WindowTypes::Main].id() {
					framework.handle_event(&event);
				} else {
					if event == &WindowEvent::CloseRequested {
						let win = windows.iter().find(| (_, win) | win.id() == *window_id).map(| (win_type, _) | *win_type);
						if let Some(win_type) = win {
							windows.remove(&win_type);
							pixels.remove(window_id);
							match win_type {
								WindowTypes::Tileset => {framework.gui.disp_tileset = false}
								WindowTypes::Tilemap => {framework.gui.disp_tilemap = false}
								_ => {}
							}
						}
					}
					return;
				}
			}
			Event::RedrawRequested(win_id) => {
				let render_result;
				let win = windows.iter().find(| (_, win) | win.id() == *win_id).map(| (win_type, _) | *win_type);
				if let Some(win_type) = win {
					match win_type {
						WindowTypes::Tileset => {
							if let Some(memory_bus) = &mut memory_bus {
								ppu.update_tileset_win(memory_bus, pixels.get_mut(win_id).unwrap().frame_mut());
							}
							render_result = pixels[&win_id].render();
						}
						WindowTypes::Tilemap => {
							if let Some(memory_bus) = &mut memory_bus {
								ppu.update_tilemap_win(memory_bus, pixels.get_mut(win_id).unwrap().frame_mut());
							}
							render_result = pixels[&win_id].render();
						}
						WindowTypes::Main => {
							framework.prepare(&windows[&WindowTypes::Main], &cpu);
							render_result = pixels[&win_id].render_with(|encoder, render_target, context| {
								context.scaling_renderer.render(encoder, render_target);
								framework.render(encoder, render_target, context);
								Ok(())
							});
							next_redraw = Instant::now() + Duration::from_micros(16665);
							frame_completed = false;
						}
					}
					if let Err(err) = render_result {
						println!("Window {:?}: pixels.render: {}", win_id, err);
						*control_flow = ControlFlow::Exit;
						return;
					}
				}
			}
			_ => {}
		}
		if main_input.update(&event) {
			if main_input.key_pressed(VirtualKeyCode::Escape) || main_input.close_requested() || main_input.destroyed() {
                *control_flow = ControlFlow::Exit;
                return;
            }
			if let Some(size) = main_input.window_resized() {
                if let Err(err) = pixels.get_mut(&windows[&WindowTypes::Main].id()).unwrap().resize_surface(size.width, size.height) {
                    println!("pixels.resize_surface: {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
			if framework.gui.disp_tilemap && windows.len() != 1 + framework.gui.disp_tilemap as usize + framework.gui.disp_tileset as usize {
				windows.insert(WindowTypes::Tilemap, {
					let size = LogicalSize::new(TILEMAP_PX_WIDTH as f64 * 2.0, TILEMAP_PX_HEIGHT as f64 * 2.0);
					WindowBuilder::new()
						.with_title("Tilemap")
						.with_inner_size(size)
						.with_min_inner_size(size)
						.with_resizable(false)
						.with_window_icon(window_icon.clone())
						.build(&event_loop)
						.unwrap()
				});
				pixels.insert(windows[&WindowTypes::Tilemap].id(), {
					let window_size = windows[&WindowTypes::Tilemap].inner_size();
					let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &windows[&WindowTypes::Tilemap]);
					Pixels::new(TILEMAP_PX_WIDTH as u32, TILEMAP_PX_HEIGHT as u32, surface_texture).unwrap()
				});
			}
			if framework.gui.disp_tileset && windows.len() != 1 + framework.gui.disp_tilemap as usize + framework.gui.disp_tileset as usize {
				windows.insert(WindowTypes::Tileset, {
					let size = LogicalSize::new(TILESET_VIEWER_PX_WIDTH as f64 * 4.0, TILESET_VIEWER_PX_HEIGHT as f64 * 4.0);
					WindowBuilder::new()
						.with_title("Tileset")
						.with_inner_size(size)
						.with_min_inner_size(size)
						.with_resizable(false)
						.with_window_icon(window_icon.clone())
						.build(&event_loop)
						.unwrap()
				});
				pixels.insert(windows[&WindowTypes::Tileset].id(), {
					let window_size = windows[&WindowTypes::Tileset].inner_size();
					let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &windows[&WindowTypes::Tileset]);
					Pixels::new(TILESET_VIEWER_PX_WIDTH as u32, TILESET_VIEWER_PX_HEIGHT as u32, surface_texture).unwrap()
				});
			}
			if framework.gui.reset_requested {
				memory_bus = None;
				cpu = Cpu::new();
				ppu = Ppu::new();
				timer = Timer::new();
				framework.gui.reset_requested = false;
			}
			if let None = &memory_bus {
				if let Some(path) = &framework.gui.opened_file {
					memory_bus = Some(MemoryBus::new(path.to_str(), framework.gui.force_dmg));
					cpu.tick(memory_bus.as_mut().unwrap());
				}
			} else {
				while !frame_completed && !framework.gui.is_execution_paused && (!framework.gui.debugger_window_open || framework.gui.is_debugger_stepping_instr || framework.gui.is_debugger_stepping_frame ){
					framework.gui.is_debugger_stepping_instr = false;
					let nb_cycles = cpu.tick(memory_bus.as_mut().unwrap());
					input::tick(memory_bus.as_mut().unwrap(), &main_input);
					let mut i = 0;
					let mut ppu_is_halting_cpu = false;
					while i < nb_cycles || ppu_is_halting_cpu {
						timer.tick(memory_bus.as_mut().unwrap());
						let res = if !memory_bus.as_ref().unwrap().is_double_speed || i % 2 == 0 {
													ppu.tick(memory_bus.as_mut().unwrap(), pixels.get_mut(&windows[&WindowTypes::Main].id()).unwrap().frame_mut())
												} else {(false, ppu_is_halting_cpu)};
						frame_completed |= res.0;
						ppu_is_halting_cpu = res.1;
						i += 1;
					}
				}
				if frame_completed {
					framework.gui.is_debugger_stepping_frame = false;
				}
			}
			if Instant::now() >= next_redraw {
				windows[&WindowTypes::Main].request_redraw();
				if framework.gui.disp_tilemap {
					windows[&WindowTypes::Tilemap].request_redraw();
				}
				if framework.gui.disp_tileset {
					windows[&WindowTypes::Tileset].request_redraw();
				}
			}
		}
	});
}
