use std::collections::HashMap;
use lsp_types::Position;

pub struct DocumentLibrary{

    pub raw_documents: HashMap<String, String>,
    pub busy_document: Option<String>
}

impl DocumentLibrary{
    pub fn new() -> Self{
        Self{
            raw_documents: HashMap::new(),
            busy_document: None
        }
    }
}

pub struct SymbolShelve{
    documents: HashMap<String, Vec<SymbolBook>>
}

pub struct SymbolBook{
    functions: Vec<Symbol>,
    local_vars: Vec<Symbol>,
    global_vars: Vec<Symbol>
}

pub struct Symbol{
    pub name: String,
    pub position: Position
}
