use std::error::Error;

use lsp_types::{GotoDefinitionParams, Hover, HoverContents, HoverParams, OneOf};
use lsp_types::{
    request::{Request as TypeRequest, GotoDefinition, HoverRequest}, GotoDefinitionResponse, InitializeParams, ServerCapabilities, HoverProviderCapability
};

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response, ResponseError};

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

    eprintln!("starting example main loop");

    for msg in &connection.receiver {
        eprintln!("got msg: {:?}", msg);
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }

                eprintln!("got request: {:?}", req);

                let _ = match handle_request(req)
                {
                    Ok(res) => connection.sender.send(Message::Response(res)),
                    Err(err) => connection.sender.send(Message::Response(err)),
                };
            }
            Message::Response(resp) => {
                eprintln!("got response: {:?}", resp);
            }
            Message::Notification(not) => {
                eprintln!("got notification: {:?}", not);
            }
        }
    }
    Ok(())
}

fn handle_request(request: Request) -> Result<Response, Response>
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


    Ok(Some(serde_json::Value::String("hej".to_string())))
}

fn handle_gotodefinition(params: GotoDefinitionParams) -> Result<Option<serde_json::Value>, ResponseError>
{
    let pos = params.text_document_position_params;
    let url = pos.text_document.uri;

        
    Ok(Some(serde_json::Value::String("hej".to_string())))
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
