use std::{env::args, ffi::OsStr, path::Path};
#[macro_use]
extern crate structure;
fn get_crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;
    let table = generate_crc32_table();

    for byte in data.iter() {
        let index = ((crc ^ u32::from(*byte)) & 0xFF) as usize;
        crc = (crc >> 8) ^ table[index];
    }

    !crc
}
fn generate_crc32_table() -> [u32; 256] {
    const POLY: u32 = 0xEDB88320;

    let mut table = [0u32; 256];

    for i in 0..256 {
        let mut crc = i as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = POLY ^ (crc >> 1);
            } else {
                crc >>= 1;
            }
        }
        table[i] = crc;
    }

    table
}
fn crc32_exp(res: &mut String, pic: &mut Vec<u8>, cur_width: u32, real_crc32: u32, path: &Path) {
    let s = structure!("I");
    *res += &format!("STARTING CRC32!\n");
    for i in [cur_width].into_iter().chain(0..4096) {
        for j in 0..4096 {
            let mut data = pic[12..16].to_vec();
            data.append(&mut s.pack(i).unwrap());
            data.append(&mut s.pack(j).unwrap());
            data.extend(&pic[24..29]);

            let current_crc32: u32 = get_crc32(&data);
            if current_crc32 == real_crc32 {
                *res += &format!("CRC32KEY MATCHED!\n");
                *res += &format!("Real Width: {{{}}} Real Height: {{{}}}\n", i, j);
                pic.splice(12..29, data);
                let final_file_name =
                    "fix_".to_string() + path.file_name().unwrap().to_str().unwrap();
                let final_path = path.with_file_name(OsStr::new(&final_file_name));
                let _ = std::fs::write(final_path.clone(), pic);
                *res += &format!("Fixed File Save to {:?}\n", final_path);
                return;
            }
        }
    }
}
fn png_width_height(p: &str) -> String {
    let mut res = String::new();
    let path = Path::new(p);
    if let Ok(mut pic) = std::fs::read(path) {
        let crc32key = get_crc32(&pic[12..29]);
        let s = structure!("I");
        let real_crc32 = s.unpack(&pic[29..33]).unwrap().0;
        res += &format!(
            "CUR_CRC32: {:#x}\nREAL_CRC32: {:#x}\n",
            crc32key, real_crc32
        );
        if crc32key == real_crc32 {
        } else {
            let cur_width: u32 = s.unpack(&pic[16..20]).unwrap().0;
            crc32_exp(&mut res, &mut pic, cur_width, real_crc32, path)
        }
    } else {
        res += &format!("WRONG PATH!");
    }

    res
}
fn main() {
    let args: Vec<String> = args().collect();
    if let Some(k) = args.get(1) {
        let res = png_width_height(k);
        println!("{}", res);
    } else {
        println!("Input a valid path!");
    }
}
