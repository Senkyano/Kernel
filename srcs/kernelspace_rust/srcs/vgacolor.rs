#[repr(u8)] // spécifier que chaque data feras 8 bits donc 1 octet
#[allow(dead_code)]
pub enum	VgaColor {
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
