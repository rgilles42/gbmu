mod memory_bus;
mod cpu;
mod ppu;
mod input;
mod timer;

use std::collections::HashMap;
use std::env::args;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
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
	let event_loop = EventLoop::new();
	let mut windows = HashMap::new();
	let mut pixels = HashMap::new();
	let mut main_input = WinitInputHelper::new();
	windows.insert(WindowTypes::Main, 
		{
			let size = LogicalSize::new(VIEWPORT_PX_WIDTH as f64, VIEWPORT_PX_HEIGHT as f64);
			WindowBuilder::new()
				.with_title("GBMU")
				.with_inner_size(size)
				.with_min_inner_size(size)
				.build(&event_loop)
				.unwrap()
		}
	);
	pixels.insert(windows[&WindowTypes::Main].id(),
		{
			let window_size = windows[&WindowTypes::Main].inner_size();
			let surface_texture = SurfaceTexture::new(window_size.width * 4, window_size.height * 4, &windows[&WindowTypes::Main]);
			Pixels::new(VIEWPORT_PX_WIDTH as u32, VIEWPORT_PX_HEIGHT as u32, surface_texture)?
		}
	);
	let mut memory_bus = MemoryBus::new(args().collect::<Vec<String>>().get(1).map(|str| str.as_str()));
	let mut cpu = Cpu::new();
	let mut ppu = Ppu::new();
	let mut timer = Timer::new();

	let mut nb_ticks = 0;
	let mut debug_enabled = false;

	let mut disp_tilemap = true;
	let mut disp_tileset = true;

	memory_bus.load_dmg_bootrom();
	cpu.tick(&mut memory_bus);									// "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	event_loop.run(move |event, event_loop, control_flow| {
		if let Event::RedrawRequested(win_id) = event {
			if win_id != windows[&WindowTypes::Main].id() {
				let win = windows.iter().find(| (_, win) | win.id() == win_id).map(| (win_type, _) | *win_type);
				if let Some(win_type) = win {
					match win_type {
						WindowTypes::Tileset => {ppu.update_tileset_win(&mut memory_bus, pixels.get_mut(&win_id).unwrap().frame_mut())}
						WindowTypes::Tilemap => {ppu.update_tilemap_win(&mut memory_bus, pixels.get_mut(&win_id).unwrap().frame_mut())}
						_ => {}
					}
				}
			}
            if let Err(err) = pixels[&win_id].render() {
                println!("Window {:?}: pixels.render: {}", win_id, err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
		if let Event::WindowEvent { window_id, event } = &event {
			if window_id != &windows[&WindowTypes::Main].id() {
				if event == &WindowEvent::CloseRequested {
					let win = windows.iter().find(| (_, win) | win.id() == *window_id).map(| (win_type, _) | *win_type);
					if let Some(win_type) = win {
						windows.remove(&win_type);
						pixels.remove(window_id);
						match win_type {
							WindowTypes::Tileset => {disp_tileset = false}
							WindowTypes::Tilemap => {disp_tilemap = false}
							_ => {}
						}
					}
				}
				return;
			}
		}
		if main_input.update(&event) {
			if main_input.key_pressed(VirtualKeyCode::Escape) || main_input.close_requested() || main_input.destroyed() { // || nb_ticks > 23579000 // || cpu.registers.program_counter - 1 == 0xFFFF
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
			if disp_tilemap && windows.len() != 1 + disp_tilemap as usize + disp_tileset as usize {
				windows.insert(WindowTypes::Tilemap, {
					let size = LogicalSize::new(TILEMAP_PX_WIDTH as f64, TILEMAP_PX_HEIGHT as f64);
					WindowBuilder::new()
						.with_title("Tilemap")
						.with_inner_size(size)
						.with_min_inner_size(size)
						.build(&event_loop)
						.unwrap()
				});
				pixels.insert(windows[&WindowTypes::Tilemap].id(), {
					let window_size = windows[&WindowTypes::Tilemap].inner_size();
					let surface_texture = SurfaceTexture::new(window_size.width * 4, window_size.height * 4, &windows[&WindowTypes::Tilemap]);
					Pixels::new(TILEMAP_PX_WIDTH as u32, TILEMAP_PX_HEIGHT as u32, surface_texture).unwrap()
				});
			}
			if disp_tileset && windows.len() != 1 + disp_tilemap as usize + disp_tileset as usize {
				windows.insert(WindowTypes::Tileset, {
					let size = LogicalSize::new(TILESET_VIEWER_PX_WIDTH as f64, TILESET_VIEWER_PX_HEIGHT as f64);
					WindowBuilder::new()
						.with_title("Tileset")
						.with_inner_size(size)
						.with_min_inner_size(size)
						.build(&event_loop)
						.unwrap()
				});
				pixels.insert(windows[&WindowTypes::Tileset].id(), {
					let window_size = windows[&WindowTypes::Tileset].inner_size();
					let surface_texture = SurfaceTexture::new(window_size.width * 4, window_size.height * 4, &windows[&WindowTypes::Tileset]);
					Pixels::new(TILESET_VIEWER_PX_WIDTH as u32, TILESET_VIEWER_PX_HEIGHT as u32, surface_texture).unwrap()
				});
			}
			let mut frame_completed = false;
			while !frame_completed {
				if nb_ticks >= 25030750 {//cpu.registers.program_counter - 1 == 0xc370 {	
					debug_enabled = false;
				}
				if debug_enabled {
					println!("{:x?}", cpu);
					println!("Tick count: {}", nb_ticks);
					std::thread::sleep(std::time::Duration::from_millis(500))
				}
				let nb_cycles = cpu.tick(&mut memory_bus);
				input::tick(&mut memory_bus, &main_input);
				for _ in 0..nb_cycles {
					frame_completed |= ppu.tick(&mut memory_bus, pixels.get_mut(&windows[&WindowTypes::Main].id()).unwrap().frame_mut());
					timer.tick(&mut memory_bus);
				}
				nb_ticks += nb_cycles as u64;
			}
			windows[&WindowTypes::Main].request_redraw();
			if disp_tilemap {
				windows[&WindowTypes::Tilemap].request_redraw();
			}
			if disp_tileset {
				windows[&WindowTypes::Tileset].request_redraw();
			}
		}

	});
}
