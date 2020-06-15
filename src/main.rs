use std::fs;
use tonic::{transport::Server, Request, Response, Status};
use log::{debug, info};
use yaml_rust::YamlLoader;

use commands::commander_server::{Commander, CommanderServer};
use commands::{CommandRequest, CommandResponse};

pub mod commands {
    tonic::include_proto!("commands");
}

#[derive(Debug, Default)]
pub struct BridgeCommander {}

#[tonic::async_trait]
impl Commander for BridgeCommander {
    async fn send(
        &self,
        request: Request<CommandRequest>
    ) -> Result<Response<CommandResponse>, Status> {
        debug!("Got a request: {:?}", request);
        
        let message = request.into_inner();

        let reply = CommandResponse {
            client_id: message.client_id,
            command: message.command,
            args: message.args,
            message: format!("Executed {}!", message.command).into(), 
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_contents = fs::read_to_string("./config.yaml").expect("Cannot read config.yaml");
    let configs = YamlLoader::load_from_str(&config_contents)?;
    let config = &configs[0];
    
    let server_port = config["server"]["port"].as_i64().unwrap();

    let server_address: std::string::String = format!("[::1]:{port}", port=server_port).into();
    let addr = server_address.parse()?;
    let commander = BridgeCommander::default();

    info!("Server listening on {}", addr);

    Server::builder()
        .add_service(CommanderServer::new(commander))
        .serve(addr)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {

}
