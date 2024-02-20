mod process;

use process::process;

fn main() {
    match process("init.txt".into(), "input.txt".into(), "output.txt".into()) {
        Ok(_) => println!("Process completed successfully"),
        Err(e) => println!("Process failed: {:?}", e),
    }
}
