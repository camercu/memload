use std::ffi::CString;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    memload::daemonize();

    let f = File::open("/bin/sleep").unwrap();
    let mut reader = BufReader::new(f);
    let mut bin = Vec::new();
    reader.read_to_end(&mut bin).unwrap();

    let argv = vec![
        CString::new("[kworker/u!0]").unwrap(),
        CString::new("600").unwrap(),
    ];
    let env = vec![];
    memload::run_from_mem(&bin, &argv, &env)
}
