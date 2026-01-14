#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub methods: Vec<RpcMethod>,
}

#[derive(Debug)]
pub struct RpcMethod {
    pub name: String,
    pub input_type: Message,
    pub output_type: Message,
}

#[derive(Debug)]
pub struct Message{
    pub name: String,
    pub fields: Vec<Field>
}


#[derive(Debug)]
pub struct Field {
    pub kind : FieldKind,
    pub name : String,
    pub repeated: bool
}   

#[derive(Debug)]
pub enum FieldKind{
    String,
    Int32,
    Int64,
    Bool,
    Float,
    Double,
    Message(String), // enum variant with string field. No way connected to struct
}

// cardinality - whether the given field is repeated (list) or not 
// is_list() - checks the cardinality; true = repeated, false - singluar


// okay we would need to make enum for the kind but the problem is that that can be nested. And since that 
// can be nested then the major issue is that during after compilation if it sees a new type that is not in our enum
// it can cause problems. 



/*

syntax = "proto3";

package demo.v1;


service UserService {
  rpc CreateUser (CreateUserRequest) returns (CreateUserResponse);
}

message CreateUserRequest {
  string name = 1;
  int32 age = 2;
  bool is_active = 3;

  Address address = 4;          // message field
  repeated string tags = 5;     // repeated scalar
}

// simple message
message CreateUserResponse {
  User user = 1;
}

// nested message
message User {
  string id = 1;
  string name = 2;
  int32 age = 3;
  repeated Address addresses = 4; // repeated message // repeated means this field is a list of Address message 
}

message Address {
  string city = 1;
  string country = 2;
}


*/