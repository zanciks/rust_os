use volatile::Volatile; // lets us avoid unwanted optimizations
use core::fmt; // to allow for normal writing macros
use lazy_static::lazy_static; // lazy static dependency
use spin::Mutex; // used to make the writer mutable static

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// global writer, which lets us write from anywhere! Must be a lazy static to work
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// we use a C like enum to number all of the colors. u4 would work, but Rust doesn't support it
pub enum Color {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
// a color code is essentially a background + foreground
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
        // we shift the background by 4, as we essentially only want a u4,
        // but again, Rust doesn't support those. We can combine the two colors
        // into a single u8 this way, which is twice as efficient memory-wise.
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
// the buffer is simply where text goes. We set the size with our constants,
// and then we can write characters with a given ascii code and ColorCode.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// we will use this to write to the screen.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // using \n is an accepted way to write a new line
            byte => {
                if self.column_position >= BUFFER_WIDTH { // wrap-around text to new line
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // we are going to set the existing char to whatever our byte is
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1; // shift over 1 character-space
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // we take every character in line row, and then copy them to row - 1
                // (we are moving the lines up
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        // clear the very bottom row
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0; // reset cursor column
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ', // empty byte
            color_code: self.color_code,
        };
        // we just iter through all one row and replace everything with the empty ScreenChar
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}