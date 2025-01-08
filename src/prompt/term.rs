pub(crate) struct TermSize {
    window_max_rows: usize,
    window_pos: usize,
}

impl Default for TermSize {
    fn default() -> Self {
        let mut window_max_rows = usize::MAX;

        if let Some(termsize) = get_term_size() {
            window_max_rows = (termsize.rows as usize)
                .checked_sub(3)
                .unwrap_or(termsize.rows as usize);
        }

        Self {
            window_max_rows,
            window_pos: 0,
        }
    }
}

impl TermSize {
    pub fn get_max_rows(&self) -> usize {
        self.window_max_rows
    }

    pub fn set_max_rows(&mut self, rows: usize) {
        self.window_max_rows = rows;
    }

    pub fn get_pos(&self) -> usize {
        self.window_pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.window_pos = pos;
    }
}

// IMPORTANT - Everything bellow this should be removed once
// https://github.com/softprops/termsize/pull/24 is merged!
// and the forbid unsafe rule should be reenabled!

#[cfg(unix)]
use std::io::IsTerminal;

#[cfg(unix)]
use std::ffi::{c_ushort, CString};

#[cfg(unix)]
use libc::{ioctl, O_RDONLY, STDOUT_FILENO, TIOCGWINSZ};

/// A representation of the size of the current terminal
#[repr(C)]
#[derive(Debug)]
#[cfg(unix)]
pub struct UnixSize {
    /// number of rows
    pub rows: c_ushort,
    /// number of columns
    pub cols: c_ushort,
    x: c_ushort,
    y: c_ushort,
}


/// Workaround for SSH terminal size
pub fn get_term_size() -> Option<termsize::Size> {
    #[cfg(not(unix))]
    {
        termsize::get()
    }

    #[cfg(unix)]
    {
        _get_unix_termsize()
    }
}

/// Gets the current terminal size
#[cfg(unix)]
fn _get_unix_termsize() -> Option<termsize::Size> {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    if !std::io::stdout().is_terminal() {
        return None;
    }
    let mut us = UnixSize {
        rows: 0,
        cols: 0,
        x: 0,
        y: 0,
    };

    let fd = if let Ok(ssh_term) = std::env::var("SSH_TTY") {
        // Convert path to a C-compatible string
        let c_path = CString::new(ssh_term).expect("Failed to convert path to CString");

        // Open the terminal device
        let fd = unsafe { libc::open(c_path.as_ptr(), O_RDONLY) };
        if fd < 0 {
            return None; // Failed to open the terminal device
        }

        fd
    } else {
        STDOUT_FILENO
    };

    let r = unsafe { ioctl(fd, TIOCGWINSZ, &mut us) };

    // Closing the open file descriptor
    if fd != STDOUT_FILENO {
        unsafe { libc::close(fd); }
    }

    if r == 0 {
        Some(termsize::Size {
            rows: us.rows,
            cols: us.cols,
        })
    } else {
        None
    }
}
