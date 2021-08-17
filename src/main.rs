extern crate sdl2;

use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;
use sdl2::image::Sdl2ImageContext;
use sdl2::timer::Timer;
use crate::util::color::HsvColor;
use sdl2::EventPump;
use crate::util::tilemap::TileMap;
use crate::util::sdl2_helper::StretchMode;
use sdl2::VideoSubsystem;
use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::path::Path;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::*;
use sdl2::image::{InitFlag, LoadTexture};
use std::time::Duration;

use legion::*;

pub mod util;

struct Game {
	pub canvas: Canvas<Window>,
	pub context: Sdl,
	pub video: VideoSubsystem,
	pub stretch_mode: StretchMode,
	pub world: World,
	pub tiles: TileMap,
	pub events: EventPump,
	pub funny_color: HsvColor,
	pub img_context: Sdl2ImageContext,
	pub texture_creator: TextureCreator<WindowContext>
}

impl emscripten_main_loop::MainLoop for Game {
	fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
		let r = sdl2::rect::Rect::new(0, 0, 64, 64);
		let tile_rect = sdl2::rect::Rect::new(0, 0, 4, 4);
		let timer = self.context.timer().unwrap();
		self.funny_color.h = ((timer.ticks() as f32 % (360_f32 * 48_f32)) / (360_f32 * 48_f32) * 255.0) as u8;
		let c = self.funny_color.to_rgb();
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
		self.canvas.set_draw_color(Color::RGB(c.r, c.g, c.b));
		let _ = self.canvas.fill_rect(r);
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		let texture = self.texture_creator.load_texture(&Path::new("assets/hmm_yes_funny_tile.png")).unwrap();
	
		for y in 0..self.tiles.get_height() {
			for x in 0..self.tiles.get_width() {
				let t_id = self.tiles.get_cell(x,y);
				if t_id == 1 {
					let _ = self.canvas.copy(&texture, tile_rect, sdl2::rect::Rect::new(x as i32 * 4, y as i32 * 4, 4, 4));
					//let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(x as i32 * 4, y as i32 * 4, 4, 4));
				}
			}
		}


        for event in self.events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return emscripten_main_loop::MainLoopEvent::Terminate;
                },
				Event::KeyDown { keycode: Some(Keycode::P), .. } => {
					if self.stretch_mode == util::sdl2_helper::StretchMode::PIXELPERFECT {
						self.stretch_mode = util::sdl2_helper::StretchMode::LETTERBOX;
					}
					else if self.stretch_mode == util::sdl2_helper::StretchMode::LETTERBOX {
						self.stretch_mode = util::sdl2_helper::StretchMode::STRETCH;
					}
					else if self.stretch_mode == util::sdl2_helper::StretchMode::STRETCH {
						self.stretch_mode = util::sdl2_helper::StretchMode::PIXELPERFECT;
					}
                    util::sdl2_helper::set_stretch_mode(&mut self.canvas, &self.stretch_mode);
                }
                _ => {}
            }
        }
		
		//let w = canvas.output_size().unwrap();

		if self.events.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
			let mouse_pos = util::sdl2_helper::logical_mouse_position(&self.events, &self.canvas);
			if let Some((mx, my)) = mouse_pos {
				self.tiles.set_cell(mx / 4, my / 4, 1);
			}
		}


		if self.events.mouse_state().is_mouse_button_pressed(MouseButton::Right) {
			let mouse_pos = util::sdl2_helper::logical_mouse_position(&self.events, &self.canvas);
			if let Some((mx, my)) = mouse_pos {
				self.tiles.set_cell(mx / 4, my / 4, 0);
			}		
		}

        // The rest of the game loop goes here...

        self.canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
		self.canvas.window().gl_swap_window();
		emscripten_main_loop::MainLoopEvent::Continue
	}
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
	let world = World::default();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

	video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::GLES);
	video_subsystem.gl_attr().set_context_major_version(3);
	video_subsystem.gl_attr().set_context_minor_version(0);

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
		.set_window_flags(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_UTILITY as u32)
		.opengl()
		.resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
	let _ = canvas.set_logical_size(64, 64);
	unsafe {
        sdl2::sys::SDL_RenderSetIntegerScale(canvas.raw(), sdl2::sys::SDL_bool::SDL_TRUE);
    }

	let stretch_mode = util::sdl2_helper::StretchMode::PIXELPERFECT;

	let pixel_scale = 10;
	let new_window_size = (500, 300);
	let _ = canvas.window_mut().set_size(new_window_size.0, new_window_size.1);

	let tiles = util::tilemap::TileMap::new(16, 16);
    let events = sdl_context.event_pump().unwrap();
	let funny_color: util::color::HsvColor = util::color::HsvColor::new(0, 255, 255);
	util::sdl2_helper::set_stretch_mode(&mut canvas, &stretch_mode);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

	let texture_creator = canvas.texture_creator();

	let mut game = Game {
		canvas,
		context: sdl_context,
		video: video_subsystem,
		stretch_mode,
		world,
		tiles,
		events,
		funny_color,
		img_context: _image_context,
		texture_creator,
	};
	emscripten_main_loop::run(game);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
	let world = World::default();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

	video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::GLES);
	video_subsystem.gl_attr().set_context_major_version(3);
	video_subsystem.gl_attr().set_context_minor_version(0);

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
		.set_window_flags(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_UTILITY as u32)
		.opengl()
		.resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
	let _ = canvas.set_logical_size(64, 64);
	unsafe {
        sdl2::sys::SDL_RenderSetIntegerScale(canvas.raw(), sdl2::sys::SDL_bool::SDL_TRUE);
    }

	let stretch_mode = util::sdl2_helper::StretchMode::PIXELPERFECT;

	let pixel_scale = 10;
	let new_window_size = (500, 300);
	let _ = canvas.window_mut().set_size(new_window_size.0, new_window_size.1);

	let tiles = util::tilemap::TileMap::new(16, 16);
    let events = sdl_context.event_pump().unwrap();
	let funny_color: util::color::HsvColor = util::color::HsvColor::new(0, 255, 255);
	util::sdl2_helper::set_stretch_mode(&mut canvas, &stretch_mode);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

	let texture_creator = canvas.texture_creator();

	let mut game = Game {
		canvas,
		context: sdl_context,
		video: video_subsystem,
		stretch_mode,
		world,
		tiles,
		events,
		funny_color,
		img_context: _image_context,
		texture_creator,
	};

	use emscripten_main_loop::MainLoop;
	'running:  loop {
		match game.main_loop() {
			emscripten_main_loop::MainLoopEvent::Continue => (),
			emscripten_main_loop::MainLoopEvent::Terminate => break 'running
		}
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}