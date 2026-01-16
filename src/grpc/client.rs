use tonic::transport::Channel; // network connection to a gRPC server // after 3 way handshake in http/2 connection is not terminated after complete transmission it remain open until the connection is intentionally closed or one disconnect
use tonic::{Request, Response, Status}; // hold the message body and metadata (headers)
use http::uri::PathAndQuery; // to identify and create the gRPC's http path
use prost_reflect::{DynamicMessage, MessageDescriptor};
use tonic::client::Grpc;
use super::codec::DynamicProstCodec;

// makes the http/2 connection on the given address
async fn make_channel(addr: &str) ->  Result<Channel, tonic::transport::Error> {
    // Channel::from_shared(addr.to_string())
    //     .unwrap()
    //     .connect()
    //     .await
    //     .unwrap()


    let endpoint = Channel::from_shared(addr.to_string()).unwrap(); // this is same as something we are doing with matching... but in case of error it will throw error and panic

    let channel_result = endpoint.connect().await;
    let channel = match channel_result {
        Ok(ch) => ch,
        Err(e)=> return Err(e),
    };

    Ok(channel)
}

// call one unary RPC dynamically
pub async fn call_unary(
    addr: &str, // server address
    service_name: &str, // service name (A service has many methods)
    method_name: &str, // method to be called 
    request_msg: DynamicMessage, // request body that will be encoded by codec in the proper MessageDescriptor format that is stored to us 
    response_descriptor: MessageDescriptor
) -> Result<DynamicMessage, Status> {
    let channel = make_channel(addr).await.unwrap(); // create the channel to that address // think of channel like a pipe 
    let mut grpc = Grpc::new(channel);  // this right here is to give the rules to the pipe how things is going to transfer here 

    grpc.ready().await.unwrap();

    // Build RPC path: /package.Service/Method
    let path = format!("/{}/{}", service_name, method_name); // /demo.v1.PingService/Ping
    let path = PathAndQuery::from_static(Box::leak(path.into_boxed_str())); // ! Improve this later
    // * &static str means that this string's reference will exist from program beginning to program end. 
    // * PathAndQuery is the path that grpc understand on which the request is to be made from_static is the function to create a static str (meaning which will remain in program till end of program) in here 
    /* 
    * path.into_boxed_str() means to convert the String into Box<str> (Heap appocated string)
    * Box::leak() // it forgets who owns that string meaning that references remains forever (memory leak is permanent)
    */


    let request = Request::new(request_msg); // wrap the request message into gRPC format adding some metadata check definition
    let codec = DynamicProstCodec::new(response_descriptor);

    let response: Response<DynamicMessage> = grpc
        .unary(request, path, codec)
        .await
        .unwrap();

    Ok(response.into_inner()) // remove the gRPC envelope and return only message
}

// * DynamicMessage → bytes → gRPC → bytes → DynamicMessage // the grpc part needs connection to be made and this is where everything fits in.

/*
 * HTTP/2 knows only about headers, frames, bytes, streams but not about the protobuf, unary vs streaming, RPC paths status codes
 * gRPC adds extra rules on top of HTTP/2
 * example every request header must have :
 * content-type: application/grpc
 * te: trailers
 * 
 * and every request body must look like this : [compressed?][length][protobuf bytes]
 * so the line in call_unary below channel is to take the raw channel and enforce http/2 in it 
 * 
 */

 /*
 DynamicMessage
   ↓ ProstCodec.encode
protobuf bytes
   ↓ gRPC framing
   ↓ HTTP/2
server
   ↓ HTTP/2
protobuf bytes
   ↑ ProstCodec.decode
DynamicMessage

 */