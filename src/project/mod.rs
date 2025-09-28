use std::fs;

pub const APP_FILE: &'static str = "app.norm";
pub const LIB_FILE: &'static str = "lib.norm";

pub enum Target {
    Application,
    Library,
}

pub fn create(name: &str, target: Target) -> std::io::Result<()> {
    fs::create_dir(name)?;

    match target {
        Target::Application => {
            fs::write(String::from(name) + "/" + APP_FILE, "@app")?;
        }
        Target::Library => {
            fs::write(String::from(name) + "/" + LIB_FILE, "@lib")?;
        }
    }

    Ok(())
}

pub fn build() -> std::io::Result<()> {
    let build_dir = "build";

    if !fs::exists(build_dir)? {
        fs::create_dir(build_dir)?;
    }

    Ok(())
}
