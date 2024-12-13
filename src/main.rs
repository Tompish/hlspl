use std::error::Error;

use documentlibrary::DocumentLibrary;
use lsp_types::{HoverParams, Hover, HoverContents, MarkedString, HoverProviderCapability, request::HoverRequest};
use lsp_types::{GotoDefinitionParams, request::GotoDefinition, GotoDefinitionResponse};
use lsp_types::{
    request::Request as TypeRequest, Range, Location, Position, InitializeParams, ServerCapabilities, OneOf 
};

use lsp_server::{Connection, ExtractError, Message, Request, Response, ResponseError};
use notifications::handle_notification;

mod notifications;
mod documentlibrary;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {

    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    let mut documents = &Box::new(DocumentLibrary::new());

    eprintln!("starting example main loop");

    for msg in &connection.receiver {
        eprintln!("got msg: {:?}", msg);
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }

                eprintln!("got request: {:?}", req);

                let _ = match handle_request(req, &documents)
                {
                    Ok(res) => connection.sender.send(Message::Response(res)),
                    Err(err) => connection.sender.send(Message::Response(err)),
                };
            }
            Message::Response(resp) => {
                eprintln!("got response: {:?}", resp);
            }
            Message::Notification(not) => {
                let _ = handle_notification(not);
            }
        }
    }
    Ok(())
}

fn handle_request(request: Request, mut documents:  &Box<DocumentLibrary>) -> Result<Response, Response>
{

    let req_id = request.clone().id;

    let response = match SupportedMethods::try_from(request){
        Ok(SupportedMethods::Hover(params)) => handle_hover(params),
        Ok(SupportedMethods::GotoDefinition(params)) => handle_gotodefinition(params),
        Err(_) => Err(ResponseError {
            code: 1,
            message: String::from("hejsan"),
            data: Some(serde_json::Value::String(String::from("hello")))
        })
    };

    match response
    {
        Ok(res) => Ok(
            Response{
                id: req_id,
                result: res,
                error: None
            }),
        Err(err) => Err(
            Response{
                id: req_id,
                result: None,
                error: Some(err)
            })
    }
}

fn handle_hover(params: HoverParams) -> Result<Option<serde_json::Value>, ResponseError>
{
    let pos = params.text_document_position_params;
    let url = pos.text_document.uri;

    let messager = MarkedString::String(String::from("Idiot"));
    let hover = Hover{
        contents: HoverContents::Scalar(messager),
        range: None
    };

    match serde_json::to_value(hover)
    {
        Ok(res) => Ok(Some(res)),
        Err(err) => Err(ResponseError{
            code: 1,
            message: "Det gick trasigt".to_string(),
            data: None
        })
    }

    //Ok(Some(serde_json::Value::String("hej".to_string())))
}

fn handle_gotodefinition(params: GotoDefinitionParams) -> Result<Option<serde_json::Value>, ResponseError>
{
    let pos = params.text_document_position_params;
    let url = pos.text_document.uri;

    let go_pos_start = Position::new(0, 0);
    let go_pos_end = Position::new(0, 4);

    let resp = GotoDefinitionResponse::Scalar(Location{ uri: url, range: Range::new(go_pos_start, go_pos_end)});

    match serde_json::to_value(resp)
    {

        Ok(res) => Ok(Some(res)),
        Err(err) => Err(ResponseError{
            code: 666,
            message: String::from("I suck"),
            data: None
        })
    }
        
}

pub enum SupportedMethods{
    GotoDefinition(GotoDefinitionParams),
    Hover(HoverParams)
}

impl std::convert::TryFrom<Request> for SupportedMethods{
    type Error = ExtractError<Request>;

     fn try_from(value: Request) -> Result<Self, Self::Error> {
         match value.method.as_str()
         {
             GotoDefinition::METHOD => {
                 match value.extract(GotoDefinition::METHOD)
                 {
                    Ok((_, param)) => Ok(SupportedMethods::GotoDefinition(param)),
                     Err(err) => Err(err),
                 }
             },
             HoverRequest::METHOD => {
                 match value.extract(HoverRequest::METHOD)
                 {
                     Ok((_, param)) => Ok(SupportedMethods::Hover(param)),
                     Err(err) => Err(err),
                 }
             },
             _ => Err(ExtractError::MethodMismatch(value)),
         }
     }
}
