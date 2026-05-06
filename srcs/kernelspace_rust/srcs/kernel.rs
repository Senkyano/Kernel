// kernel.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Cette fonction est appelée si ton code Rust "panique" (erreur fatale)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// #[repr(u8)] // spécifier que chaque data feras 8 bits donc 1 octet
// enum	VGA_Color {
// 	Black = 0,
// 	Blue = 1,
// 	Green = 2,
// 	Red = 3,
// 	Magenta = 5,
// 	Brown = 6,
// 	LightGREY = 7,
// 	DarkGREY = 8,
// 	LightBLUE = 9,	
// 	LightGREEN = 10,
// 	LightCYAN = 11,
// 	LightRED = 12,
// 	LightMAGENTA = 13,
// 	LightBROWN = 14,
// 	White = 15,
// }

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	// struct TerminalInfo {
	// 	let	width: u32 = 80, // Si on veut pouvoir changer et selectionner la taille il faudras mettre en mut
	// 	let height: u32 = 25,
	// 	let	mut color: u8 = 0,
	// }
    loop {}
}