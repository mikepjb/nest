use std::env;
use std::str;
use std::io::{BufReader, Read};
use std::fs::File;

/// Entrypoint of the emulator, looks for a single argument which it assumes is a filepath to an
/// iNES file.
fn main() {
    println!("nest, the NES-trial emulator");
    if env::args().len() > 1 {

        let game_filepath = env::args().nth(1).unwrap();
        read_ines_file(game_filepath);
    }
}

// https://www.nesdev.org/wiki/INES#iNES_file_format
fn read_ines_file(game_filepath: String) {
    println!("game filepath: {:#?}", game_filepath);

    let file = File::open(game_filepath)
        .expect("File should be read");
    let mut reader = BufReader::new(file);
    // let mut buffer = Vec::new();
    let mut header = [0; 16];

    reader.read_exact(&mut header)
        .expect("header to be present at the beginning of the .nes file");

    println!("'NES' found at the start?: {}", str::from_utf8(&header[0..3]).unwrap() == "NES");

    let prg_rom_size = &header[4];
    println!("Size of PRG ROM: {} * 16 KB units", prg_rom_size);
    
    let chr_rom_size = &header[5];
    println!("Does the board (cartridge) use CHR RAM?: {}", chr_rom_size == &0u8);
    println!("Size of CHR ROM: {} * 8 KB units", chr_rom_size);

    // 6 -> flags 6
    // 7 -> flags 7
    // 8 -> flags 8 - PRG RAM size, rarely used extension
    // 9 -> flags 9 - TV system, rarely used extension
    // 10 -> flags 10 - unofficial, rarely used extension
    // 11-15 -> unused padding (should be filled with zero, but some rippers put their name across
    // 7-15)

    println!("{:#?}", header);

    // reader.read_to_end(&mut buffer)
    //     .expect("bytes should be read from file");

    // println!("{:#?}", buffer[0..3]);
    // for value in buffer {
    //     println!("BYTE: {}", value);
    // }
}
