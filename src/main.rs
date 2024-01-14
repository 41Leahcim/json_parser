use std::{
    env::args,
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

use json_parser::Json;

struct BytesToChars<T> {
    iter: T,
}

impl<T: Iterator<Item = u8>> Iterator for BytesToChars<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Create an integer set to 0
        let mut value = 0;

        // Take at most 4 bytes, for the size of the utf-8 value
        for c in self.iter.by_ref().take(4) {
            // Shift the value 8 bits, and add the current byte
            value = value * 256 + c as u32;

            // Return the current character if it's valid utf-8
            if let Some(c) = char::from_u32(value) {
                return Some(c);
            }
        }

        // No character was found
        None
    }
}

fn main() {
    // Open the file
    let file = File::open(
        args()
            .nth(1)
            .expect("Pass a file when running this program"),
    )
    .expect("Failed to open json file");

    // Put the file into a buffer
    let reader = BufReader::new(file);

    // Add an iterator to convert the bytes to chars
    let iter = BytesToChars {
        iter: reader.bytes().map(|c| c.unwrap()),
    };

    let start = Instant::now();

    // Convert the iterator to json
    let json = Json::from_iter(iter);
    let generating_json = Instant::now();

    // Print the json
    println!("{json}\n");
    let printing_json = Instant::now();

    // Print performance
    println!(
        "Generated json in {} seconds",
        (generating_json - start).as_secs_f64()
    );
    println!(
        "Printed json in {} seconds",
        (printing_json - generating_json).as_secs_f64()
    );
}
