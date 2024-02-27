mod process;

use process::process;

fn main() {
    match process(
        "init-dp.txt".into(),
        "input-dp.txt".into(),
        "output-dp.txt".into(),
    ) {
        Ok(_) => println!("Process completed successfully"),
        Err(e) => println!("Process failed: {:?}", e),
    }
}
