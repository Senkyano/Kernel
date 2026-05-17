// kernel.rs
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod terminal;
mod vgacolor;
// mod interrupts;

#[allow(unused_imports)]
#[allow(unused)]
use crate::terminal::{ScreenTerminal, Console};
use	crate::vgacolor::VgaColor;
use core::fmt::Write;
use core::panic::PanicInfo;

// Cette fonction est appelée si ton code Rust "panique" (erreur fatale)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

static mut TERM: ScreenTerminal = ScreenTerminal::new(0xB8000, 0x0F);

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	unsafe { // Le bloc unsafe est pour spécifier que l'on sait ce que l'on fait aux compilateur
		terminal::CURRENT_CONSOLE = Some(&mut TERM);
		TERM.clear_screen();
		// interrupts::init_idt();
		TERM.set_color(VgaColor::LightGREEN, VgaColor::Black);
		write!(TERM, "                          42 Kernel | Rihoy & Ythouihar\n");
		TERM.set_color(VgaColor::White, VgaColor::Black);
		// core::arch::asm!("sti"); // Instruction de l'activation d'interruption matérielle
		print!("Ca change tout");
	}
	loop {}
}
