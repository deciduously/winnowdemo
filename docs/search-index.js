var N=null,E="",T="t",U="u",searchIndex={};
var R=["string","option","usize","result","try_from","try_into","borrow","borrow_mut","type_id","typeid","ordering","nodetype","formatter","NodeType","NodesParser"];
searchIndex["winnow_sm"]={"doc":"winnow_sm This module implements a state machine populated…","i":[[3,"Node","winnow_sm","Node type",N,N],[12,"node_type",E,"Node variant",0,N],[12,"transition_one",E,"ID to transition for option 1",0,N],[12,"transition_two",E,"ID to transition for option 2",0,N],[12,"variable",E,"Variable name associated with his node",0,N],[3,"Env",E,"User-defined variables",N,N],[12,"0",E,E,1,N],[3,R[14],E,E,N,N],[3,"Nodes",E,"Containing structure for all nodes Nodes are registered in…",N,N],[12,"current_node",E,"Current node tracker",2,N],[12,"env",E,"User-defined mapping of names to values",2,N],[12,"internal_state",E,"Internal state tracker",2,N],[12,"nodes",E,"Array (actually variable-sized heap-allocated vector) of…",2,N],[4,R[13],E,"Each possible node variant",N,N],[13,"Branching",E,E,3,N],[13,"Question",E,E,3,N],[13,"Terminating",E,E,3,N],[4,"Rule",E,E,N,N],[13,"EOI",E,E,4,N],[13,"int",E,E,4,N],[13,"digit",E,E,4,N],[13,"int_line",E,E,4,N],[13,"newline",E,E,4,N],[13,"string_line",E,E,4,N],[13,"letter",E,E,4,N],[13,"punctuation",E,E,4,N],[13,R[0],E,E,4,N],[13,"branching",E,E,4,N],[13,"branching_id",E,E,4,N],[13,"question",E,E,4,N],[13,"question_id",E,E,4,N],[13,"terminating",E,E,4,N],[13,"terminating_id",E,E,4,N],[13,"node",E,E,4,N],[13,"nodes",E,E,4,N],[5,"trim_whitespace",E,"Trim either \\n or \\r\\n from a String",N,[[["str"]],[R[0]]]],[5,"parse_string_line",E,"helper function to parse string_line rule",N,[[["rule"],["pair",["rule"]]],[R[0]]]],[5,"parse_int_line",E,"helper function to parse int_line rule",N,[[["rule"],["pair",["rule"]]],[R[2]]]],[6,"NodeId",E,"Unique node identifier type",N,N],[6,"QuestionList",E,"Array type alias for holding question string literals",N,N],[7,"TERMINATING_NODE",E,"Exit condition",N,N],[7,"DEFAULT_INPUT_FILE",E,"Input file",N,N],[17,"_PEST_GRAMMAR_NodesParser",E,E,N,N],[11,"new",E,"Construct a fresh node",0,[[[R[11]],[R[2]],[R[0]],[R[1],[R[2]]],[R[1],[R[0]]]],["self"]]],[11,"get_variable",E,"Retrieve the value stored at variable_name.",1,[[["self"],["str"]],[R[0]]]],[11,"set_variable",E,"Set variable_name to value, always overwrites",1,[[["self"],[R[0]],["str"]]]],[11,"resolve_template",E,"Resolve a string template # Example",1,[[["self"],["str"]],[R[0]]]],[11,"new",E,"Constructor will parse specified file or `input.txt` if None",2,[[[R[0]],[R[1],[R[0]]]],["self"]]],[11,"run",E,"Execute machine",2,[[["self"]]]],[11,"register_question_node",E,"Add a question node to the set",2,[[["self"],[R[2]],["vec",[R[0]]],[R[0]],["str"]]]],[11,"register_branching_node",E,"Add a branching node to the set",2,[[["self"],[R[2]],["str"]]]],[11,"register_terminating_node",E,"Add a terminating node to the set",2,[[["self"],["str"]]]],[11,"read_and_register",E,"Catch-all to register a parsed node",2,[[["self"],["rule"],["pair",["rule"]]]]],[11,"state_transition",E,"State transition",2,[[["self"],[R[2]]]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[],[U]]],[11,R[4],E,E,0,[[[U]],[R[3]]]],[11,R[5],E,E,0,[[],[R[3]]]],[11,R[6],E,E,0,[[["self"]],[T]]],[11,R[7],E,E,0,[[["self"]],[T]]],[11,R[8],E,E,0,[[["self"]],[R[9]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,"into",E,E,1,[[],[U]]],[11,R[4],E,E,1,[[[U]],[R[3]]]],[11,R[5],E,E,1,[[],[R[3]]]],[11,R[6],E,E,1,[[["self"]],[T]]],[11,R[7],E,E,1,[[["self"]],[T]]],[11,R[8],E,E,1,[[["self"]],[R[9]]]],[11,"from",E,E,5,[[[T]],[T]]],[11,"into",E,E,5,[[],[U]]],[11,R[4],E,E,5,[[[U]],[R[3]]]],[11,R[5],E,E,5,[[],[R[3]]]],[11,R[6],E,E,5,[[["self"]],[T]]],[11,R[7],E,E,5,[[["self"]],[T]]],[11,R[8],E,E,5,[[["self"]],[R[9]]]],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[],[U]]],[11,"to_string",E,E,2,[[["self"]],[R[0]]]],[11,R[4],E,E,2,[[[U]],[R[3]]]],[11,R[5],E,E,2,[[],[R[3]]]],[11,R[6],E,E,2,[[["self"]],[T]]],[11,R[7],E,E,2,[[["self"]],[T]]],[11,R[8],E,E,2,[[["self"]],[R[9]]]],[11,"from",E,E,3,[[[T]],[T]]],[11,"into",E,E,3,[[],[U]]],[11,R[4],E,E,3,[[[U]],[R[3]]]],[11,R[5],E,E,3,[[],[R[3]]]],[11,R[6],E,E,3,[[["self"]],[T]]],[11,R[7],E,E,3,[[["self"]],[T]]],[11,R[8],E,E,3,[[["self"]],[R[9]]]],[11,"to_owned",E,E,4,[[["self"]],[T]]],[11,"clone_into",E,E,4,[[["self"],[T]]]],[11,"from",E,E,4,[[[T]],[T]]],[11,"into",E,E,4,[[],[U]]],[11,R[4],E,E,4,[[[U]],[R[3]]]],[11,R[5],E,E,4,[[],[R[3]]]],[11,R[6],E,E,4,[[["self"]],[T]]],[11,R[7],E,E,4,[[["self"]],[T]]],[11,R[8],E,E,4,[[["self"]],[R[9]]]],[11,"default",E,E,1,[[],["self"]]],[11,"default",E,E,2,[[],["nodes"]]],[11,"assert_receiver_is_total_eq",E,E,4,[[["self"]]]],[11,"clone",E,E,4,[[["self"]],["rule"]]],[11,"partial_cmp",E,E,4,[[["self"],["rule"]],[[R[1],[R[10]]],[R[10]]]]],[11,"eq",E,E,3,[[["self"],[R[11]]],["bool"]]],[11,"ne",E,E,3,[[["self"],[R[11]]],["bool"]]],[11,"eq",E,E,0,[[["self"],["node"]],["bool"]]],[11,"ne",E,E,0,[[["self"],["node"]],["bool"]]],[11,"eq",E,E,1,[[["self"],["env"]],["bool"]]],[11,"ne",E,E,1,[[["self"],["env"]],["bool"]]],[11,"eq",E,E,4,[[["self"],["rule"]],["bool"]]],[11,"eq",E,E,2,[[["self"],["nodes"]],["bool"]]],[11,"ne",E,E,2,[[["self"],["nodes"]],["bool"]]],[11,"cmp",E,E,4,[[["self"],["rule"]],[R[10]]]],[11,"hash",E,E,4,[[["self"],["__h"]]]],[11,"fmt",E,E,2,[[["self"],[R[12]]],[R[3]]]],[11,"fmt",E,E,3,[[["self"],[R[12]]],[R[3]]]],[11,"fmt",E,E,0,[[["self"],[R[12]]],[R[3]]]],[11,"fmt",E,E,1,[[["self"],[R[12]]],[R[3]]]],[11,"fmt",E,E,4,[[["self"],[R[12]]],[R[3]]]],[11,"fmt",E,E,2,[[["self"],[R[12]]],[R[3]]]],[11,"parse",E,E,5,[[["rule"],["str"]],[["pairs",["rule"]],["error",["rule"]],[R[3],["pairs","error"]]]]]],"p":[[3,"Node"],[3,"Env"],[3,"Nodes"],[4,R[13]],[4,"Rule"],[3,R[14]]]};
searchIndex["winnowdemo"]={"doc":E,"i":[[5,"format_authors","winnowdemo",E,N,[[["str"]],[R[0]]]],[5,"get_input_file_arg",E,"Return the first command line argument, if present",N,[[],[[R[0]],[R[1],[R[0]]]]]],[5,"main",E,E,N,[[]]],[17,"VERSION",E,E,N,N],[17,"AUTHORS",E,E,N,N]],"p":[]};
initSearch(searchIndex);addSearchOptions(searchIndex);