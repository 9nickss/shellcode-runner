use crate::config::Config;
use iced_x86::{Decoder, DecoderOptions, Instruction};

pub fn inject_junk(shellcode: &mut Vec<u8>, density: f32, config: &Config) -> Result<(), String> {
    if !(0.0..=1.0).contains(&density) {
        return Err("Density must be 0.0-1.0".to_string());
    }

    let instrs = decode_instructions(shellcode);
    for (offset, bytes) in &instrs {
        config.log(&format!("  [{:#06x}] {} bytes: {:02X?}", offset, bytes.len(), bytes));
    }

    config.log(&format!("Injecting junk code ({}%)...", density * 100.0));
    // TODO: impl
    Ok(())
}

pub fn decode_instructions(shellcode: &[u8]) -> Vec<(usize, Vec<u8>)> {
    let mut decoder = Decoder::with_ip(
        64,
        shellcode,
        0,
        DecoderOptions::NONE
    );
    let mut result = Vec::new();
    let mut instruction = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        let offset = instruction.ip() as usize;
        let len = instruction.len();
        let bytes = shellcode[offset..offset + len].to_vec();
        if instruction.is_invalid() {
            break;
        }
        result.push((offset, bytes));
    }
    return result
}
