use std::io;
use std::io::Write;

use bobinator_macros::leave_trace;

#[cfg(feature = "trace")]
use conch::StringWrapper;

#[allow(unused_variables)]
pub fn flush_stdout() {
    io::stdout().flush().unwrap_or_else(|err| {
        leave_trace!("Error flushing stdout" | "{}", err);
        ()
    })
}
