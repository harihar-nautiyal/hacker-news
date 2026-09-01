use std::path::Path;
use std::process::Command;

fn main() {
    // Re-run this build script if anything in `src` or `src/input.css` changes
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=src/input.css");

    // Ensure output directory exists
    if let Err(err) = std::fs::create_dir_all("static") {
        panic!("Failed to create static directory: {err}");
    }

    let input_css = "src/input.css";
    let output_css = "static/tailwind.css";

    if !Path::new(input_css).exists() {
        panic!("Input CSS file not found at '{input_css}'");
    }

    let is_release = std::env::var("PROFILE").map(|p| p == "release").unwrap_or(false);

    // Try finding the tailwindcss binary or runner
    let build_success = run_tailwind_build(input_css, output_css, is_release);

    if !build_success {
        panic!(
            "\n========================================================\n\
             Failed to build Tailwind CSS with any available runner!\n\
             Please ensure Tailwind CSS v4 is installed via one of:\n\
               1. `npm install` (recommended)\n\
               2. Standalone `tailwindcss` binary in PATH\n\
               3. `npx` or `bunx`\n\
             ========================================================\n"
        );
    }
}

fn run_tailwind_build(input: &str, output: &str, is_release: bool) -> bool {
    let mut candidates: Vec<Command> = Vec::new();

    // Candidate 1: Local node_modules binary
    let local_bin = if cfg!(windows) {
        "./node_modules/.bin/tailwindcss.cmd"
    } else {
        "./node_modules/.bin/tailwindcss"
    };
    if Path::new(local_bin).exists() {
        let mut cmd = Command::new(local_bin);
        cmd.args(["-i", input, "-o", output]);
        if is_release {
            cmd.arg("--minify");
        }
        candidates.push(cmd);
    }

    // Candidate 2: Global `tailwindcss` in PATH
    {
        let mut cmd = Command::new("tailwindcss");
        cmd.args(["-i", input, "-o", output]);
        if is_release {
            cmd.arg("--minify");
        }
        candidates.push(cmd);
    }

    // Candidate 3: npx @tailwindcss/cli
    {
        let mut cmd = if cfg!(windows) {
            Command::new("npx.cmd")
        } else {
            Command::new("npx")
        };
        cmd.args(["@tailwindcss/cli", "-i", input, "-o", output]);
        if is_release {
            cmd.arg("--minify");
        }
        candidates.push(cmd);
    }

    // Candidate 4: bunx @tailwindcss/cli
    {
        let mut cmd = Command::new("bunx");
        cmd.args(["@tailwindcss/cli", "-i", input, "-o", output]);
        if is_release {
            cmd.arg("--minify");
        }
        candidates.push(cmd);
    }

    for mut cmd in candidates {
        match cmd.output() {
            Ok(output) if output.status.success() => {
                return true;
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                eprintln!("Command {:?} exited with {}: \nSTDOUT:\n{}\nSTDERR:\n{}", cmd, output.status, stdout, stderr);
            }
            Err(_) => {
                // Command failed to execute (e.g. binary not found), try next candidate
            }
        }
    }

    false
}
