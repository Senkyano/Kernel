// kernel.rs
#![no_std]
#![no_main]

mod terminal;
mod vgacolor;
mod interrupts;

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
	macro_rules! term {
    	() => { &mut *(&raw mut TERM) }
	}

	unsafe { // Le bloc unsafe est pour spécifier que l'on sait ce que l'on fait aux compilateur
		term!().clear();
		term!().set_color(VgaColor::LightGREEN, VgaColor::Black);
		terminal::CURRENT_CONSOLE = Some(term!());
		let _ = write!(term!(), "                          42 Kernel | Rihoy & Ythouihar\n");
		term!().set_color(VgaColor::White, VgaColor::Black);
		print!("Ca change touta\n");
		interrupts::init_idt();
	}
	loop {}
}
