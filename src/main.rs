mod process;

use process::process;

fn main() {
    match process(
        "test-data/init.txt".into(),
        "test-data/input.txt".into(),
        "output.tmp".into(),
    ) {
        Ok(_) => println!("Process completed successfully"),
        Err(e) => println!("Process failed: {:?}", e),
    }
}
