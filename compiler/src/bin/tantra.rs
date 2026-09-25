use std::{env, fs, process};

fn main() {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: tantra <file.tantra>");
            process::exit(2);
        }
    };

    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("T0000: file વાંચી શકાયું નથી: {error}");
            process::exit(1);
        }
    };

    let (_, diagnostics) = tantra_compiler::parse_source(&source);
    for diagnostic in &diagnostics {
        eprintln!(
            "{} {:?} [{}..{}]: {}",
            diagnostic.code,
            diagnostic.severity,
            diagnostic.span.start,
            diagnostic.span.end,
            diagnostic.message
        );
    }

    if diagnostics
        .iter()
        .any(|d| d.severity == tantra_compiler::diagnostic::Severity::Error)
    {
        process::exit(1);
    }

    println!("Tantra syntax check: OK");
}
