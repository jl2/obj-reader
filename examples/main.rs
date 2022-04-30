// main.rs
//
// Copyright (c) 2022 Jeremiah LaRocco <jeremiah_larocco@fastmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

extern crate argparse;
extern crate obj_reader;

use argparse::{ArgumentParser, Store};


fn main() {
    let mut file_name = "".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Read an OBJ file.");
        ap.refer(&mut file_name)
            .add_argument("filename", Store,
                          "Path to a Wavefront OBJ file.");
        ap.parse_args_or_exit();
    }
    println!("Reading file: {}", file_name);
    match obj_reader::obj::obj_from_file(file_name) {
        Some(obj)=>
            println!("{:?}", obj),
        None =>
            println!("obj_from_file failed!"),
    }
}
