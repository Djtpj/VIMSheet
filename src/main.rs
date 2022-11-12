use std::{fs, io};

mod mapline;
mod sheet;

#[cfg(test)]
mod tests;

use sheet::Sheet;

fn main() {
    println!("Where is your init.vim file?");
    let input = query_path();

    let sheet = Sheet::new(input);

    println!("Where would you like to put your csv? (Include the filename)");
    let output = query_path();

    fs::write(&output, sheet.serialize()).expect("Unable to write file.");

    println!("CSV exported to {output}");
}

fn query_path() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    return String::from(input.trim());
}


