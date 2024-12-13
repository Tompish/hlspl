
use lsp_server::ResponseError;
use lsp_types::{notification::DidOpenTextDocument, DidOpenTextDocumentParams};
use std::error::Error;

pub fn handle_didopen(params: DidOpenTextDocumentParams) -> Result<(), ResponseError>
{
//    let uri = params.text_document.uri;
//    let document_text = params.text_document.text;

    Ok(())
}
