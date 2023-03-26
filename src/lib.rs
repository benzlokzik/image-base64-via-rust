use pyo3::prelude::*;
use std::{
    io::{BufReader, Read, Write},
    fs::File,
    path::Path,
    ffi::OsString,
};
use base64;

//  TODO: use not deprecated methods
// use base64::Engine;

/// The following Rust code defines a function that takes a file name as input, reads the contents
/// of the file, encodes it to base64, and writes the encoded data to a new file
#[pyfunction]
fn img_to_base64(file_name: &str, input: &str) -> PyResult<String> {
    if Path::exists((&file_name).as_ref()) {
        // Open the file in read-only mode and create a buffered reader
        let file = File::open(file_name).expect("Failed to open file");
        let mut buf_reader = BufReader::new(file);


        // Read the contents of the file into a vector
        let mut content = Vec::new();
        buf_reader.read_to_end(&mut content).expect("Failed to read file");


        // Encode the vector to base64
        let encoded = base64::encode(content);


        // TODO: use not deprecated methods
        // let mut engine = Engine::new();
        // let mut engine = Engine::<Config = base64::DefaultConfig,
        //     DecodeEstimate = base64::DefaultDecodeEstimator>::new();
        // let encoded = engine.encode(&content);
        // let mut engine = <dyn base64::Engine<Config=(), DecodeEstimate=()>>::new();
        // let encoded = engine.encode(&content);
        // let encoded = base64::Engine::encode(content, ());

        // Create a new file with the same name as the input file but with a .b64.txt extension,
        // and write the encoded data to it
        let mut output_file_name = Path::new(file_name)
            .file_stem()
            .expect("Failed to get file stem")
            .to_owned();
        output_file_name.push(".b64.txt");
        let mut output_file = File::create(output_file_name.clone()).expect("Failed to create output file");
        output_file
            .write_all(encoded.as_bytes())
            .expect("Failed to write to output file");

        println!("File successfully saved as {:?}", OsString::from(output_file_name));
    } else {
        println!("File doesn't exist")
    }
    // Convert each character from second argument to its corresponding Unicode code point value
    let ord_values: Vec<u32> = input.chars().map(|c| c as u32).collect();


    // Convert the vector of code point values to a string with each value separated by a space
    let ord_str = ord_values
        .iter()
        .map(|&v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    // Return the string as a PyResult
    Ok(ord_str)
}


/// A Python module implemented in Rust.
#[pymodule]
fn img_and_base64(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(img_to_base64, m)?)?;
    Ok(())
}