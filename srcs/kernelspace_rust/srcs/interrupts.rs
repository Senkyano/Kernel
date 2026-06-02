#![allow(dead_code)]

use core::arch::asm;

use crate::print;
use crate::debug_at;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA:	u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA:	u16 = 0xA1;
const PIC_EOI:	  u8  = 0x20; // End Of Interrupt

pub const PIC_1_OFFSET: u8 = 0x20;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[repr(u8)]
pub enum Irq {
	Timer			= PIC_1_OFFSET + 0,
	Keyboard		= PIC_1_OFFSET + 1,
	Cascade			= PIC_1_OFFSET + 2,
	Com2			= PIC_1_OFFSET + 3,
	Com1			= PIC_1_OFFSET + 4,
	Lpt2			= PIC_1_OFFSET + 5,
	FloppyDisk	  	= PIC_1_OFFSET + 6,
	Lpt1			= PIC_1_OFFSET + 7,
	RealTimeClock   = PIC_1_OFFSET + 8,
	Peripheral1		= PIC_1_OFFSET + 9,
	Peripheral2		= PIC_1_OFFSET + 10,
	Peripheral3		= PIC_1_OFFSET + 11,
	Ps2Mouse		= PIC_1_OFFSET + 12,
	Fpu				= PIC_1_OFFSET + 13,
	PrimaryAta		= PIC_1_OFFSET + 14,
	SecondaryAta	= PIC_1_OFFSET + 15,
}

static SCANCODE_MAP: [u8; 58] = [
	0,	// 0x00 - rien
	0,	// 0x01 - Escape
	b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0',
	b'-', b'=',
	0,	// 0x0E - Backspace
	b'\t',// 0x0F - Tab
	b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p',
	b'[', b']',
	b'\n',// 0x1C - Enter
	0,	// 0x1D - Ctrl gauche
	b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l',
	b';', b'\'', b'`',
	0,	// 0x2A - Shift gauche
	b'\\',
	b'z', b'x', b'c', b'v', b'b', b'n', b'm',
	b',', b'.', b'/',
	0,	// 0x36 - Shift droit
	b'*', // 0x37 - pavé num *
	0,	// 0x38 - Alt
	b' ', // 0x39 - Espace
];

static SCANCODE_MAP_SHIFT: [u8; 58] = [
	0,
	0,
	b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')',
	b'_', b'+',
	0,
	b'\t',
	b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P',
	b'{', b'}',
	b'\n',
	0,
	b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L',
	b':', b'"', b'~',
	0,
	b'|',
	b'Z', b'X', b'C', b'V', b'B', b'N', b'M',
	b'<', b'>', b'?',
	0,
	b'*',
	0,
	b' ',
];

// ── Ports I/O ─────────────────────────────────────────────────────
pub unsafe fn outb(port: u16, value: u8) {
	asm!("outb %al, %dx",
		in("dx") port,
		in("al") value,
		options(att_syntax)
	);
}

pub unsafe fn inb(port: u16) -> u8 {
	let value: u8;
	asm!("inb %dx, %al",
		in("dx") port,
		out("al") value,
		options(att_syntax)
	);
	value
}

unsafe fn io_wait() {
	outb(0x80, 0); // port 0x80 = port de diagnostic, écrire dessus = délai
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct	IdtEntry {
	offset_low:		u16,
	selector:		u16,
	zero:			u8,
	type_attr:		u8,
	offset_high:	u16,
}

impl IdtEntry {
	const fn missing() -> Self {
		IdtEntry {
			offset_low:  0,
			selector:	 0,
			zero:		 0,
			type_attr:	 0,
			offset_high: 0,
		}
	}

	fn new(handler: u32) -> Self {
		let mut current_cs: u16;
		unsafe {
			asm!("mov {:x}, cs", out(reg) current_cs);
		}

		IdtEntry {
			offset_low:	 (handler & 0xFFFF) as u16,
			selector:	 current_cs,
			zero:		 0,
			type_attr:	 0x8E,
			offset_high: (handler >> 16) as u16,
		}
	}
}

#[repr(C, packed)]
struct IdtPointer {
	limit:	u16,
	base:	u32,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::missing(); 256];

unsafe fn pic_remap() {
	// Sauvegarder les masques actuels
	let mask1 = inb(PIC1_DATA);
	let mask2 = inb(PIC2_DATA);

	// ICW1 : début de l'initialisation
	outb(PIC1_COMMAND, 0x11); io_wait();
	outb(PIC2_COMMAND, 0x11); io_wait();

	// ICW2 : offsets des vecteurs
	outb(PIC1_DATA, PIC_1_OFFSET); io_wait(); // PIC1 -> IRQ 0x20-0x27
	outb(PIC2_DATA, PIC_2_OFFSET); io_wait(); // PIC2 -> IRQ 0x28-0x2F

	// ICW3 : cascade
	outb(PIC1_DATA, 0x04); io_wait(); // PIC1 a un esclave sur IRQ2
	outb(PIC2_DATA, 0x02); io_wait(); // PIC2 est l'esclave sur IRQ2

	// ICW4 : mode 8086
	outb(PIC1_DATA, 0x01); io_wait();
	outb(PIC2_DATA, 0x01); io_wait();

	// Restaurer les masques
	outb(PIC1_DATA, mask1);
	outb(PIC2_DATA, mask2);
}

// ── Envoyer EOI au PIC ────────────────────────────────────────────
pub unsafe fn pic_send_eoi(irq: u8) {
	if irq >= 8 {
		outb(PIC2_COMMAND, PIC_EOI); // EOI au PIC esclave aussi
	}
	outb(PIC1_COMMAND, PIC_EOI);
}

// ── Handlers d'interruptions ──────────────────────────────────────

unsafe fn do_exception() {
	// Si une exception CPU arrive on loop
	loop {}
}

// IRQ 0 : Timer
unsafe fn do_timer() {
	// Tick du timer système
	pic_send_eoi(0);
}

unsafe fn handle_keypress(scancode: u8) {
	let idx = scancode as usize;

	debug_at!(42, 0, "handlekey : {}", idx);
	if idx >= SCANCODE_MAP.len() {
		return;
	}

	let ch = if SHIFT_PRESSED {
		SCANCODE_MAP_SHIFT[idx]
	} else {
		SCANCODE_MAP[idx]
	};

	if ch == 0 {
		if let Some(ref mut console) = crate::terminal::CURRENT_CONSOLE {
			if scancode == 0x0E {
				console.move_cursor_left();
				console.change_char(b' ');
			}
		}
		return;
	}

	print!("{}", ch as char);
}

static mut SHIFT_PRESSED:	bool = false;
static mut CTRL_PRESSED:	bool = false;
static mut ALT_PRESSED:		bool = false;
static mut EXTENDED_KEY:	bool = false;

// IRQ 1 : Clavier
unsafe fn do_keyboard() {
	let scancode = inb(0x60); // lire le scancode depuis le port PS/2
	let released = scancode & 0x80 != 0;
	let key = scancode & 0x7F;

	if scancode == 0xE0 {
		EXTENDED_KEY = true;
		pic_send_eoi(1);
		return ;
	}
	
	if EXTENDED_KEY {
		EXTENDED_KEY = false;
		
		if !released {
			if let Some(ref mut console) = crate::terminal::CURRENT_CONSOLE {
				match key {
					0x4B => console.move_cursor_left(),
					0x4D => console.move_cursor_right(),
					0x48 => console.move_cursor_up(),
					0x50 => console.move_cursor_down(),
					_ => { print!("Do other"); }
				}
			}
		}
	} else {
		match key {
			0x2A | 0x36 => SHIFT_PRESSED = !released,
			0x1D		=> CTRL_PRESSED  = !released,
			_ => {
				if !released {
					handle_keypress(key);
				}
			}
		}
	}
	pic_send_eoi(1);
}

#[no_mangle]
pub unsafe extern "C" fn exception_handler() {
	asm!(
		"pushad",
		"call {}",
		"popad",
		"iretd",
		sym do_exception,
		options(noreturn)
	);
}

#[no_mangle]
pub unsafe extern "C" fn timer_handler() {
	asm!(
		"pushad",
		"call {}",
		"popad",
		"iretd",
		sym do_timer,
		options(noreturn)
	);
}

#[no_mangle]
pub unsafe extern "C" fn keyboard_handler() {
	asm!(
		"pushad",		  // Sauvegarde EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI
		"call {}",		 // Appelle ta logique Rust do_keyboard
		"popad",		   // Restaure tous les registres à l'identique
		"iretd",		   // Quitte l'interruption matérielle proprement !
		sym do_keyboard,
		options(noreturn)
	);
}

// ── Initialisation de l'IDT ───────────────────────────────────────
pub fn init_idt() {
	unsafe {
		// Remap le PIC avant tout
		pic_remap();

		// Exceptions CPU (0-31)
		for i in 0..32 {
			IDT[i] = IdtEntry::new(exception_handler as u32);
		}

		// IRQ matérielles (32-47 après remap)
		IDT[32] = IdtEntry::new(timer_handler	as u32); // IRQ 0 Timer
		IDT[33] = IdtEntry::new(keyboard_handler as u32); // IRQ 1 Clavier

		// Charger l'IDT avec LIDT
		let idt_ptr = IdtPointer {
			limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
			base:  IDT.as_ptr() as u32,
		};

		asm!("lidt [{}]", in(reg) &idt_ptr, options(nostack));

		// Activer les interruptions
		asm!("sti", options(nostack));
	}
}
