use std::{env::args, ffi::OsStr, path::Path};

fn detect_unvisible_zip(p: &str) -> String {
    let mut res = String::new();
    let path = Path::new(p);
    let final_file_name =
        "extracted_".to_string() + path.file_name().unwrap().to_str().unwrap() + ".zip";
    let final_path = path.with_file_name(OsStr::new(&final_file_name));
    if let Ok(pic) = std::fs::read(path) {
        let mut cur = 0u64;
        for i in 1..pic.len() {
            cur = (cur << 8) + pic[i] as u64;
            cur = cur & 0xFFFFFFFF;
            if cur == 0x504b0304 {
                res += &format!("Found unvisible zip at byte {:#x}\n", i);
                let _ = std::fs::write(final_path.clone(), pic);

                res += &format!("Extract to {:?}\n", final_path);
                return res;
            }
        }
    } else {
        res += &format!("WRONG PATH!");
    }
    res += &format!("Can't find unvisible zip!");
    res
}
fn main() {
    let args: Vec<String> = args().collect();
    if let Some(k) = args.get(1) {
        let res = detect_unvisible_zip(k);
        println!("{}", res);
    } else {
        println!("Input a valid path!");
    }
}
