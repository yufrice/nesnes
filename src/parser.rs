use std::fs;
use std::io::prelude::Read;
use std::io::BufReader;

pub fn parser(path: &str) -> Result<(Vec<u8>, Vec<u8>, u8), String> {
    let f = fs::File::open(path).map_err(|v| format!("{}", v))?;
    let mut reader = BufReader::new(f).bytes();

    // header verify
    static VERIFY: &'static [u8; 4] = &[0x4E, 0x45, 0x53, 0x1A];
    for (b, v) in reader.by_ref().take(4).zip(VERIFY) {
        if let Ok(ref b) = b {
            if b != v {
                return Err("err".to_string());
            }
        }
    }

    // prg,chr pages
    static HEADER_END: usize = 0x0009;
    static PRG_SIZE: usize = 0x4000;
    static CHR_SIZE: usize = 0x2000;
    let prg_pages = (reader.next().ok_or("parse")?)
        .map_err(|err| err.to_string())
        .map(|b| PRG_SIZE * b as usize)?;
    let chr_pages = (reader.next().ok_or("parse")?)
        .map_err(|err| err.to_string())
        .map(|b| CHR_SIZE * b as usize)?;

    let flag6 = (reader.next().ok_or("parse")?).map_err(|err| err.to_string())?;

    let prg = reader
        .by_ref()
        .skip(HEADER_END)
        .take(prg_pages)
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|err| err.to_string())?;

    let chr = reader
        .take(chr_pages)
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|err| err.to_string())?;

    Ok((prg, chr, flag6))
}
