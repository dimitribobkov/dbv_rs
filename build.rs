use std::process::Command;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=program/test.dbv");
    
    let output = Command::new("cmd")
    .args(&["/C", "customasm ./program/source/test.asm -o ./program/test.dbv & customasm ./program/source/test.asm -o ./program/test-out.txt -f annotated"])
    .output()
    .expect("failed to execute process");

    for out in String::from_utf8(output.stdout).iter() {
        println!("{}", out);
    }
}