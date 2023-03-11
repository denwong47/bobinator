use std::io;
use std::io::Write;

use bobinator_macros::leave_trace;

#[cfg(feature = "trace")]
use conch::StringWrapper;

/// Flushing everything in the buffer of stdout, ensure that everything
/// is printed to screen up to this point.
#[allow(unused_variables)]
pub fn flush_stdout() {
    io::stdout().flush().unwrap_or_else(|err| {
        leave_trace!("Error flushing stdout" | "{}", err);
        ()
    })
}
