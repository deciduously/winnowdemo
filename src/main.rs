//
// Name of Module: winnowdemo
// Description: A demonstration of a state machine for branching text-defined user prompts
// Author: Ben Lovy
// Date: 9/22/2019
// For: SIMC
// Copyright SIMC or Ben Lovy  - All Rights Reserved.
//

use winnow_sm::Nodes;

// Grab package metadata
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn format_authors(s: &str) -> String {
    let mut ret = String::new();
    let names: Vec<&str> = s.split(':').collect();
    for i in 0..names.len() {
        ret.push_str(names[i]);
        if i < names.len() - 1 {
            ret.push_str(", ");
        }
    }
    ret
}

/// Return the first command line argument, if present
fn get_input_file_arg() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0 => unreachable!(), // will always contain name of executable
        1 => None,
        _ => Some(args[1].clone()),
    }
}

fn main() {
    // Display preamble
    println!(
        "Winnow Automation Demonstration {}\n{}\n\n",
        VERSION,
        format_authors(AUTHORS)
    );
    // Init nodes
    let mut nodes = Nodes::new(get_input_file_arg());
    // Run machine
    nodes.run();
}
