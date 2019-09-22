//
// Name of Module: winnowdemo
// Description: A demonstration of a state machine for branching text-defined user prompts
// Author: Ben Lovy
// Date: 9/22/2019
// For: SIMC
// Copyright SIMC or Ben Lovy  - All Rights Reserved.
//

use winnow_sm::Nodes;

fn main() {
    let mut nodes = Nodes::new();
    nodes.run();
}
