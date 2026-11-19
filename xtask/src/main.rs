//! Local pre-commit and CI checks; real geometry checks require SOLIDCRAFT_CLI.

mod check;

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

type TaskResult<T = ()> = Result<T, Box<dyn Error>>;

fn run_command(repo: &Path, program: &std::ffi::OsStr, args: &[&str]) -> TaskResult {
    println!("Running {} {}", program.to_string_lossy(), args.join(" "));
    let status = Command::new(program).args(args).current_dir(repo).status()?;
    if !status.success() {
        return Err(format!("{} failed with {status}", program.to_string_lossy()).into());
    }
    Ok(())
}

fn ci(repo: &Path) -> TaskResult {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    run_command(repo, &cargo, &["fmt", "--all", "--", "--check"])?;
    run_command(
        repo,
        &cargo,
        &[
            "fmt",
            "--manifest-path",
            "tools/componentize/Cargo.toml",
            "--",
            "--check",
        ],
    )?;
    for legacy in [false, true] {
        let mut args = vec!["test", "--locked", "--offline", "--workspace"];
        if legacy {
            args.extend(["--features", "solidcraft-servo-assets/legacy-host"]);
        }
        run_command(repo, &cargo, &args)?;
    }
    for legacy in [false, true] {
        let mut args = vec!["clippy", "--locked", "--offline", "--workspace", "--all-targets"];
        if legacy {
            args.extend(["--features", "solidcraft-servo-assets/legacy-host"]);
        }
        args.extend(["--", "-D", "warnings"]);
        run_command(repo, &cargo, &args)?;
    }
    run_command(
        repo,
        &cargo,
        &[
            "test",
            "--locked",
            "--offline",
            "--manifest-path",
            "tools/componentize/Cargo.toml",
        ],
    )?;
    run_command(
        repo,
        &cargo,
        &[
            "clippy",
            "--locked",
            "--offline",
            "--manifest-path",
            "tools/componentize/Cargo.toml",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    check::all(repo)?;
    run_command(repo, "bash".as_ref(), &["build.sh"])?;
    run_command(repo, "bash".as_ref(), &["build-legacy.sh"])?;
    if let Some(cli) = env::var_os("SOLIDCRAFT_CLI").filter(|value| !value.is_empty()) {
        for (package, output) in [
            ("dist/servo-assets", "target/host-tests/current"),
            ("dist/legacy/servo-assets", "target/host-tests/legacy"),
        ] {
            run_command(repo, &cli, &["plugin", "check", package, "--json"])?;
            run_command(repo, &cli, &["plugin", "test", package, "--json", "--out", output])?;
        }
    } else {
        println!(
            "NOT RUN: real SolidCraft host checks. Set SOLIDCRAFT_CLI to a compatible solidcraft-cli to run them."
        );
    }
    println!("CI gate passed.");
    Ok(())
}

fn run() -> TaskResult {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("xtask must be a direct child of the repository")?
        .to_path_buf();
    let args: Vec<_> = env::args().skip(1).take(3).collect();
    if args.len() != 1 {
        return Err("usage: cargo xtask check | ci".into());
    }
    match args.first().map(String::as_str) {
        Some("check") => check::all(&repo),
        Some("ci") => ci(&repo),
        _ => Err("usage: cargo xtask check | ci".into()),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask: {error}");
            ExitCode::FAILURE
        }
    }
}
