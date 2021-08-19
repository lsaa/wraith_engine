//
// sdl2_helper.rs - SDL2 Helper
//

pub mod texture_manager;
pub mod atlas;
pub mod texture_store;

use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use std::path::Path;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::image::SaveSurface;

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
			unsafe { sdl2::sys::SDL_RenderSetIntegerScale(canvas.raw(), sdl2::sys::SDL_bool::SDL_FALSE);}
		},
		StretchMode::PIXELPERFECT => {
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

/*
    SDL_Texture* target = SDL_GetRenderTarget(renderer);
    SDL_SetRenderTarget(renderer, texture);
    int width, height;
    SDL_QueryTexture(texture, NULL, NULL, &width, &height);
    SDL_Surface* surface = SDL_CreateRGBSurface(0, width, height, 32, 0, 0, 0, 0);
    SDL_RenderReadPixels(renderer, NULL, surface->format->format, surface->pixels, surface->pitch);
    IMG_SavePNG(surface, file_name);
    SDL_FreeSurface(surface);
    SDL_SetRenderTarget(renderer, target);
*/

pub fn save_texture_to_file(canvas: &mut Canvas<Window>, path: &Path, texture: &mut Texture) -> Result<(), String> {
	let query = texture.query();
	let (w, h, format) = (query.width, query.height, query.format);
	let mut target = canvas.texture_creator().create_texture_target(format, w, h).unwrap();
	canvas.with_texture_canvas(&mut target, |tex| {
		tex.copy(texture, None, None).unwrap();
		let mut pixels_vec = tex.read_pixels(None, format).unwrap();
		let pitch = (format.byte_size_per_pixel() * w as usize) as u32;
		let surface = Surface::from_data(&mut pixels_vec, w, h, pitch, format).unwrap();
		let _= surface.save(path);
	}).unwrap();
	Ok(())
}