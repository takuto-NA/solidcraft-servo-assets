//! Wrap the Rust module without requiring an installed wasm-tools binary.

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let input = args.next().ok_or("usage: componentize INPUT.wasm OUTPUT.wasm")?;
    let output = args.next().ok_or("usage: componentize INPUT.wasm OUTPUT.wasm")?;
    let module = std::fs::read(input)?;
    let component = wit_component::ComponentEncoder::default()
        .module(&module)?
        .validate(true)
        .encode()?;
    let temporary = format!("{output}.tmp");
    std::fs::write(&temporary, component)?;
    std::fs::rename(&temporary, output)?;
    Ok(())
}
