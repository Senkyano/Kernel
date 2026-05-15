// kernel.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Cette fonction est appelée si ton code Rust "panique" (erreur fatale)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {
		unsafe {
			core::arch::asm!("hlt");
		}
	}
}

#[repr(u8)] // spécifier que chaque data feras 8 bits donc 1 octet
#[allow(dead_code)]
enum	VGA_Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Red = 3,
	Magenta = 5,
	Brown = 6,
	LightGREY = 7,
	DarkGREY = 8,
	LightBLUE = 9,	
	LightGREEN = 10,
	LightCYAN = 11,
	LightRED = 12,
	LightMAGENTA = 13,
	LightBROWN = 14,
	White = 15,
}

const	VGA_HEIGHT: usize = 25;
const	VGA_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
	chars: [[u16; VGA_WIDTH]; VGA_HEIGHT],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ScreenChar {
	asii_char: u8,
	color_code: u8,
}

pub struct Terminal {
	pos_height : usize,
	pos_width : usize,
	color_code : u8,
	buffer: &'static mut Buffer,
}

pub fn make_color(fg: VGA_Color, bg: VGA_Color) -> u8 {
	(bg as u8) << 4 | (fg as u8)
}

fn make_entry(c: u8, color: u8) -> u16 {
	(color as u16) << 8 | (c as u16)
}

impl Terminal {
	pub fn new() -> Self {
		let mut t = Terminal {
			pos_height : 0,
			pos_width : 0,
			color_code : make_color(VGA_Color::White, VGA_Color::Black),
			buffer: unsafe { &mut *(0xB8000 as *mut Buffer)},
		};
		t.clear_screen();
		t
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
		let ptr = &mut self.buffer.chars[height][width] as *mut u16;
		core::ptr::write_volatile(ptr, make_entry(c, color));
	}
}

	fn scroll(&mut self) {
		for height in 1..VGA_HEIGHT {
			for width in 0..VGA_WIDTH {
				self.buffer.chars[height - 1][width] = self.buffer.chars[height][width];
			}
		}
		for width in 0..VGA_WIDTH {
			self.buffer.chars[VGA_HEIGHT - 1][width] = make_entry(b' ', self.color_code);
		}
		self.pos_height = VGA_HEIGHT - 1;
	}

	pub fn write_char(&mut self, c: u8) {
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

	pub fn write_str(&mut self, s: &str) {
		for byte in s.bytes() {
			self.write_char(byte);
		}
	}

	pub fn set_color(&mut self, frontg: VGA_Color, backg: VGA_Color) {
		self.color_code = make_color(frontg, backg);
	}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	// 1. On initialise le terminal (ce qui va vider l'écran automatiquement)
	let mut term = Terminal::new();

	// 2. On écrit notre premier message propre
	term.write_str("Hello World depuis Rust!\n");
	term.write_str("Le Kernel est officiellement fonctionnel.sduyggyfs");

	// 3. On bloque le CPU proprement
	loop {
		unsafe {
			core::arch::asm!("hlt");
		}
	}
}