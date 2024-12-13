use std::collections::HashMap;

pub struct DocumentRecord{

    pub documents: HashMap<String, Vec<Row>>
}

pub struct Row{
    pub line: String
}

pub struct SymbolParser{
    functions: Vec<String>,
    local_vars: Vec<String>,
    global_vars: Vec<String>
}
