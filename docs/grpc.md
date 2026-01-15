### GRPC
- grpc uses http/2 to make connection 
- usually we convert proto file to the language specific code and it is done by `codegen`
- but here it is not possible and we need to find the structure of request and response from the descriptor that we developed
- 


### Tonic
Tonic is the rust library for managing the http/2 connection making RPC calls and also handling the `codec` process. codec is the process of encoding/decoding the request response into protobuf relevant bytes.


so the output by tonic is understood through dynamicMessage. Since we wont have struct to understand new proto files in compile time because we are not generating the struct files of proto that are given to us because rust is **static complied language** and that struct it wont understand therefore we will be using the stored response message fields to check and match (rust style) the values to the actual outputs. 


