use crate::config::Config;

pub fn inject_junk(shellcode: &mut Vec<u8>, density: f32, config: &Config) -> Result<(), String> {
    if !(0.0..=1.0).contains(&density) {
        return Err("Density must be 0.0-1.0".to_string());
    }
    config.log(&format!("Injecting junk code ({}%)...", density * 100.0));
    // TODO: impl
    Ok(())
}
