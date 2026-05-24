use std::{fs, path::PathBuf};

use clap::{Arg, Command, command, value_parser};

fn main() {
    let matches = command!()
        .about("A CLI tool to unpack/pack DVPL Resource Archive")
        .subcommand_required(true)
        .subcommand(
            Command::new("pack")
            .about("Pack input file into DVPL Resource Archive")
            .arg(
                Arg::new("file").help("Input file path")
                .required(true)
                .value_parser(value_parser!(PathBuf))
            )
        )
        .subcommand(
            Command::new("unpack")
            .about("Unpack DVPL Resource Archive")
            .arg(
                Arg::new("file").help("DVPL Resource Archive path")
                .required(true)
                .value_parser(value_parser!(PathBuf))
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("pack") {
        let path = matches.get_one::<PathBuf>("file").unwrap();
        pack_program(path).unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("unpack") {
        let path = matches.get_one::<PathBuf>("file").unwrap();
        unpack_program(path).unwrap();
    }

}

fn pack_program(source: &PathBuf) -> Result<(), std::io::Error>{
    let source = source;
    let mut destination = PathBuf::from(source);
    destination.add_extension("dvpl");

    println!("Packing: {} -> {}", source.display(), destination.display());

    let input_bytes = fs::read(source)?;
    let mut output_bytes = lz4::block::compress(
        &input_bytes, 
        Some(lz4::block::CompressionMode::HIGHCOMPRESSION(999)), 
        false
    )?;

    let mut footer_bytes= vec![0u8; 20];
    let footer = Footer::new(input_bytes.len(), output_bytes.len(), 
    crc32fast::hash(&output_bytes), 2);
    footer.write(&mut footer_bytes);
    output_bytes.extend_from_slice(&footer_bytes);


    fs::write(destination, output_bytes)?;
    Ok(())
}

fn unpack_program(source: &PathBuf) -> Result<(), std::io::Error> {
    let source = source;
    let mut destination = PathBuf::from(source);
    
    if destination.extension().and_then(|f|f.to_str()) == Some("dvpl") {
        destination.set_extension("");
    }

    println!("Unpacking: {} -> {}", source.display(), destination.display());

    let input_bytes = fs::read(source)?;

    if input_bytes.len() < 20 {
        panic!("Input file is not big enough to be a DVPL Resource Archive. It is less than 20 bytes long")
    }

    let compressed_bytes = &input_bytes[..input_bytes.len() - 20];
    let footer_bytes = &input_bytes[input_bytes.len() - 20..].to_vec();
    let footer = Footer::from_vec(footer_bytes);
    let output_bytes = lz4::block::decompress(compressed_bytes, 
        Some(footer.input_size.try_into().unwrap())
    )?;
    fs::write(destination, output_bytes)?;
    Ok(())
}

#[derive(Debug)]
struct Footer {
    input_size: u32,
    compressed_size: u32,
    compressed_checksum: u32,
    compression_format: u32,
}
impl Footer {
    fn new(input_size: usize, compressed_size: usize, compressed_checksum: u32, compression_format: u32) -> Footer {
        Footer{ 
            input_size: input_size.try_into().unwrap(), 
            compressed_size: compressed_size.try_into().unwrap(), 
            compressed_checksum, 
            compression_format
        }
    }
    fn write(&self, target_vec: &mut Vec<u8>){
        if target_vec.len() != 20 {
            panic!("PANIC!: Footer.write need to write to a Vec<u8> that is exactly 20 bytes")
        }
        target_vec[ 0 ..  4].copy_from_slice(&self.input_size.to_le_bytes());
        target_vec[ 4 ..  8].copy_from_slice(&self.compressed_size.to_le_bytes()); 
        target_vec[ 8 .. 12].copy_from_slice(&self.compressed_checksum.to_le_bytes());
        target_vec[12 .. 16].copy_from_slice(&self.compression_format.to_le_bytes()); 
        target_vec[16 .. 20].copy_from_slice(&*b"DVPL");
    }
    fn from_vec(target_vec: &Vec<u8>) -> Footer{
        if target_vec.len() != 20 {
            panic!("PANIC!: Footer.from_vec need to read from a Vec<u8> that is exactly 20 bytes")
        }

        Footer { 
            input_size:          u32::from_le_bytes(target_vec[ 0 ..  4].try_into().unwrap()), 
            compressed_size:     u32::from_le_bytes(target_vec[ 4 ..  8].try_into().unwrap()), 
            compressed_checksum: u32::from_le_bytes(target_vec[ 8 .. 12].try_into().unwrap()), 
            compression_format:  u32::from_le_bytes(target_vec[12 .. 16].try_into().unwrap())
        }
    }
}

