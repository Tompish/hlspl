pub mod didopen;

use didopen::handle_didopen;
//use lsp_types::notification::*;
use lsp_server::{ExtractError, Notification, ResponseError};
use lsp_types::{notification::{DidChangeTextDocument, DidOpenTextDocument, Notification as TypeNotification}, request::Request, DidChangeTextDocumentParams, DidOpenTextDocumentParams};

pub fn handle_notification(note: Notification) -> Result<(), ResponseError>
{
    match SupportedNotifications::try_from(note)
    {
        Ok(SupportedNotifications::DidOpenTextdocument(params)) => handle_didopen(params),
        _ => panic!("Did not handle notification")
    }
}

pub enum SupportedNotifications{
    DidOpenTextdocument(DidOpenTextDocumentParams),
    DidChangeTextdocument(DidChangeTextDocumentParams)
}

impl std::convert::TryFrom<Notification> for SupportedNotifications{
    type Error = ExtractError<Notification>;

    fn try_from(value: Notification) -> Result<Self, Self::Error> {

        match value.method.as_str()
        {
            DidOpenTextDocument::METHOD => {
                match value.extract(DidOpenTextDocument::METHOD){
                    Ok(params) => Ok(SupportedNotifications::DidOpenTextdocument(params)),
                    Err(err) => Err(err)
                }
            }
            DidChangeTextDocument::METHOD => {
                match value.extract(DidChangeTextDocument::METHOD){
                    Ok(params) => Ok(SupportedNotifications::DidChangeTextdocument(params)),
                    Err(err) => Err(err)
                }
            }
            _ => Err(ExtractError::MethodMismatch(value))
        }
    }
}
