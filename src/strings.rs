use std::io::{Write, Read};

use extra::OptionalExt;

/// A trait for characters/bytes that can be printable.
pub trait IsPrintable {
    /// Is this character printable?
    fn is_printable(self) -> bool;
}

impl IsPrintable for u8 {
    #[inline]
    fn is_printable(self) -> bool {
        // TODO handle unicode.
        self >= 0x20 && self <= 0x7e
    }
}

#[derive(Copy, Clone)]
struct Trailing {
    chars: [u8; 4],
    current: usize,
}

// Wow, such premature, much optimization
#[allow(dead_code)] // DAFUQ rustc?
impl Trailing {
    #[inline]
    fn new() -> Trailing {
        Trailing {
            chars: [0; 4],
            current: 0,
        }
    }

    #[inline]
    fn set(&mut self, b: u8) -> bool {
        self.chars[self.current] = b;
        self.current += 1;

        self.is_complete()
    }

    #[inline]
    fn reset(&mut self) {
        self.current = 0;
    }

    #[inline]
    fn is_complete(self) -> bool {
        self.current == 4
    }

    #[inline]
    fn chars(self) -> [u8; 4] {
        self.chars
    }
}

/// Read a stream of bytes and output printable strings of length 4 or more seperated by 0x0A
/// (NL)
pub fn read<R: Read, W: Write>(stdin: R, mut stdout: W) {
    let mut trailing = Trailing::new();

    for i in stdin.bytes().map(|x| x.try()) {
        if i.is_printable() {
            if trailing.is_complete() {
                stdout.write(&[i]).try();
            } else if trailing.set(i) {
                stdout.write(&trailing.chars()).try();
            }
        } else {
            if trailing.is_complete() {
                stdout.write(&[b'\n']).try();
            }
            trailing.reset();
        }
    }
}