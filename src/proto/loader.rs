use prost_reflect::{DescriptorPool, Kind, DynamicMessage, Value};
use crate::proto::model::{Message, RpcMethod, Service, FieldKind, Field};

pub fn load() -> Vec<Service> {
    let bytes = include_bytes!("../../descriptor.bin");
    let pool = DescriptorPool::decode(&bytes[..]).unwrap(); // Descriptorpool is all the descriptors from the descriptor bin that was created 

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


// Get the values of the schema of the message
pub fn build_dynamic_message(msg_schema: &Message, pool: &DescriptorPool) -> DynamicMessage {
    let descriptor = pool.get_message_by_name(&msg_schema.name).expect("Message not found in descriptor pool"); // getting the MessageDescriptor from the MessageSchema return type is option
    let mut message = DynamicMessage::new(descriptor.clone()); // Create an empty protobuf message value whose shape is defined by this schema // Since we are mutating it that's why it asked for descriptor.clone otherwise for non mutable w could have passed borrowed value

    for field in &msg_schema.fields{
        let field_desc = descriptor.get_field_by_name(&field.name).expect("Field not found");

        let value  = match &field.kind {
            FieldKind::String => Value::String("test".to_string()),
            FieldKind::Bool => Value::Bool(true),
            FieldKind::Int32=> Value::I32(43),
            FieldKind::Int64=> Value::I64(13),
            FieldKind::Float => Value::F32(13.21),
            FieldKind::Double => Value::F32(22.21),

            FieldKind::Message(msg_name)=> {
                let nested_schema = Message {
                    name: msg_name.clone(),
                    fields: Vec::new(),
                };

                let nested_msg = build_dynamic_message(&nested_schema, pool);
                Value::Message(nested_msg)
            }

        };

        let final_value = match field.repeated {
            true => Value::List(vec![value]),
            false => value
        };

        message.set_field(&field_desc, final_value);
    }


    message
}


/*
Pool - a readonly db of all the protobuf schemas loaded in descriptor.bin
Schema - describes what is allowed (structure from MessageDescriptor)
DynamicMessage - actual values of schema (messageDescriptor)

Conceptually for a file descriptorpool has something like this
DescriptorPool
 ├── demo.v1.CreateUserRequest
 ├── demo.v1.CreateUserResponse
 ├── demo.v1.User
 └── demo.v1.Address


we are not converting the proto fils into rust struct because there is no point 
everything is to be done in runtime. SO how will tonic make connection and since 
we dont have struct for response because that is in proto file and no generated 
file for rust is created how will wwe able to understand the structure of it? 
using field descriptor just like how we did in build message

*/