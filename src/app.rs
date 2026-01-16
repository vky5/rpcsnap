use tonic::Status;

use crate::grpc;
use crate::proto;

pub struct AppConfig {
    pub descriptor_path: String,
    pub addr: String,
    pub service: String,
    pub method: String,
}

pub async fn run(config: AppConfig) -> Result<(), Status> {
    // 1. Load schema
    let services = proto::loader::load(&config.descriptor_path);

    // 2. Find service
    let service = services
        .iter()
        .find(|s| s.name == config.service)
        .ok_or_else(|| Status::not_found("service not found"))?;

    // 3. Find method
    let method = service
        .methods
        .iter()
        .find(|m| m.name == config.method)
        .ok_or_else(|| Status::not_found("method not found"))?;

    // 4. Build request
    let request = proto::loader::build_dynamic_message(&method.input_type);

    // 5. Call gRPC
    let response = grpc::call_unary(
        &config.addr,
        &service.name,
        &method.name,
        request,
        method.output_type.desc.clone(),
    )
    .await?;

    // 6. Print response
    println!("✔ gRPC call succeeded\n");
    println!("Service: {}", service.name);
    println!("Method:  {}\n", method.name);
    println!("Response:");

    proto::model::print_dynamic_message(
        &response,
        &method.output_type.desc,
        2,
    );

    Ok(())
}
