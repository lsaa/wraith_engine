//
// tilemap.rs - Simple Tilemap System
//

pub struct TileMap {
	width: u32,
	height: u32,
	pub cells: Vec<u32>, // Flat Map of tile IDs to be CPU cache friendly
}

#[allow(dead_code)]
pub struct TileSet {
	tile_size_x: u32,
	tile_size_y: u32,
	length: u32,	
}

#[allow(dead_code)]
impl TileSet {
	fn new() -> Self {
		Self {
			tile_size_x: 0,
			tile_size_y: 0,
			length: 0,
		}
	}
}

impl TileMap {
	
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			cells: vec![0; width as usize* height as usize]
		}
	}

	pub fn get_cell(&self, x: u32, y: u32) -> u32 {
		self.cells[self.width as usize * y as usize + x as usize]
	}

	pub fn set_cell(&mut self, x: u32, y: u32, value: u32) {
		self.cells[self.width as usize * y as usize + x as usize] = value;
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}

	pub fn get_height(&self) -> u32 {
		self.height
	}

}