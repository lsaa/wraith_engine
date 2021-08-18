//
// altas.rs - TextureAtlas builder
//

use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use rectangle_pack::pack_rects;
use rectangle_pack::volume_heuristic;
use rectangle_pack::contains_smallest_box;
use rectangle_pack::TargetBin;
use sdl2::pixels::PixelFormatEnum;
use std::collections::BTreeMap;
use crate::texture_manager::TextureManager;
use rectangle_pack::RectToInsert;
use rectangle_pack::GroupedRectsToPlace;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::Texture;

pub struct TextureAtlas {
	pub img: Texture,
	pub rects: HashMap<String, Rect>
}

impl TextureAtlas {
	pub fn get(&self, path: &str) -> Option<&Rect> {
		self.rects.get(&path.to_string())
	}
}

pub struct TextureAtlasBuilder {
	pub rects: GroupedRectsToPlace<String>,
	pub texture_manager: TextureManager,
}

impl TextureAtlasBuilder {
	pub fn from_files(files:  Vec<&str>, canvas: &mut Canvas<Window>) -> Result<Self, String> {
		let mut rects = GroupedRectsToPlace::new();
		let mut texture_manager = TextureManager::new();
		for tex in files {
			let res = texture_manager.create(tex, canvas);
			if res.is_err() { return Err(format!("Couldn't load texture {} from the list", tex)); }
			let t = texture_manager.load(tex).unwrap();
			let (w, h) = (t.query().width, t.query().height);
			rects.push_rect(tex.to_owned(), None, RectToInsert::new(w, h, 1));
		}

		Ok(Self {
			rects,
			texture_manager
		})
	}

	pub fn build(&mut self, canvas: &mut Canvas<Window>) -> Result<TextureAtlas, String> {
		let start_w = 32;
		let start_h = 32;
		let max_h = 4096;
		let max_w = 4096;
		let mut cw = start_w;
		let mut ch = start_h;

		let mut built = None;

		while built.is_none() {
			let stop = max_h == ch && cw == max_w;

			let mut bins = BTreeMap::new();
			bins.insert(0, TargetBin::new(cw, ch, 1));
			built = match pack_rects(
				&self.rects,
                &mut bins,
                &volume_heuristic,
                &contains_smallest_box,
			) {
				Ok(built) => {
					Some(built)
				},
				Err(rectangle_pack::RectanglePackError::NotEnoughBinSpace) => {
					cw = (cw * 2).clamp(0, max_w);
					ch = (ch * 2).clamp(0, max_h);
					None
				}  
			};

			if stop {
				break;
			}
		}

		let built = built.ok_or(String::from("Couldn't fit texture in atlas bin")).unwrap();

		let texture_creator = canvas.texture_creator();
		let surface = Surface::new(cw, ch, PixelFormatEnum::RGBA8888).unwrap();
		let mut texture = texture_creator.create_texture_from_surface(surface).unwrap();

		let rects = HashMap::new();
		for (path, (_, location)) in built.packed_locations().iter() {
			let ct = self.texture_manager.load(path).unwrap();
			let (w, h) = (ct.query().width, ct.query().height);
			let (x1, y1) = (location.x() as i32, location.y() as i32);
			let (x2, y2) = (x1 as u32 + location.width(), y1 as u32 + location.height());
			canvas.with_texture_canvas(&mut texture, |texture_canvas| {
                let _res = texture_canvas.copy(ct, Rect::new(0, 0, w, h), Rect::new(x1, y1, x2, y2));
            })
            .map_err(|e| e.to_string())?;
			self.texture_manager.drop(path.to_string());
		}

		Ok(TextureAtlas {
			img: texture,
			rects
		})

	}
}