use std::io;
use std::io::Write;

use bobinator_macros::leave_trace;

pub fn flush_stdout() {
    io::stdout().flush().unwrap_or_else(|err| {
        leave_trace!("Error flushing stdout" | "{}", err);
    })
}
