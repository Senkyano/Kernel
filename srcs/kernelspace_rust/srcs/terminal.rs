
use crate::vgacolor::VgaColor;
use core::fmt;

const	VGA_HEIGHT: usize = 25;
const	VGA_WIDTH: usize = 80;

pub static mut CURRENT_CONSOLE: Option<&'static mut dyn Console> = None;

#[repr(transparent)]
struct Buffer {
	chars: [[u16; VGA_WIDTH]; VGA_HEIGHT],
}

// Information de ScreenTerminal
pub struct ScreenTerminal {
	pos_height : usize,
	pos_width : usize,
	color_code : u8,
	buffer: *mut Buffer,
}

pub fn make_color(fg: VgaColor, bg: VgaColor) -> u8 {
	(bg as u8) << 4 | (fg as u8)
}

fn make_entry(c: u8, color: u8) -> u16 {
	(color as u16) << 8 | (c as u16)
}

pub trait Console: fmt::Write {
	fn clear(&mut self);
}

// Implémentation de la Class ScreenTerminal
impl ScreenTerminal {
	pub const fn new(buffer_addr: usize, choose_color: u8) -> Self {
		ScreenTerminal {
			pos_height : 0,
			pos_width : 0,
			color_code : choose_color,
			buffer: buffer_addr as *mut Buffer,
		}
	}

	pub fn clear_screen(&mut self) {
		for height in 0..VGA_HEIGHT {
			for width in 0..VGA_WIDTH {
				self.put_entry_at(b' ', self.color_code, width, height);
			}
		}
	}

	fn put_entry_at(&mut self, c: u8, color: u8, width: usize, height: usize) {
		unsafe {
			let ptr = &mut (*self.buffer).chars[height][width] as *mut u16;
			core::ptr::write_volatile(ptr, make_entry(c, color));
		}
	}

	fn write_char(&mut self, c: u8) {
		match c {
			b'\n' => {
				self.pos_width = 0;
				self.pos_height += 1;
				if self.pos_height >= VGA_HEIGHT {
					self.scroll();
				}
			}
			_ => {
				self.put_entry_at(c, self.color_code, self.pos_width, self.pos_height);
				self.pos_width += 1;
				if self.pos_width >= VGA_WIDTH {
					self.pos_width = 0;
					self.pos_height += 1;
					if self.pos_height >= VGA_HEIGHT {
						self.scroll();
					}
				}
			}
		}
	}

	fn scroll(&mut self) {
		unsafe {
			for height in 2..VGA_HEIGHT {
				for width in 0..VGA_WIDTH {
					(*self.buffer).chars[height - 1][width] = (*self.buffer).chars[height][width];
				}
			}
			for width in 0..VGA_WIDTH {
				(*self.buffer).chars[VGA_HEIGHT - 1][width] = make_entry(b' ', self.color_code);
			}
		}
		self.pos_height = VGA_HEIGHT - 1;
	}

	pub fn set_color(&mut self, frontg: VgaColor, backg: VgaColor) {
		self.color_code = make_color(frontg, backg);
	}
}

// Implémentation de write! pour l'utiliser comme macro
impl fmt::Write for ScreenTerminal {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for byte in s.bytes() {
			self.write_char(byte);
		}
		Ok(())
	}
}

// Implémentation du Trait Console pour ScreenTerminal pour la flexibiliter 
// de tout type d'écran
impl Console for ScreenTerminal {
	fn clear(&mut self) {
		self.clear_screen();
	}
}

// Pour utilisation global et pour utiliser la fonction du terminal actuelle
pub fn write_format_global(args: fmt::Arguments) {
	unsafe {
		if let Some(ref mut console) = CURRENT_CONSOLE {
			let _ = console.write_fmt(args);
		}
	}
}

// Création de la Macro print pour "printf"
#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {{
		$crate::terminal::write_format_global(format_args!($($arg)*));
	}};
}
