//
// sdl2_helper.rs - SDL2 Helper
//

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::EventPump;

// Window

#[derive(PartialEq, Debug)]
pub enum StretchMode {
	STRETCH,
	LETTERBOX,
	PIXELPERFECT
}

pub fn set_stretch_mode(canvas: &mut Canvas<Window>, mode: &StretchMode) {
	match mode {
		StretchMode::STRETCH => {
			unsafe { sdl2::sys::SDL_RenderSetIntegerScale(canvas.raw(), sdl2::sys::SDL_bool::SDL_FALSE);}
			let (window_w, window_h) = canvas.output_size().unwrap();
			let (logical_w, logical_h) = canvas.logical_size();
			let s = (window_w as f32 / logical_w as f32, window_h as f32 / logical_h as f32);
			let _ = canvas.set_logical_size(window_w, window_h);
			let _ = canvas.set_scale(s.0, s.1);
		},
		StretchMode::LETTERBOX => {
			let _ = canvas.set_logical_size(64, 64);
			unsafe { sdl2::sys::SDL_RenderSetIntegerScale(canvas.raw(), sdl2::sys::SDL_bool::SDL_FALSE);}
		},
		StretchMode::PIXELPERFECT => {
			let _ = canvas.set_logical_size(64, 64);
			unsafe { sdl2::sys::SDL_RenderSetIntegerScale(canvas.raw(), sdl2::sys::SDL_bool::SDL_TRUE);}
		},
	}
}

pub fn logical_mouse_position(events: &EventPump, canvas: &Canvas<Window>) -> Option<(u32, u32)> {
	let m = events.mouse_state();
	let (x, y) = (m.x(), m.y());
	let (window_w, window_h) = canvas.output_size().unwrap();
	let (logical_w, logical_h) = canvas.logical_size();
	let (scale_x, scale_y) = canvas.scale();
	let (real_w, real_h) = (logical_w as f32 * scale_x, logical_h as f32 * scale_y);
	let (r_mid_x, r_mid_y) = (real_w / 2.0, real_h / 2.0);
	let (w_mid_x, w_mid_y) = (window_w / 2, window_h / 2);
	let (rx, ry) = (w_mid_x - r_mid_x as u32, w_mid_y - r_mid_y as u32);
	let (new_x, new_y) = (x - rx as i32, y - ry as i32);

	if new_x < 0 || new_y < 0 
	|| (new_x as f32 / real_w * logical_w as f32) as u32 >= logical_w 
	|| (new_y as f32 / real_h * logical_h as f32) as u32 >= logical_h {
		return None;
	}

	Some((
		(new_x as f32 / real_w * logical_w as f32) as u32,
		(new_y as f32 / real_h * logical_h as f32) as u32,
	))
}