use prost_reflect::DescriptorPool;
use crate::proto::model::{RpcMethod, Service};

pub fn load() -> Vec<Service> {
    let bytes = include_bytes!("../../descriptor.bin");
    let pool = DescriptorPool::decode(&bytes[..]).unwrap();

    let mut services = Vec::new();

    for service in pool.services() {
        let mut methods = Vec::new();

        for method in service.methods() {
            let temp_method = RpcMethod {
                name: method.name().to_string(), // all of them return &str that's why to_string() because all struct only accepts String not &str  
                input_type: method.input().name().to_string(),
                output_type: method.output().name().to_string(),
            };

            methods.push(temp_method);
        }

        let svc = Service {
            name: service.name().to_string(),
            methods,
        };

        services.push(svc);
    }

    return services
}
