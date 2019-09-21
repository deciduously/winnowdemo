//use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fmt,
    io::{self, BufRead, Write},
};

/// Unique node identifier type
// usize is the platform-dependent pointer-sized unsigned integer type
// e.g. on 64 bit platform this is 8 bytes
type NodeId = usize;

/// Maximum number of questions to hold
const MAX_QUESTION_STATES: usize = 3;

/// Array types alias for holding question string literals
type QuestionList = [&'static str; MAX_QUESTION_STATES];

/// Exit condition
static TERMINATING_NODE: NodeId = 9999;

/// Each possible node variant
#[derive(Debug, Clone)]
enum NodeType {
    // Question Text, Option One, Option Two
    Branching(&'static str, &'static str, &'static str),
    // List of question states
    Question([&'static str; MAX_QUESTION_STATES]),
    // Terminating message text
    Terminating(&'static str),
}

/// Node type
#[derive(Debug)]
struct Node {
    /// Unique ID - position in Nodes array
    id: NodeId,
    /// Node variant
    node_type: NodeType,
    /// ID to transition for option 1
    transition_one: Option<NodeId>,
    /// ID to transition for option 2
    transition_two: Option<NodeId>,
    /// Variable name associated with his node
    variable: Option<&'static str>,
}

impl Node {
    /// Construct a fresh node
    fn new(
        id: NodeId,
        node_type: NodeType,
        transition_one: Option<NodeId>,
        transition_two: Option<NodeId>,
        variable: Option<&'static str>,
    ) -> Self {
        Self {
            id,
            node_type,
            transition_one,
            transition_two,
            variable,
        }
    }
}

/// User-defined variables
// Note, HashMap not necessary if you want to stick to an array, just quick to start with
#[derive(Debug)]
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
        // Just pass it through
        //template.into()
    }
}

impl Default for Env {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

/// Containing structure for all nodes
/// Nodes are registered in sequential order
#[derive(Debug, Default)]
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
    /// Get next NodeId
    fn next_id(&self) -> NodeId {
        self.nodes.len()
    }

    /// Add a question node to the set
    fn register_question_node(
        &mut self,
        if_answered: NodeId,
        if_terminate: NodeId,
        variable_name: &'static str,
        questions: QuestionList,
    ) {
        // Push node
        self.nodes.push(Node::new(
            self.next_id(),
            NodeType::Question(questions),
            Some(if_answered),
            Some(if_terminate),
            Some(variable_name),
        ));
    }

    /// Add a branching node to the set
    fn register_branching_node(
        &mut self,
        option_one_dest: NodeId,
        option_two_dest: NodeId,
        variable_name: &'static str,
        question: &'static str,
        option_one: &'static str,
        option_two: &'static str,
    ) {
        // Push node
        self.nodes.push(Node::new(
            self.next_id(),
            NodeType::Branching(question, option_one, option_two),
            Some(option_one_dest),
            Some(option_two_dest),
            Some(variable_name),
        ));
    }

    /// Add a terminating node to the set
    fn register_terminating_node(&mut self, text: &'static str) {
        // Just push node, no variable
        self.nodes.push(Node::new(
            self.next_id(),
            NodeType::Terminating(text),
            None,
            None,
            None,
        ))
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
                        line = line[..line.len() - 1].into();
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
                                    self.nodes[self.current_node].variable.unwrap(),
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
                    line = line[..line.len() - 1].into();
                    match line.len() {
                        0 => {
                            eprintln!("TODO - The graphical option won't allow empty input, so just comply please")
                        },
                        _ => {
                            // NOTE - assumes Branching type always only sets variable on input 1, to that string, this is definitely a stand-in
                            match line.as_str() {
                                "1" => {
                                    self.env.set_variable(self.nodes[self.current_node].variable.unwrap(), String::from(*one));
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

/// Stand-in initializer that registers the examples from the spec
fn init_example_nodes() -> Nodes {
    let mut ret = Nodes::default();
    // Node 0
    ret.register_question_node(
        1,
        3,
        "NAME",
        [
            "What is your name?",
            "Please tell me your name",
            "You better tell me your name",
        ],
    );
    // Node 1
    ret.register_branching_node(
        2,
        3,
        "QUEST",
        "$NAME, what is your quest?",
        "The Holy Grail",
        "Run and Hide",
    );
    // Node 2
    ret.register_branching_node(
        4,
        5,
        "COLOR",
        "$NAME, who seeks $QUEST, what is your favorite color?",
        "Red",
        "I mean blue",
    );
    // Node 3
    ret.register_terminating_node(
        "Since you have REFUSED to answer, customer service has been called",
    );
    // Node 4
    ret.register_terminating_node(
        "You may pass, $NAME who loves $COLOR, on your noble quest for the $QUEST.",
    );
    // Node 5
    ret.register_terminating_node("AAAARRRRGGGGGHHHHH");
    // Return completed node structure
    ret
}

fn main() {
    let mut nodes = init_example_nodes();
    nodes.run();
}
