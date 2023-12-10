use std::fs::File;
use std::io::{self, BufRead};

struct SimpleLines {
    reader: io::Lines<io::BufReader<File>>,
}

impl SimpleLines {
    fn from(reader: io::Lines<io::BufReader<File>>) -> Self {
        SimpleLines { reader }
    }
}

impl Iterator for SimpleLines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.reader.next();

        next.map(|value| value.unwrap())
    }
}

pub fn read_lines_from_file(file_name: &str) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(file_name)?;

    Ok(SimpleLines::from(io::BufReader::new(file).lines()))
}
