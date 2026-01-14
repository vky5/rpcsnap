use prost_reflect::{DescriptorPool, Kind};
use crate::proto::model::{Message, RpcMethod, Service, FieldKind, Field};

pub fn load() -> Vec<Service> {
    let bytes = include_bytes!("../../descriptor.bin");
    let pool = DescriptorPool::decode(&bytes[..]).unwrap();

    let mut services = Vec::new();

    for service in pool.services() {
        let mut methods = Vec::new();

        for method in service.methods() {

            let input_msg = build_message(method.input());
            let output_msg = build_message(method.output());


            let temp_method = RpcMethod {
                name: method.name().to_string(), // all of them return &str that's why to_string() because all struct only accepts String not &str  
                input_type: input_msg,
                output_type: output_msg,
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


fn build_message(msg: prost_reflect::MessageDescriptor) -> Message {
    let mut fields = Vec::new();

    for field in msg.fields(){
        let kind = match field.kind() { // this returns the kind of field
            Kind::String => FieldKind::String,
            Kind::Bool => FieldKind::Bool,
            Kind::Int32 => FieldKind::Int32,
            Kind::Int64 => FieldKind::Int64,
            Kind::Double => FieldKind::Double,
            Kind::Float => FieldKind::Float,

            Kind::Message(m)=> FieldKind::Message(m.name().to_string()),

            _ => continue,

        };

        fields.push(Field{
            name: field.name().to_string(),
            kind,
            repeated: field.is_list(),
        });
    }

    Message { name: msg.name().to_string(), fields }
}