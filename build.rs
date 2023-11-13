use std::{env, fs::File, path::Path, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    built::write_built_file().expect("Failed to acquire build-time information");

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    {
        let glibc = glibc_version::glibc_version().unwrap();
        let dest_path = Path::new(&env::var("OUT_DIR").expect("OUT_DIR not set")).join("glibc_version.rs");
        let mut glibc_file = File::create(dest_path)?;
        glibc_file.write_all(
            format!("pub const GLIBC_VERSION: &str = \"glibc {}.{}\";\n",
            glibc.major, glibc.minor).as_ref()
        )?;
    }

    Ok(())
}