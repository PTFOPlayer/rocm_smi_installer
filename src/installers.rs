use std::io::Write;

use sys_info;
pub fn auto_installer() -> Option<()> {
    let dist = sys_info::linux_os_release();
    match dist {
        Ok(res) => {
            println!("installing for: {}", res.pretty_name?);

            match res.id?.as_str() {
                "fedora" => fedora(),
                "rhel" => rhel(),
                "suse" => suse(),
                "manjaro" | "arch" => arch(),
                &_ => panic!("this method of instalation is not available, try instalation from source or select correct distro manualy"),
            }
        }
        Err(err) => println!("error during instalation::{:?}", err),
    }

    Some(())
}

pub fn repo_file(dist: &str) -> String {
    format!(
        "
[ROCm-5.5.3]
name=ROCm5.5.3
baseurl=https://repo.radeon.com/rocm/{}/5.5.3/main
enabled=1
priority=50
gpgcheck=1
gpgkey=https://repo.radeon.com/rocm/rocm.gpg.key
",
        dist
    )
}

pub fn fedora() {
    let mut file = match std::fs::File::create("/etc/yum.repos.d/rocm.repo") {
        Ok(res) => res,
        Err(err) => panic!("cannot operate on file: {:?}", err),
    };
    match file.write_all(repo_file("rhel9").as_bytes()) {
        Ok(_) => println!("succesfully written rocm.repo"),
        Err(err) => panic!("cannot operate on file: {:?}", err),
    }
    let mut manager = std::process::Command::new("dnf");
    manager.args(vec![
        "install",
        "rocm-smi",
        "rocm-smi-devel",
        "rocm-smi-lib",
    ]);
    println!("installing...");
    match manager.output() {
        Ok(res) => println!("{}\n{}\n{}", String::from_utf8_lossy(res.stdout.as_ref()), String::from_utf8_lossy(res.stderr.as_ref()), res.status),
        Err(err) => panic!("{}", err)
    }

    println!("succesfully installed rocm");
}

pub fn rhel() {
    let mut file = match std::fs::File::create("/etc/yum.repos.d/rocm.repo") {
        Ok(res) => res,
        Err(err) => panic!("cannot operate on file: {:?}", err),
    };
    match file.write_all(repo_file("rhel9").as_bytes()) {
        Ok(_) => println!("succesfully written rocm.repo"),
        Err(err) => panic!("cannot operate on file: {:?}", err),
    }
    let mut manager = std::process::Command::new("yum");
    manager.args(vec![
        "install",
        "--nogpgcheck",
        "rocblas",
        "rocm-smi-lib",
    ]);
    println!("installing...");
    match manager.output() {
        Ok(res) => println!("{}\n{}\n{}", String::from_utf8_lossy(res.stdout.as_ref()), String::from_utf8_lossy(res.stderr.as_ref()), res.status),
        Err(err) => panic!("{}", err)
    }

    println!("succesfully installed rocm-smi");
}

pub fn suse() {
    let mut file = match std::fs::File::create("/etc/zypp/repos.d/rocm.repo ") {
        Ok(res) => res,
        Err(err) => panic!("cannot operate on file: {:?}", err),
    };
    match file.write_all(repo_file("zyp").as_bytes()) {
        Ok(_) => println!("succesfully written rocm.repo"),
        Err(err) => panic!("cannot operate on file: {:?}", err),
    }
    let mut manager = std::process::Command::new("zypper");
    manager.args(vec![
        "install",
        "rocblas",
        "rocm-smi-lib",
    ]);
    println!("installing...");
    match manager.output() {
        Ok(res) => println!("{}\n{}\n{}", String::from_utf8_lossy(res.stdout.as_ref()), String::from_utf8_lossy(res.stderr.as_ref()), res.status),
        Err(err) => panic!("{}", err)
    }

    println!("succesfully installed rocm-smi");
}

pub fn arch() {
    let mut manager = std::process::Command::new("pacman");
    manager.args(vec![
        "-S",
        "rocm-smi-lib",
    ]);
    println!("installing...");
    match manager.output() {
        Ok(res) => println!("{}\n{}\n{}", String::from_utf8_lossy(res.stdout.as_ref()), String::from_utf8_lossy(res.stderr.as_ref()), res.status),
        Err(err) => panic!("{}", err)
    }

    println!("succesfully installed rocm-smi");
}
