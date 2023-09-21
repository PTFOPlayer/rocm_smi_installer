use sys_info;
pub fn auto_installer() -> Option<()> {
    let dist = sys_info::linux_os_release();
    match dist {
        Ok(res) => {
            println!("installing for: {}", res.pretty_name?);

            match res.id?.as_str() {
                "fedora" => todo!(),
                "rhel" => panic!("there was no budget for testing, currently not implemented"),
                "suse" => todo!(),
                "manjaro" => todo!(),
                &_ => todo!(),
            }
        }
        Err(err) => println!("error during instalation::{:?}", err),
    }

    Some(())
}
