use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn wait_for_key() {
    let mut buf = [0u8; 1];
    let _ = io::stdin().read(&mut buf);
}

fn rename_file(old: &str, new: &str) -> io::Result<()> {
    let path = Path::new(old);

    if path.exists() {
        fs::rename(old, new)?;
        println!("Renamed: {} → {}", old, new);
    } else {
        println!("Skipped (not found): {}", old);
    }

    Ok(())
}

fn ensure_exists(path: &str) -> io::Result<()> {
    if !Path::new(path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Required files are missing:\n\
            - 東方紅魔鄉.exe\n\
            - custom.exe\n\
            - 紅魔鄉ED.dat\n\
            - 紅魔鄉ST.dat\n\
            - 紅魔鄉CM.dat\n\
            - 紅魔鄉MD.dat\n\
            - 紅魔鄉IN.dat\n\
            - 紅魔鄉TL.dat\n\
            Please ensure all files are present before running this patcher.",
        ));
    }
    Ok(())
}

fn replace_all(buf: &mut [u8], find: &[u8], replace: &[u8]) {
    assert_eq!(find.len(), replace.len());
    let len = find.len();
    let mut i = 0;

    while i + len <= buf.len() {
        if &buf[i..i + len] == find {
            buf[i..i + len].copy_from_slice(replace);
            i += len;
        } else {
            i += 1;
        }
    }
}

fn patch_file(original: &str, patches: &[(&[u8], &[u8])]) -> io::Result<()> {
    ensure_exists(original)?;
    let orig_path = Path::new(original);

    let backup_name = format!(
        "{}_original{}",
        orig_path.file_stem().unwrap().to_string_lossy(),
        orig_path.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default()
    );

    fs::rename(original, &backup_name)?;

    let mut data = fs::read(&backup_name)?;

    for (find, replace) in patches {
        replace_all(&mut data, find, replace);
    }

    fs::write(original, &data)?;

    println!("Patched: {} (backup: {})", original, backup_name);
    Ok(())
}

fn main() -> io::Result<()> {
    let patches: Vec<(&[u8], &[u8])> = vec![
        (
            &[0x8D, 0x67, 0x96, 0x82, 0x8B, 0xBD, 0x45, 0x44, 0x2E, 0x64, 0x61, 0x74, 0x00, 0x00, 0x00, 0x00], // (Shift-JIS) 紅魔鄉ED.dat
            &[0x6B, 0x6F, 0x75, 0x6D, 0x61, 0x6B, 0x79, 0x6F, 0x75, 0x45, 0x44, 0x2E, 0x64, 0x61, 0x74, 0x00], // koumakyouED.dat
        ),
        (
            &[0x8D, 0x67, 0x96, 0x82, 0x8B, 0xBD, 0x53, 0x54, 0x2E, 0x64, 0x61, 0x74, 0x00, 0x00, 0x00, 0x00], // (Shift-JIS) 紅魔鄉ST.dat
            &[0x6B, 0x6F, 0x75, 0x6D, 0x61, 0x6B, 0x79, 0x6F, 0x75, 0x53, 0x54, 0x2E, 0x64, 0x61, 0x74, 0x00], // koumakyouST.dat
        ),
        (
            &[0x8D, 0x67, 0x96, 0x82, 0x8B, 0xBD, 0x43, 0x4D, 0x2E, 0x64, 0x61, 0x74, 0x00, 0x00, 0x00, 0x00], // (Shift-JIS) 紅魔鄉CM.dat
            &[0x6B, 0x6F, 0x75, 0x6D, 0x61, 0x6B, 0x79, 0x6F, 0x75, 0x43, 0x4D, 0x2E, 0x64, 0x61, 0x74, 0x00], // koumakyouCM.dat
        ),
        (
            &[0x8D, 0x67, 0x96, 0x82, 0x8B, 0xBD, 0x4D, 0x44, 0x2E, 0x64, 0x61, 0x74, 0x00, 0x00, 0x00, 0x00], // (Shift-JIS) 紅魔鄉MD.dat
            &[0x6B, 0x6F, 0x75, 0x6D, 0x61, 0x6B, 0x79, 0x6F, 0x75, 0x4D, 0x44, 0x2E, 0x64, 0x61, 0x74, 0x00], // koumakyouMD.dat
        ),
        (
            &[0x8D, 0x67, 0x96, 0x82, 0x8B, 0xBD, 0x49, 0x4E, 0x2E, 0x64, 0x61, 0x74, 0x00, 0x00, 0x00, 0x00], // (Shift-JIS) 紅魔鄉IN.dat
            &[0x6B, 0x6F, 0x75, 0x6D, 0x61, 0x6B, 0x79, 0x6F, 0x75, 0x49, 0x4E, 0x2E, 0x64, 0x61, 0x74, 0x00], // koumakyouMD.dat
        ),
        (
            &[0x8D, 0x67, 0x96, 0x82, 0x8B, 0xBD, 0x54, 0x4C, 0x2E, 0x64, 0x61, 0x74, 0x00, 0x00, 0x00, 0x00], // (Shift-JIS) 紅魔鄉TL.dat
            &[0x6B, 0x6F, 0x75, 0x6D, 0x61, 0x6B, 0x79, 0x6F, 0x75, 0x54, 0x4C, 0x2E, 0x64, 0x61, 0x74, 0x00], // koumakyouED.dat
        ),
        (
            &[0x93, 0x8C, 0x95, 0xFB, 0x8D, 0x67, 0x96, 0x82, 0x8B, 0xBD, 0x2E, 0x63, 0x66, 0x67, 0x00, 0x00], // (Shift-JIS) 東方紅魔鄉.cfg
            &[0x6B, 0x6F, 0x75, 0x6D, 0x61, 0x6B, 0x79, 0x6F, 0x75, 0x2E, 0x63, 0x66, 0x67, 0x00, 0x00, 0x00], // koumakyou.cfg
        ),
    ];

    patch_file("東方紅魔鄉.exe", &patches)?;
    patch_file("custom.exe", &patches)?;
    rename_file("紅魔鄉ED.dat", "koumakyouED.dat")?;
    rename_file("紅魔鄉ST.dat", "koumakyouST.dat")?;
    rename_file("紅魔鄉CM.dat", "koumakyouCM.dat")?;
    rename_file("紅魔鄉MD.dat", "koumakyouMD.dat")?;
    rename_file("紅魔鄉IN.dat", "koumakyouIN.dat")?;
    rename_file("紅魔鄉TL.dat", "koumakyouTL.dat")?;
    rename_file("東方紅魔鄉.exe", "th06.exe")?;
    rename_file("東方紅魔鄉.cfg", "koumakyou.cfg")?;

    println!("TH6 executables backed up and patched.");
    println!("Press any key to continue...");
    wait_for_key();

    Ok(())
}
