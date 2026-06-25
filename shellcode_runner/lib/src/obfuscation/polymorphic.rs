use crate::config::Config;

pub fn generate_polymorphic(_shellcode: &mut Vec<u8>, config: &Config) -> Result<(), String> {
    config.log("Generating polymorphic variant...");
    // TODO: impl
    Ok(())
}