mod memory_bus;
mod cpu;
mod ppu;
mod input;
mod timer;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::Ppu;
use timer::Timer;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
	let mut memory_bus = MemoryBus::new(Some("/home/rgilles/Desktop/roms/Pokemon - Jaune.gbc"));
	memory_bus.load_dmg_bootrom();
	let mut cpu = Cpu::new();
	let mut ppu = Ppu::new(false, false);
	let mut timer = Timer::new();
	cpu.tick(&mut memory_bus);									// "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	let mut nb_ticks = 0;

	let event_loop = EventLoop::new();
	let mut input = WinitInputHelper::new();
	let window = {
		let size = LogicalSize::new(160 as f64, 144 as f64);
		WindowBuilder::new()
			.with_title("GBMU")
			.with_inner_size(size)
			.with_min_inner_size(size)
			.build(&event_loop)
			.unwrap()
	};
	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(160, 144, surface_texture)?
	};	

	let mut debug_enabled = false;
	event_loop.run(move |event, _, control_flow| {
		if let Event::RedrawRequested(_) = event {
            if let Err(err) = pixels.render() {
                println!("pixels.render: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
		if input.update(&event) {
			if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() || input.destroyed() { // || nb_ticks > 23579000 // || cpu.registers.program_counter - 1 == 0xFFFF
                *control_flow = ControlFlow::Exit;
                return;
            }
			if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    println!("pixels.resize_surface: {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
			if nb_ticks >= u64::MAX {	
				debug_enabled = true;
			}
			let mut frame_completed = false;
			while !frame_completed {
				if debug_enabled {
					println!("{:x?}", cpu);
					println!("Tick count: {}", nb_ticks);
					std::thread::sleep(std::time::Duration::from_millis(50))
				}
				let nb_cycles = cpu.tick(&mut memory_bus);
				input::tick(&mut memory_bus, &input);
				for _ in 0..nb_cycles {
					frame_completed |= ppu.tick(&mut memory_bus, pixels.frame_mut());
					timer.tick(&mut memory_bus);
				}
				nb_ticks += nb_cycles as u64;
			}
			window.request_redraw();
		}

	});
}
