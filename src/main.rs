#![warn(clippy::pedantic, clippy::nursery, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::blanket_clippy_restriction_lints,
    clippy::missing_trait_methods,
    clippy::arithmetic_side_effects,
    clippy::default_numeric_fallback,
    clippy::implicit_return,
    clippy::expect_used,
    clippy::print_stdout
)]

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
        for byte in self.iter.by_ref().take(4) {
            // Shift the value 8 bits, and add the current byte
            value = (value << 8) + u32::from(byte);

            // Return the current character if it's valid utf-8
            if let Some(ch) = char::from_u32(value) {
                return Some(ch);
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
        iter: reader.bytes().map_while(Result::ok),
    };

    let start = Instant::now();

    // Convert the iterator to json
    let json = iter.collect::<Json>();
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
