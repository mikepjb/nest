use std::env;
use std::str;
use std::io::{BufReader, Read};
use std::fs::File;
// use std::convert::TryFrom;

/// Entrypoint of the emulator, looks for a single argument which it assumes is a filepath to an
/// iNES file.
fn main() {
    println!("nest, the NES-trial emulator");
    if env::args().len() > 1 {

        let game_filepath = env::args().nth(1).unwrap();
        read_ines_file(game_filepath);
    }
}

#[derive(Debug)]
enum NametableArrangement {
    Vertical,
    Horizontal
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

    // let prg_rom_size = u32::from_be_bytes(<[u8; 4]>::try_from(&header[4]).unwrap());
    let prg_rom_size = &header[4];
    println!("Size of PRG ROM: {} * 16 KB units", prg_rom_size);
    
    let chr_rom_size = &header[5];
    println!("Does the board (cartridge) use CHR RAM?: {}", chr_rom_size == &0u8);
    println!("Size of CHR ROM: {} * 8 KB units", chr_rom_size);

    let flags_6 = &header[6]; // as in "flags stored on byte 6 of the header"
    let trainer_used = (flags_6 >> 2 & 1) == 1u8;
    let nametable_arrangement_bit = flags_6 >> 0 & 1;
    let mut nametable_arrangement = NametableArrangement::Vertical;
    if nametable_arrangement_bit == 1u8 {
        nametable_arrangement = NametableArrangement::Horizontal;
    }
    println!("Nametable Arrangement: {:#?}", nametable_arrangement);

    println!("Trainer used?: {}", trainer_used);
    println!("flags_6: {:08b}", flags_6);

    // header
    // trainer 0/512
    // prg 16384 * x bytes

    // let chr_start_position = 16 + 0 + (prg_rom_size * 16384);
    // let chr_end_position = chr_start_position + (chr_rom_size * 8192);
    // println!("{chr_start_position} to {chr_end_position}");
    // TODO: for now just statically calculate:
    let chr_start_position = 16 + 0 + (8 * 16384);
    // let chr_end_position = chr_start_position + (16 * 8192);
    // should have been 262160 but now using 262144 with the assumption that reading kinda pops the
    // buffer.
    // Yes, it would seem that reading pops the buffer.
    const CHR_END_POSITION: usize = 262144;
    let chr_count = CHR_END_POSITION - chr_start_position;
    println!("{chr_start_position}:{CHR_END_POSITION}");
    println!("{chr_start_position}:{CHR_END_POSITION}");
    println!("{chr_count}");

    // 131072 after header, up to start of CHR
    // 131056 remaining CHR only
    //
    //

    const TO_CHR_START: usize = 131072;
    let mut _up_to_chr = [0; TO_CHR_START];
    reader.read_exact(&mut _up_to_chr)
        .expect("all data up to CHR is read");

    const CHR_SIZE: usize = 131056;
    let mut chr = [0; CHR_SIZE];
    reader.read_exact(&mut chr)
        .expect("Entire CHR is read");

    for tile in chr.chunks(16) {
        println!("{:#?}", tile);
    }



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
