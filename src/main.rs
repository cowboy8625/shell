use std::io::Write;
use std::process;

const SIGINT: i32 = 2;
type SignalHandler = extern "C" fn(i32);

extern "C" {
    fn signal(signum: i32, handler: SignalHandler) -> SignalHandler;
}

extern "C" fn handle_sigint(_: i32) {
    process::exit(0);
}

fn main() {
    unsafe {
        // SAFETY: We are calling the `signal` function from libc, which is an external C function
        // that installs a signal handler for SIGINT (Ctrl+C). The function signature and the constant
        // `SIGINT` are correct for POSIX systems. Our handler function (`handle_sigint`) matches the
        // expected C signature: `extern "C" fn(i32)`, and performs only signal-safe operations
        // (a simple `println!` and `process::exit`).
        //
        // This usage is safe because:
        // - We do not access shared memory or mutable state from the signal handler.
        // - `process::exit` terminates immediately, so no further execution continues from the signal.
        // - We register the signal handler once, before any multi-threading starts.
        signal(SIGINT, handle_sigint);
    }

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        let read_bytes = std::io::stdin().read_line(&mut input).unwrap();
        if read_bytes == 0 {
            break;
        }
        println!("{}", input);
    }
}
