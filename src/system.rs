use std::process::{Command, Stdio};
pub fn get_kernel() -> String {
    let name = String::from_utf8(Command::new("uname").arg("-s").output().unwrap().stdout).unwrap();
    let version =
        String::from_utf8(Command::new("uname").arg("-r").output().unwrap().stdout).unwrap();
    return format!("{} {}", name, version);
}

pub fn get_cpu() -> String {
    let ls = Command::new("lscpu")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let cpu = String::from_utf8(
        Command::new("grep")
            .stdin(ls.stdout.unwrap())
            .arg("Model name")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    return cpu;
}
