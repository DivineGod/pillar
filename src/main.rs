use std::io::Read;

/// This should be called before calling any cli method or printing any output.
pub fn reset_signal_pipe_handler() -> nix::Result<()> {
    #[cfg(target_family = "unix")]
    {
        // SAFETY: Previous Handler is not invalid.
        unsafe {
            nix::sys::signal::signal(
                nix::sys::signal::Signal::SIGPIPE,
                nix::sys::signal::SigHandler::SigDfl,
            )?;
        }
    }

    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    reset_signal_pipe_handler()?;
    let stdin = std::io::stdin();
    let mut in_handle = stdin.lock();

    let mut buffer = String::new();
    in_handle.read_to_string(&mut buffer)?;
    print!("{}", buffer);

    Ok(())
}
