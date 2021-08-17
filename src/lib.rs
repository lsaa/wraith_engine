extern crate sdl2;

#[cfg(target_arch = "wasm32")]
use emscripten_main_loop::{MainLoop, MainLoopEvent};

use std::time::Duration;
use std::time::Instant;

pub mod util;

pub struct WraithGame<State> {
	pub state: State,
	pub run_loop: fn(&mut State, Duration) -> LoopState,
	pub last_time: Instant,
}

#[derive(PartialEq)]
pub enum LoopState {
	CONTINTUE,
	QUIT
}

#[cfg(target_arch = "wasm32")]
impl<State> MainLoop for WraithGame<State> {
	fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent { 
		let dtime = Instant::now() - self.last_time;
		self.last_time = Instant::now();
		if (self.run_loop)(&mut self.state, dtime) == LoopState::QUIT {
			return MainLoopEvent::Terminate;
		}
		return MainLoopEvent::Continue;
	}
}

#[cfg(target_arch = "wasm32")]
pub fn run<State: 'static>(mut game: WraithGame<State>) {
	emscripten_main_loop::run(game);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run<State>(mut game: WraithGame<State>) {
	loop {
		let dtime = Instant::now() - game.last_time;
		game.last_time = Instant::now();
		if (game.run_loop)(&mut game.state, dtime) == LoopState::QUIT {
			break
		}
	}
}

impl<State> WraithGame<State> {
	pub fn new(state: State, run_loop: fn(&mut State, Duration) -> LoopState) -> Self {
		Self {
			state,
			run_loop,
			last_time: Instant::now()
		}
	}
}
