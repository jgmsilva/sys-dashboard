use std::process::Command;
use std::process::Stdio;
pub fn get_mem() -> (i32, i32) {
    let cmd1 = Command::new("free").stdout(Stdio::piped()).spawn().unwrap();
    let cmd2 = Command::new("free").stdout(Stdio::piped()).spawn().unwrap();
    let mem1 = Command::new("grep")
        .stdin(cmd1.stdout.unwrap())
        .stdout(Stdio::piped())
        .arg("Mem")
        .spawn()
        .unwrap();
    let mem2 = Command::new("grep")
        .stdin(cmd2.stdout.unwrap())
        .stdout(Stdio::piped())
        .arg("Mem")
        .spawn()
        .unwrap();
    let memTotal = String::from_utf8(
        Command::new("awk")
            .stdin(mem1.stdout.unwrap())
            .arg("{print $2}")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let memUsed = String::from_utf8(
        Command::new("awk")
            .stdin(mem2.stdout.unwrap())
            .arg("{print $3}")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    return (
        memUsed.trim().parse::<i32>().unwrap(),
        memTotal.trim().parse::<i32>().unwrap(),
    );
}
