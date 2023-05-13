mod memory_bus;
mod cpu;
mod ppu;
mod input;
mod timer;

use std::env::args;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::{Ppu, VIEWPORT_PX_WIDTH, VIEWPORT_PX_HEIGHT, TILEMAP_PX_HEIGHT, TILEMAP_PX_WIDTH, TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT};
use timer::Timer;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;



fn main() -> Result<(), Error> {
	let event_loop = EventLoop::new();
	let mut input = WinitInputHelper::new();
	let main_window = {
		let size = LogicalSize::new(VIEWPORT_PX_WIDTH as f64, VIEWPORT_PX_HEIGHT as f64);
		WindowBuilder::new()
			.with_title("GBMU")
			.with_inner_size(size)
			.with_min_inner_size(size)
			.build(&event_loop)
			.unwrap()
	};
	let mut main_pixels = {
		let window_size = main_window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width * 4, window_size.height * 4, &main_window);
		Pixels::new(VIEWPORT_PX_WIDTH as u32, VIEWPORT_PX_HEIGHT as u32, surface_texture)?
	};
	// let tilemap_window = {
	// 	let size = LogicalSize::new(TILEMAP_PX_WIDTH as f64, TILEMAP_PX_HEIGHT as f64);
	// 	WindowBuilder::new()
	// 		.with_title("Tilemap")
	// 		.with_inner_size(size)
	// 		.with_min_inner_size(size)
	// 		.build(&event_loop)
	// 		.unwrap()
	// };
	// let mut tilemap_pixels = {
	// 	let window_size = tilemap_window.inner_size();
	// 	let surface_texture = SurfaceTexture::new(window_size.width * 4, window_size.height * 4, &tilemap_window);
	// 	Pixels::new(TILEMAP_PX_WIDTH as u32, TILEMAP_PX_HEIGHT as u32, surface_texture)?
	// };
	// let tileset_window = {
	// 	let size = LogicalSize::new(TILESET_VIEWER_PX_WIDTH as f64, TILESET_VIEWER_PX_HEIGHT as f64);
	// 	WindowBuilder::new()
	// 		.with_title("Tileset")
	// 		.with_inner_size(size)
	// 		.with_min_inner_size(size)
	// 		.build(&event_loop)
	// 		.unwrap()
	// };
	// let mut tileset_pixels = {
	// 	let window_size = tileset_window.inner_size();
	// 	let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &tileset_window);
	// 	Pixels::new(TILESET_VIEWER_PX_WIDTH as u32, TILESET_VIEWER_PX_HEIGHT as u32, surface_texture)?
	// };
	let args: Vec<String> = args().collect();
	let mut memory_bus = MemoryBus::new(Some(&args[1]));
	let mut cpu = Cpu::new();
	let mut ppu = Ppu::new();
	let mut timer = Timer::new();

	let mut nb_ticks = 0;
	let mut debug_enabled = false;

	memory_bus.load_dmg_bootrom();
	cpu.tick(&mut memory_bus);									// "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	event_loop.run(move |event, _, control_flow| {
		if let Event::RedrawRequested(_) = event {
            if let Err(err) = main_pixels.render() {
                println!("pixels.render: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
			// if let Err(err) = tilemap_pixels.render() {
            //     println!("pixels.render: {}", err);
            //     *control_flow = ControlFlow::Exit;
            //     return;
            // }
			// if let Err(err) = tileset_pixels.render() {
            //     println!("pixels.render: {}", err);
            //     *control_flow = ControlFlow::Exit;
            //     return;
            // }
        }
		if input.update(&event) {
			if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() || input.destroyed() { // || nb_ticks > 23579000 // || cpu.registers.program_counter - 1 == 0xFFFF
                *control_flow = ControlFlow::Exit;
                return;
            }
			if let Some(size) = input.window_resized() {
                if let Err(err) = main_pixels.resize_surface(size.width, size.height) {
                    println!("pixels.resize_surface: {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
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
				input::tick(&mut memory_bus, &input);
				for _ in 0..nb_cycles {
					frame_completed |= ppu.tick(&mut memory_bus, (main_pixels.frame_mut(), None, None));//Some(tilemap_pixels.frame_mut()), None));//Some(tileset_pixels.frame_mut())));
					timer.tick(&mut memory_bus);
				}
				nb_ticks += nb_cycles as u64;
			}
			main_window.request_redraw();
		}

	});
}
