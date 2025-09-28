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
