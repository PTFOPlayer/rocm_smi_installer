mod auto_installer;

fn main() {
    print!("

---------------------------
|rocm_smi_installer v0.1.0|
---------------------------

Please select type of instalation you want to start:

    1. Automatic - Recomended (check your distro, detect if your package manager has these libs and install them, otherewise install from source)
    2. From source (program will compile rocm-smi from source)
    3. DNF - Fedora
    4. Yum - RHEL
    5. Zypper - Suse
    6. Pacman - Manjaro/Arch

select option (default 1):
    ");
    let buf = &mut String::new();
    let reader = std::io::stdin();
    'l1: loop {
        if let Ok(len) = reader.read_line(buf) {
            match input_parser(len, buf) {
                Ok(_) => break 'l1,
                Err(_) => {
                    println!("input error try again:");
                }
            }
        } else {
            println!("input error try again:");
        }
    }
}

fn input_parser(_len: usize, buf: &str) -> Result<(), ()> {
    let buf = &buf[0..buf.len() - 1];
    if buf.is_empty() {
        auto_installer::auto_installer();
        return Ok(());
    }

    match buf {
        "1" => auto_installer::auto_installer(),
        "2" => todo!(),
        "3" => todo!(),
        "4" => todo!(),
        "5" => todo!(),
        "6" => todo!(),
        &_ => return Err(()),
    };
    Ok(())
}
