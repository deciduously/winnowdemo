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

/// Array type alias for holding question string literals
type QuestionList = Vec<String>;

/// Exit condition
static TERMINATING_NODE: NodeId = 9999;

/// Input file
static INPUT_FILE: &str = "input.txt";

/// Each possible node variant
#[derive(Debug, PartialEq)]
enum NodeType {
    // Question Text, Option One, Option Two
    Branching(String, String, String),
    // List of question states
    Question(QuestionList),
    // Terminating message text
    Terminating(String),
}

/// Node type
#[derive(Debug, PartialEq)]
struct Node {
    /// Node variant
    node_type: NodeType,
    /// ID to transition for option 1
    transition_one: Option<NodeId>,
    /// ID to transition for option 2
    transition_two: Option<NodeId>,
    /// Variable name associated with his node
    variable: Option<String>,
}

impl Node {
    /// Construct a fresh node
    fn new(
        node_type: NodeType,
        transition_one: Option<NodeId>,
        transition_two: Option<NodeId>,
        variable: Option<String>,
    ) -> Self {
        Self {
            node_type,
            transition_one,
            transition_two,
            variable,
        }
    }
}

/// User-defined variables
// Note, HashMap not necessary if you want to stick to an array, just quick to start with
#[derive(Debug, PartialEq)]
struct Env(HashMap<String, String>);

impl Env {
    /// Retrieve the value stored at variable_name.
    fn get_variable(&self, variable_name: &str) -> String {
        match self.0.get(variable_name) {
            Some(v) => v.into(),
            None => variable_name.into(),
        }
    }
    /// Set variable_name to value, always overwrites
    fn set_variable(&mut self, variable_name: &str, value: String) {
        let current = self
            .0
            .entry(variable_name.into())
            .or_insert_with(|| variable_name.into());
        *current = value;
    }
    /// Resolve a string template
    // TODO broken if a variable is at the end of a string, you're being dumb somewhere
    fn resolve_template(&self, template: &str) -> String {
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
struct Nodes {
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
    /// Constructor will parse input.txt file
    fn new() -> Self {
        let mut ret = Nodes::default();
        // read input file
        let mut file_str = String::new();
        let f = File::open(INPUT_FILE).expect("Should open input file");
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

    /// Add a question node to the set
    fn register_question_node(
        &mut self,
        if_answered: NodeId,
        if_terminate: NodeId,
        variable_name: &str,
        questions: QuestionList,
    ) {
        self.nodes.push(Node::new(
            NodeType::Question(questions),
            Some(if_answered),
            Some(if_terminate),
            Some(variable_name.into()),
        ));
    }

    /// Add a branching node to the set
    fn register_branching_node(
        &mut self,
        option_one_dest: NodeId,
        option_two_dest: NodeId,
        variable_name: &str,
        question: &str,
        option_one: &str,
        option_two: &str,
    ) {
        self.nodes.push(Node::new(
            NodeType::Branching(question.into(), option_one.into(), option_two.into()),
            Some(option_one_dest),
            Some(option_two_dest),
            Some(variable_name.into()),
        ));
    }

    /// Add a terminating node to the set
    fn register_terminating_node(&mut self, text: &str) {
        self.nodes.push(Node::new(
            NodeType::Terminating(text.into()),
            None,
            None,
            None,
        ))
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
                // transition1
                let t1 = parse_int_line(inner.next().unwrap());
                // transition2
                let t2 = parse_int_line(inner.next().unwrap());
                // variable name
                let var_name = parse_string_line(inner.next().unwrap());
                // question text
                let question = parse_string_line(inner.next().unwrap());
                // option 1 text
                let o1 = parse_string_line(inner.next().unwrap());
                // option 2 text
                let o2 = parse_string_line(inner.next().unwrap());
                self.register_branching_node(t1, t2, &var_name, &question, &o1, &o2);
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

    /// Execute machine
    fn run(&mut self) {
        while self.current_node != TERMINATING_NODE {
            use NodeType::*;
            let stdin = io::stdin();
            let mut stdout = io::stdout();
            match &self.nodes[self.current_node].node_type {
                Question(qs) => {
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
                                self.state_transition(
                                    self.nodes[self.current_node].transition_one.unwrap(),
                                );
                            }
                        }
                    } else {
                        // Too many blanks!
                        self.state_transition(
                            self.nodes[self.current_node].transition_two.unwrap(),
                        );
                    }
                }
                Branching(_, one, _) => {
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
                            // NOTE - assumes Branching type always only sets variable on input 1, to that string, this is definitely a stand-in
                            match line.as_str() {
                                "1" => {
                                    self.env.set_variable(&self.nodes[self.current_node].variable.as_ref().unwrap(), (*one).clone());
                                    self.state_transition(self.nodes[self.current_node].transition_one.unwrap())},
                                "2" => self.state_transition(self.nodes[self.current_node].transition_two.unwrap()),
                                _ => eprintln!("There's only 1 and 2")
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
            Branching(question, one, two) => write!(
                f,
                "{}\n1. {}\n2. {}\nEnter choice> ",
                self.env.resolve_template(question),
                one,
                two
            ),
            Question(qs) => {
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

fn main() {
    let mut nodes = Nodes::new();
    nodes.run();
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
            2,
            3,
            "QUEST",
            "$NAME, what is your quest?",
            "The Holy Grail",
            "Run and Hide",
        );
        // Node 2
        test.register_branching_node(
            4,
            5,
            "COLOR",
            "$NAME, who seeks $QUEST, what is your favorite color?",
            "Red",
            "I mean blue",
        );
        // Node 3
        test.register_terminating_node(
            "Since you have REFUSED to answer, customer service has been called",
        );
        // Node 4
        test.register_terminating_node(
            "You may pass, $NAME who loves $COLOR, on your noble quest for the $QUEST.",
        );
        // Node 5
        test.register_terminating_node("AAAARRRRGGGGGHHHHH");
        assert_eq!(Nodes::new(), test);
    }
}
