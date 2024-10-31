use std::path::Path;

mod test_tofrom_file;
mod traits;

fn main() {
    println!("Hello, world!");
    test_tofrom_file::do_thing(Path::new("output.bin"));
}
