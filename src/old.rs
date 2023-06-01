use std::io::{BufReader, Read, Write};
use std::process::{Command, Stdio};

// fn test() {
//     println!("Hello, world!");
//
//     let output = Command::new("./runa")
//         .output()
//         .expect("failed to execute binary");
//     println!("status: {}", output.status);
//     println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
//     println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
// }

fn binary() {
    let mut app = Command::new("./run")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute binary!");

    let mut cin = app.stdin.take().unwrap();
    let mut cout = BufReader::new(app.stdout.take().unwrap());

    let test_str = String::from("test string\nasdasd");
    let mut buffer = Vec::new();

    // cin.write_all(&test_str.as_bytes()).unwrap();
    // cin.flush().unwrap();

    match app.wait_with_output() {
        Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
        Err(_) => todo!(),
    }
    cout.read_to_end(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));
}

fn main2() {
    binary();
}
