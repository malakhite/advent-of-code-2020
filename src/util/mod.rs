use std::fs::{File};
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::error;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file(s: &String) -> Result<String, Box<dyn error::Error>> {
    let path = Path::new(s);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", &path.display(), why),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => (),
        Err(why) => panic!("couldn't read {}: {}", &path.display(), why),
    }
    Ok(s)
}