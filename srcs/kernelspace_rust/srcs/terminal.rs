
use crate::vgacolor::VgaColor;
use core::fmt;
use crate::interrupts::outb;

pub const	VGA_HEIGHT:	usize = 25;
pub const	VGA_WIDTH:	usize = 80;

pub static mut CURRENT_CONSOLE: Option<&'static mut dyn Console> = None;

#[repr(transparent)]
pub struct Buffer {
	pub chars: [[u16; VGA_WIDTH]; VGA_HEIGHT],
}

// Information de ScreenTerminal
pub struct ScreenTerminal {
	pos_height :	usize,
	pos_width :		usize,
	color_code :	u8,
	pub buffer:		*mut Buffer,
}

pub fn make_color(fg: VgaColor, bg: VgaColor) -> u8 {
	(bg as u8) << 4 | (fg as u8)
}

fn make_entry(c: u8, color: u8) -> u16 {
	(color as u16) << 8 | (c as u16)
}

pub trait Console: fmt::Write {
	fn clear(&mut self);
	fn move_cursor_down(&mut self);
	fn move_cursor_left(&mut self);
	fn move_cursor_right(&mut self);
	fn move_cursor_up(&mut self);
	fn choose_screen_console(&mut self, new_addr_buffer: *mut Buffer);
	fn set_other_color(&mut self, frontbg: VgaColor, backbg: VgaColor);

	#[allow(unused)]
	fn put_char_spe(&mut self, c: u8, at_width: usize, at_height: usize);
	fn change_char(&mut self, c: u8);
	fn write_debug(&mut self, x: usize, y: usize, args: fmt::Arguments);
}

struct	DebugClipper<'a> {
	console: &'a mut dyn Console,
	chars_left: usize,
}

impl<'a> DebugClipper<'a> {
	pub fn new(console: &'a mut dyn Console, max_chars: usize) -> Self {
		DebugClipper {
			console,
			chars_left: max_chars,
		}
	}
}

pub const VGA_HARDWARE_ADDR: usize = 0xB8000;

// Implémentation de la Class ScreenTerminal
impl ScreenTerminal {
	pub const fn new(buffer_addr: usize, choose_color: u8) -> Self {
		ScreenTerminal {
			pos_height		: 0,
			pos_width		: 0,
			color_code		: choose_color,
			buffer			: buffer_addr as *mut Buffer,
		}
	}

	pub unsafe fn flush_to_vga(&self) {
		let vga_hardware_ptr = VGA_HARDWARE_ADDR as  *mut Buffer;
		core::ptr::copy_nonoverlapping(self.buffer, vga_hardware_ptr, 1);
	}

	fn	screen_change(&mut self, new_buffer_addr: *mut Buffer) {
		self.buffer = new_buffer_addr;
	}

	fn	write_debug(&mut self,
		pos_width: usize,
		pos_height: usize,
		args: fmt::Arguments) {
		let saved_width = self.pos_width;
		let saved_heigth = self.pos_height;

		self.pos_height = pos_height;
		self.pos_width = pos_width;

		self.clear_screen_at_to(42, 0, 79, 0);
		let mut clipper = DebugClipper::new(self, 37);

		use core::fmt::Write;
		let _ = clipper.write_fmt(args);

		self.pos_width = saved_width;
		self.pos_height = saved_heigth;
		self.update_cursor();
		// self.reload_screen();
	}

	pub fn clear_screen(&mut self) {
		for height in 0..VGA_HEIGHT {
			for width in 0..VGA_WIDTH {
				self.put_entry_at(b'\0', self.color_code, width, height);
			}
		}
	}

	fn clear_screen_at_to(&mut self, mut x_start: usize, y_start: usize, x_end: usize, y_end: usize) {
		let mut vga_width = VGA_WIDTH;

		for height in y_start..=y_end {
			for width in x_start..vga_width {
				self.put_entry_at(b'\0', self.color_code, width, height);
			}
			if height == y_end {
				vga_width = x_end;
			}
			x_start = 0;
		}
	}

	fn get_buffer_chars(&mut self, width: usize, height: usize) -> u8 {
		if width >= VGA_WIDTH || height >= VGA_HEIGHT {
			return b' ';
		}

		unsafe {
			let entry = (*self.buffer).chars[height][width];

			(entry & 0xFF) as u8
		}
	}

	fn get_buffer_color(&mut self, width: usize, height: usize) -> u8 {
		if width >= VGA_WIDTH || height >= VGA_HEIGHT {
			return self.color_code;
		}

		unsafe {
			let entry = (*self.buffer).chars[height][width];

			((entry >> 8) & 0xFF) as u8
		}
	}

	// fn reload_screen(&mut self) {
	// 	for height in 0..VGA_HEIGHT {
	// 		for width in 0..VGA_WIDTH {
	// 			let mut c = self.get_buffer_chars(width, height);
	// 			let mut color_code = self.get_buffer_color(width, height);

	// 			self.put_entry_at(c, color_code, width, height);
	// 		}
	// 	}
	// }

	fn put_entry_at(&mut self, c: u8, color: u8, width: usize, height: usize) {
		unsafe {
			let ptr = &mut (*self.buffer).chars[height][width] as *mut u16;
			core::ptr::write_volatile(ptr, make_entry(c, color));
		}
	}

	fn write_char(&mut self, c: u8) {
		match c {
			b'\n' => { // si on rencontre un retour a la ligne
				self.pos_width = 0;
				self.pos_height += 1;
				if self.pos_height >= VGA_HEIGHT {
					self.scroll();
				}
			}
			_ => { // Default
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
		self.update_cursor();
		unsafe {self.flush_to_vga()};
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

	fn	mov_cursor_left(&mut self) {
		if self.pos_width > 0 {
			self.pos_width = self.pos_width - 1;
		} else {
			if self.pos_height > 1 {
				self.pos_width = VGA_WIDTH - 1;
			}
			if self.pos_height > 1 {
				self.pos_height = self.pos_height - 1;
			}
		}
	}

	fn	mov_cursor_right(&mut self) {
		if self.pos_width < VGA_WIDTH - 1 {
			self.pos_width = self.pos_width + 1;
		} else {
			if self.pos_height < VGA_HEIGHT - 2 {
				self.pos_width = 0;
			}
			if self.pos_height < VGA_HEIGHT - 1{
				self.pos_height = self.pos_height + 1;
			}
		}
	}

	fn	mov_cursor_up(&mut self) {
		if self.pos_height > 1 {
			self.pos_height = self.pos_height - 1;
		}
	}

	fn	mov_cursor_down(&mut self) {
		if self.pos_height < VGA_HEIGHT - 1 {
			self.pos_height = self.pos_height + 1;
		}
	}

	pub fn update_cursor(&self) {
		let position = (self.pos_height * VGA_WIDTH) + self.pos_width;

		unsafe {
			outb(0x3D4, 0x0F);
			outb(0x3D5, (position & 0xFF) as u8);
			outb(0x3D4, 0x0E);
			outb(0x3D5, ((position >> 8) & 0xFF) as u8);
		}
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

impl<'a> core::fmt::Write for DebugClipper<'a> {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for byte in s.bytes() {
			if self.chars_left == 0 {
				break; 
			}
			
			if byte == b'\n' {
				continue;
			}

			let _ = self.console.write_char(byte as char);
			
			self.chars_left -= 1;
		}
		Ok(())
	}
}

// Implémentation du Trait Console pour ScreenTerminal pour la flexibiliter 
// de tout type d'écran
impl Console for ScreenTerminal {
	fn clear(&mut self) { self.clear_screen(); }

	fn choose_screen_console(&mut self, new_addr_buffer: *mut Buffer) {
		self.screen_change(new_addr_buffer);
		unsafe {self.flush_to_vga()};
	}

	fn move_cursor_left(&mut self) {
		self.mov_cursor_left();
		self.update_cursor();
	}

	fn move_cursor_up(&mut self) {
		self.mov_cursor_up();
		self.update_cursor();
	}

	fn move_cursor_right(&mut self) {
		self.mov_cursor_right();
		self.update_cursor();
	}

	fn move_cursor_down(&mut self) {
		self.mov_cursor_down();
		self.update_cursor();
	}

	fn put_char_spe(&mut self, c: u8, at_width: usize, at_height: usize) {
		self.put_entry_at(c, self.color_code, at_width, at_height);
	}

	fn change_char(&mut self, c: u8) {
		self.put_entry_at(c, self.color_code, self.pos_width, self.pos_height);
	}

	fn write_debug(&mut self, x: usize, y: usize, args: fmt::Arguments) {
		self.write_debug(x, y, args);
	}
	// fn changeKeyboard(&mut self, )

	fn set_other_color(&mut self, frontbg: VgaColor, backbg: VgaColor) {
		self.set_color(frontbg, backbg);
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

pub fn write_debug_global(x: usize, y: usize, args: fmt::Arguments) {
	unsafe {
		if let Some(ref mut console) = &mut CURRENT_CONSOLE {
			console.write_debug(x, y, args);
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

#[macro_export]
macro_rules! debug_at {
	($x:expr, $y:expr, $($arg:tt)*) => {{
		$crate::terminal::write_debug_global($x, $y, format_args!($($arg)*));
	}};
}
