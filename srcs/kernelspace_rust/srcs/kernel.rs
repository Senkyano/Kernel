// kernel.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Cette fonction est appelée si ton code Rust "panique" (erreur fatale)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(u8)] // spécifier que chaque data feras 8 bits donc 1 octet
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
	cursor_position: usize,
	color_code: u8,
	buffer: &'static mut Buffer,
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	// struct TerminalInfo {
	// 	let	width: u32 = 80, // Si on veut pouvoir changer et selectionner la taille il faudras mettre en mut
	// 	let height: u32 = 25,
	// 	let	mut color: u8 = 0,
	// }
    loop {}
}