use volatile::Volatile;

// Ask compiler to not warn on unused enum variants.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// Each variant is to be stored as an u8.
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// ColourCode represents the full colour -- foreground and background.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// Ensure ColourCode has same data layout as an u8.
#[repr(transparent)]
pub struct ColourCode(u8);

impl ColourCode {
    fn new(fg: Colour, bg: Colour) -> ColourCode {
        ColourCode((bg as u8) << 4 | (fg as u8))
    }
}

// ScreenChar represents the actual byte to be printed and its control
// code to control settings such as colour, blinking, etc.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// By defualt Rust doesn't guarantee ordering between struct members.
// So, use C repr to guarantee struct ordering (similar to C).
// See https://doc.rust-lang.org/nightly/nomicon/other-reprs.html#reprc
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    colour_code: ColourCode,
}

// Buffer represents the actual VGA text buffer which is 80-chars wide
// and 25-lines long. Each character takes 2-bytes -- one each for
// actual ASCII char and control code.
//
// Mark the actual memory as volatile so that Rust compiler doesn't
// peform any optimisations. For e.g., Rust compiler doesn't know that
// we are actully writing to VGA h/w; it might think we are writing to
// some memory and never reading that data, and thus might optimise to
// remove the writes entirely!
const BUF_HEIGHT: usize = 25;
const BUF_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUF_WIDTH]; BUF_HEIGHT],
}

// Writer is used to actually write to the VGA text buffer.
pub struct Writer {
    // column_pos tracks the current position in the last row.
    column_pos: usize,

    // colour_code is the foreground/background colour used.
    colour_code: ColourCode,

    // buffer is the VGA text buffer.
    buffer: &'static mut Buffer,
}

impl Writer {
    // new() reutrns a Writer with given ColourCode and column position
    // set to 0.
    pub fn new(cc: ColourCode) -> Writer {
        Writer {
            column_pos: 0,
            colour_code: cc,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    // write_byte() writes the given 'byte' to VGA text buffer.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                // Move to next line if we have exhausted our
                // char-width limit.
                if self.column_pos >= BUF_WIDTH {
                    self.new_line();
                }

                let row = BUF_HEIGHT - 1;
                let col = self.column_pos;

                let colour_code = self.colour_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    colour_code,
                });
                self.column_pos += 1;
            }
        }
    }

    // write_string() writes the given string, 's', to VGA text buffer.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            // VGA only supports ASCII (actually it supports code page
            // 437 which is almost close to ASCII).
            // So, only allow printable-ASCII characters. For
            // unprintable chars, use code 0xfe.
            match byte {
                // Printable ASCII chars or newline.
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Unprintable chars use 0xfe.
                _ => self.write_byte(0xfe),
            }
        }
    }

    // new_line() advances to next line in VGA text buffer.
    fn new_line(&mut self) {}
}

pub fn print_something() {
    let mut w = Writer::new(ColourCode::new(Colour::Red, Colour::Black));

    w.write_byte(b'S');
    w.write_string("omething");
    w.write_byte(b'!');
}
