use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn find_rc_exe() -> Option<PathBuf> {
    // 1. Try directly in PATH
    if Command::new("rc.exe").arg("/?").output().is_ok() {
        return Some(PathBuf::from("rc.exe"));
    }

    // 2. Check standard Windows Kits locations
    let kits_roots = [
        r"C:\Program Files (x86)\Windows Kits\10\bin",
        r"C:\Program Files\Windows Kits\10\bin",
    ];

    for root in kits_roots {
        let root_path = Path::new(root);
        if let Ok(entries) = fs::read_dir(root_path) {
            for entry in entries.flatten() {
                let x64_rc = entry.path().join("x64").join("rc.exe");
                if x64_rc.exists() {
                    return Some(x64_rc);
                }
            }
        }
    }

    None
}

fn main() {
    #[cfg(windows)]
    {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let ico_path = manifest_dir.join("assets").join("power.ico");

        if !ico_path.exists() {
            eprintln!("Warning: Icon not found at {}", ico_path.display());
            return;
        }

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let rc_path = out_dir.join("app.rc");
        let res_path = out_dir.join("app.res");

        // Windows resource files use forward slashes or escaped backslashes
        let ico_str = ico_path.to_str().unwrap().replace('\\', "/");
        let rc_content = format!("1 ICON \"{}\"\n", ico_str);
        if let Err(e) = fs::write(&rc_path, rc_content) {
            eprintln!("Warning: Failed to write rc file: {}", e);
            return;
        }

        if let Some(rc_exe) = find_rc_exe() {
            let status = Command::new(&rc_exe)
                .args(["/fo", res_path.to_str().unwrap(), rc_path.to_str().unwrap()])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("cargo:rustc-link-arg={}", res_path.display());
                }
                Ok(s) => {
                    eprintln!("Warning: rc.exe exited with code: {:?}", s.code());
                }
                Err(e) => {
                    eprintln!("Warning: Failed to run rc.exe: {}", e);
                }
            }
        } else {
            eprintln!("Warning: rc.exe could not be found to compile Windows icon");
        }

        println!("cargo:rerun-if-changed=assets/power.ico");
        println!("cargo:rerun-if-changed=build.rs");
    }
}
