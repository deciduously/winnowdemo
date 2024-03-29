//
// Name of Module: winnow_sm
// Description: State machine library for branching text-defined user prompts
// Author: Ben Lovy
// Date: 9/22/2019
// For: SIMC
// Copyright SIMC - All Rights Reserved.
//

//! # winnow_sm
//! This module implements a state machine populated from a text file.
//! Library consumer can specify input file, or default to `<rootdir>/input.txt`.
//!
//! ## Input file
//!
//! This file contains nodes in one of [three types](https://deciduously.github.io/winnowdemo/winnow_sm/enum.NodeType.html): Question, Branching, or Terminating.
//! Note - comments here are for demonstration only and are not (yet) supported.
//! The `Question` type prompts for a string input, and can accept zero or more responses if prompt left blank:
//! ```txt
//! 1 // Node type must be 1
//! 1 // Node to jump to on successful input, 0-indexed
//! 3 // Node to jump to after exhausting fail prompts
//! NAME // Name of the variable to associate user's input with
//! What is your name? // First prompt
//! Please tell me your name // Second prompt
//! You better tell me your name // Third prompt
//! ```
//! The `Branching` type gives the user a series of options, each with a destination if selected:
//! ```txt
//! 2 // Node type must be 2
//! QUEST // Name of the variable associated with this node
//! $NAME, what is your quest? // Question prompt
//! The Holy Grail:2 // Option One text:Option 1 destination
//! Run and Hide:3 // Option Two text:Option 2 destination
//! ```
//! The `Terminating` type simply displays a message and signals execution should end:
//! ```txt
//! 3 // Node type must be 3
//! You may pass, $NAME who loves $COLOR, on your noble quest for the $QUEST.  // Exit message
//! ```
//!
//! Nodes are added and assigned IDs in the order they appear in the input, beginning with 0.
//!
//! The string prompts perform simple variable expansion.
//! You can refer to any variable previously defined in the file by prefixing it with a `$`, as in the examples.
//! If a lookup fails, the name of the variable in the template will be used instead, without the leading `$`.

#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
};

/// Trim either \n or \r\n from a String
fn trim_whitespace(s: &str) -> String {
    let mut ret = String::new();

    for c in s.chars() {
        if c != '\r' && c != '\n' {
            ret.push(c);
        }
    }
    ret
}

/// Unique node identifier type
// usize is the platform-dependent pointer-sized unsigned integer type
// e.g. on 64 bit platform this is 8 bytes
type NodeId = usize;

/// Exit condition
static TERMINATING_NODE: NodeId = 9999;

/// Input file
static DEFAULT_INPUT_FILE: &str = "input.txt";

/// A single branch option (prompt, destination)
type BranchOption = (String, NodeId);

/// Each possible node variant
#[derive(Debug, PartialEq)]
enum NodeType {
    // Question Text, Options
    Branching(String, Vec<BranchOption>),
    // Success destination, fail destination, list of question states
    Question(NodeId, NodeId, Vec<String>),
    // Terminating message text
    Terminating(String),
}

/// Node type
#[derive(Debug, PartialEq)]
struct Node {
    /// Node variant
    node_type: NodeType,
    /// Variable name associated with his node
    variable: Option<String>,
}

impl Node {
    /// Construct a fresh node
    fn new(node_type: NodeType, variable: Option<String>) -> Self {
        Self {
            node_type,
            variable,
        }
    }
}

/// User-defined variables
#[derive(Debug, PartialEq)]
pub struct Env(HashMap<String, String>);

impl Env {
    /// Retrieve the value stored at variable_name.
    pub fn get_variable(&self, variable_name: &str) -> String {
        match self.0.get(variable_name) {
            Some(v) => v.into(),
            None => variable_name.into(),
        }
    }
    /// Set variable_name to value, always overwrites
    pub fn set_variable(&mut self, variable_name: &str, value: String) {
        let current = self
            .0
            .entry(variable_name.into())
            .or_insert_with(|| variable_name.into());
        *current = value;
    }
    /// Resolve a string template
    /// # Example
    ///
    /// ```
    /// # use winnow_sm::Env;
    /// # let mut env = Env::default();
    /// env.set_variable("NAME", "Al Gore".into());
    /// env.set_variable("COLOR", "Green".into());
    /// let template = "Hi there, $NAME, seen anything $COLOR lately?";
    /// let expected = "Hi there, Al Gore, seen anything Green lately?";
    /// # assert_eq!(env.resolve_template(template), expected.to_string());
    /// ```
    ///
    /// If variable is not set, variable name will be used:
    ///
    /// ```
    /// # use winnow_sm::Env;
    /// # let mut env = Env::default();
    /// # env.set_variable("NAME", "Al Gore".into());
    /// # env.set_variable("COLOR", "Green".into());
    /// let template = "Hi there, $NAME, how is $TOPIC?";
    /// let expected = "Hi there, Al Gore, how is TOPIC?";
    /// # assert_eq!(env.resolve_template(template), expected.to_string());
    /// ```
    ///
    // TODO broken if a variable is at the end of a string, you're being dumb somewhere
    pub fn resolve_template(&self, template: &str) -> String {
        // Init return string
        let mut ret = String::new();
        // Operate over characters as u8 byte array
        let vec = template.as_bytes().to_owned();
        let mut i = 0;
        while i < vec.len() {
            if vec[i] == b'$' {
                // we hit a variable
                // find the rest of the word
                let mut var_name = String::new();
                // Skip the $
                i += 1;
                // Read until encounter a non-capital letter or end of vec
                while vec[i] >= b'A' && vec[i] <= b'Z' && i < vec.len() - 1 {
                    var_name.push(vec[i] as char);
                    i += 1;
                }
                ret.push_str(&self.get_variable(&var_name));
            } else {
                ret.push(vec[i] as char);
                i += 1;
            }
        }
        ret
    }
}

impl Default for Env {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Parser)]
#[grammar = "nodes.pest"]
pub struct NodesParser;

/// helper function to parse branch_option rule
fn parse_branch_option_line(parsed: Pair<Rule>) -> BranchOption {
    match parsed.as_rule() {
        Rule::branch_option => {
            let mut inner = parsed.into_inner();
            let prompt = inner.next().unwrap();
            // skip delimiter
            inner.next();
            let destination = inner.next().unwrap();
            (
                prompt.as_str().into(),
                destination
                    .as_str()
                    .parse::<usize>()
                    .expect(&format!("Fail to parse {}", destination.as_str())),
            )
        }
        _ => panic!("Called parse_branch_option_line on the wrong rule"),
    }
}

/// helper function to parse string_line rule
fn parse_string_line(parsed: Pair<Rule>) -> String {
    match parsed.as_rule() {
        Rule::string_line => trim_whitespace(parsed.as_str()),
        _ => panic!("Called parse_string_line on the wrong rule"),
    }
}

/// helper function to parse int_line rule
fn parse_int_line(parsed: Pair<Rule>) -> usize {
    match parsed.as_rule() {
        Rule::int_line => parsed
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap(),
        _ => panic!("Called parse_int_line on the wrong rule"),
    }
}

/// Containing structure for all nodes
/// Nodes are registered in sequential order
#[derive(Debug, Default, PartialEq)]
pub struct Nodes {
    /// Current node tracker
    current_node: NodeId,
    /// User-defined mapping of names to values
    env: Env,
    /// Internal state tracker
    internal_state: NodeId,
    /// Array (actually variable-sized heap-allocated vector) of nodes
    nodes: Vec<Node>,
}

impl Nodes {
    /// Constructor will parse specified file or `input.txt` if None
    pub fn new(specified_input: Option<String>) -> Self {
        let input_file = specified_input.unwrap_or_else(|| DEFAULT_INPUT_FILE.into());
        println!("Input file: {}\n", input_file);
        let mut ret = Nodes::default();
        // read input file
        let mut file_str = String::new();
        let f = File::open(input_file).expect("Should open input file");
        let mut bfr = BufReader::new(f);
        bfr.read_to_string(&mut file_str)
            .expect("Should read input file");
        let mut parsed = match NodesParser::parse(Rule::nodes, &file_str) {
            Ok(parse_tree) => parse_tree,
            Err(e) => panic!(format!("{}", e)),
        };
        ret.read_and_register(parsed.next().unwrap());
        ret
    }

    /// Execute machine
    pub fn run(&mut self) {
        while self.current_node != TERMINATING_NODE {
            use NodeType::*;
            let stdin = io::stdin();
            let mut stdout = io::stdout();
            match &self.nodes[self.current_node].node_type {
                Question(success, fail, qs) => {
                    // if we haven't run out of prompts
                    if self.internal_state < qs.len() {
                        // TODO Stand-in!
                        // Display prompt
                        print!("{}", self);
                        stdout.flush().expect("Should flush stdout");
                        // Get string
                        let mut line = String::new();
                        stdin.lock().read_line(&mut line).unwrap();
                        // Truncate newline
                        line = trim_whitespace(&line);
                        match line.len() {
                            0 => {
                                // Empty input - just a newline
                                self.internal_state += 1;
                            }
                            _ => {
                                // Store anything else
                                // Unwraps are safe - we already know its a Question
                                // Set value
                                self.env.set_variable(
                                    &self.nodes[self.current_node].variable.as_ref().unwrap(),
                                    line,
                                );
                                let destination = *success;
                                self.state_transition(destination);
                            }
                        }
                    } else {
                        // Too many blanks!
                        let destination = *fail;
                        self.state_transition(destination);
                    }
                }
                Branching(_, options) => {
                    // TODO stand-in!
                    // Display prompt
                    print!("{}", self);
                    stdout.flush().expect("Should flush stdout");
                    // Get string
                    let mut line = String::new();
                    stdin.lock().read_line(&mut line).unwrap();
                    // truncate newline
                    line = trim_whitespace(&line);
                    match line.len() {
                        0 => {
                            eprintln!("TODO - The graphical option won't allow empty input, so just comply please")
                        },
                        _ => {
                            // Check if its a valid option, if so, return options[choice-1], if not, yell
                            let choice = line.as_str().parse::<usize>();
                            match choice {
                                Ok(n) => {
                                    if n > options.len() {
                                        eprintln!("Not a valid option!");
                                    } else {
                                        // Set variable to option text
                                        self.env.set_variable(&self.nodes[self.current_node].variable.as_ref().unwrap(), options[n - 1].0.clone());
                                        // Transition to destination
                                        let destination = options[n - 1].1;
                                        self.state_transition(destination);
                                    }
                                }
                                Err(e) => eprintln!("Unrecognized input: {}", e),
                            }
                        },
                    }
                }
                Terminating(_) => {
                    // TODO stand-in!
                    // Display
                    println!("{}", self);
                    // Wait for exit
                    let _ = stdin.lock().lines().next();
                    self.state_transition(TERMINATING_NODE);
                }
            }
            // Print padding line
            println!();
        }
    }

    /// Add a question node to the set
    fn register_question_node(
        &mut self,
        if_answered: NodeId,
        if_terminate: NodeId,
        variable_name: &str,
        questions: Vec<String>,
    ) {
        self.nodes.push(Node::new(
            NodeType::Question(if_answered, if_terminate, questions),
            Some(variable_name.into()),
        ));
    }

    /// Add a branching node to the set
    fn register_branching_node(
        &mut self,
        variable_name: &str,
        question: &str,
        options: Vec<BranchOption>,
    ) {
        self.nodes.push(Node::new(
            NodeType::Branching(question.into(), options),
            Some(variable_name.into()),
        ));
    }

    /// Add a terminating node to the set
    fn register_terminating_node(&mut self, text: &str) {
        self.nodes
            .push(Node::new(NodeType::Terminating(text.into()), None))
    }

    /// Catch-all to register a parsed node
    fn read_and_register(&mut self, parsed: Pair<Rule>) {
        match parsed.as_rule() {
            Rule::nodes => {
                for child in parsed.into_inner() {
                    // each should be an expr, grab the actual node type and register it
                    self.read_and_register(child);
                }
            }
            Rule::node => self.read_and_register(parsed.into_inner().next().unwrap()),
            Rule::question => {
                // skip type
                let mut inner = parsed.into_inner().skip(1);
                // transition1
                let t1 = parse_int_line(inner.next().unwrap());
                // transition2
                let t2 = parse_int_line(inner.next().unwrap());
                // variable name
                let var_name = parse_string_line(inner.next().unwrap());
                // zero or more questions on stringlines
                let mut questions = Vec::new();
                for qline in inner {
                    questions.push(parse_string_line(qline));
                }
                self.register_question_node(t1, t2, &var_name, questions);
            }
            Rule::branching => {
                // skip type
                let mut inner = parsed.into_inner().skip(1);
                // variable name
                let var_name = parse_string_line(inner.next().unwrap());
                // question text
                let question = parse_string_line(inner.next().unwrap());
                // options
                let mut options = Vec::new();
                for oline in inner {
                    options.push(parse_branch_option_line(oline));
                }
                self.register_branching_node(&var_name, &question, options);
            }
            Rule::terminating => {
                // skip type
                let mut inner = parsed.into_inner().skip(1);
                // Terminate message
                let message = parse_string_line(inner.next().unwrap());
                self.register_terminating_node(&message);
            }
            Rule::EOI => {}
            _ => panic!(format!("Cannot handle {:?}", parsed.as_rule())),
        }
    }

    /// State transition
    fn state_transition(&mut self, new_state: NodeId) {
        self.current_node = new_state;
        self.internal_state = 0;
    }
}

impl fmt::Display for Nodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NodeType::*;
        match &self.nodes[self.current_node].node_type {
            Branching(question, options) => {
                let mut option_string = String::new();
                for (i, option) in options.iter().enumerate() {
                    option_string.push_str(&format!("{}. {}\n", i + 1, option.0));
                }
                write!(
                    f,
                    "{}\n{}\nEnter choice> ",
                    self.env.resolve_template(question),
                    option_string
                )
            }
            Question(_, _, qs) => {
                if self.internal_state > qs.len() {
                    // this situation should have been caught by the caller
                    unreachable!()
                }
                write!(
                    f,
                    "{}\nEnter string> ",
                    self.env.resolve_template(&qs[self.internal_state])
                )
            }
            Terminating(message) => write!(
                f,
                "{}\nGoodbye (enter anything to exit)> ",
                self.env.resolve_template(message)
            ),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_input() {
        use super::Nodes;
        use pretty_assertions::assert_eq;
        let mut test = Nodes::default();
        // Node 0
        test.register_question_node(
            1,
            3,
            "NAME",
            vec![
                "What is your name?".into(),
                "Please tell me your name".into(),
                "You better tell me your name".into(),
            ],
        );
        // Node 1
        test.register_branching_node(
            "QUEST",
            "$NAME, what is your quest?",
            vec![("The Holy Grail".into(), 2), ("Run and Hide".into(), 3)],
        );
        // Node 2
        test.register_branching_node(
            "COLOR",
            "$NAME, who seeks $QUEST, what is your favorite color?",
            vec![("Red".into(), 4), ("I mean blue".into(), 5)],
        );
        // Node 3
        test.register_terminating_node(
            "Since you have REFUSED to answer, The Black Night has been called",
        );
        // Node 4
        test.register_terminating_node(
            "You may pass, $NAME who loves $COLOR, on your noble quest for the $QUEST.",
        );
        // Node 5
        test.register_terminating_node("AAAARRRRGGGGGHHHHH");
        assert_eq!(Nodes::new(None), test);
    }
}
