use installers::{arch, auto_installer, fedora, rhel, suse};

mod installers;

fn main() {
    match sudo::escalate_if_needed() {
        Ok(res) => println!("running as {:?}", res),
        Err(err) => panic!("permisions not given: {}", err),
    }
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
    if let Ok(len) = reader.read_line(buf) {
        input_parser(len, buf).expect("input error, try again");
    }
}

fn input_parser(_len: usize, buf: &str) -> Result<(), ()> {
    let buf = &buf[0..buf.len() - 1];
    if buf.is_empty() {
        auto_installer();
        return Ok(());
    }

    match buf {
        "1" => {
            installers::auto_installer();
        }
        "2" => println!("currently not implemented, please wait for future versions"),
        "3" => fedora(),
        "4" => rhel(),
        "5" => suse(),
        "6" => arch(),
        &_ => return Err(()),
    };
    Ok(())
}
