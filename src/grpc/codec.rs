// @ Codec that can encode/decode protobuf messages runtime

use tonic::codec::{Codec, Decoder, Encoder, DecodeBuf, EncodeBuf};
use tonic::Status; // error that happened inside the codec can be converted into proper gRPC error and sent back to caller in standardized way

use prost_reflect::{DynamicMessage, MessageDescriptor};
use prost_reflect::prost::Message;

use bytes::Buf;

/// gRPC codec for runtime (reflection-based) protobuf messages
#[derive(Clone)]
pub struct DynamicProstCodec {
    descriptor: MessageDescriptor,
}

impl DynamicProstCodec {
    /// Create a new codec for a specific message type
    pub fn new(descriptor: MessageDescriptor) -> Self {
        Self { descriptor }
    }
}

impl Codec for DynamicProstCodec {
    type Encode = DynamicMessage;
    type Decode = DynamicMessage;

    type Encoder = DynamicProstEncoder;
    type Decoder = DynamicProstDecoder;

    fn encoder(&mut self) -> Self::Encoder {
        DynamicProstEncoder
    }

    fn decoder(&mut self) -> Self::Decoder {
        DynamicProstDecoder {
            descriptor: self.descriptor.clone(),
        }
    }
}

/// Encoder: DynamicMessage -> protobuf bytes
#[derive(Clone, Default)]
pub struct DynamicProstEncoder;

impl Encoder for DynamicProstEncoder {
    type Item = DynamicMessage;
    type Error = Status;

    fn encode(
        &mut self,
        item: Self::Item,
        dst: &mut EncodeBuf<'_>,
    ) -> Result<(), Self::Error> {
        item.encode(dst)
            .map_err(|e| Status::internal(e.to_string()))
    }
}

/// Decoder: protobuf bytes -> DynamicMessage
#[derive(Clone)]
pub struct DynamicProstDecoder {
    descriptor: MessageDescriptor,
}

impl Decoder for DynamicProstDecoder {
    type Item = DynamicMessage;
    type Error = Status;

    fn decode(
        &mut self,
        src: &mut DecodeBuf<'_>,
    ) -> Result<Option<Self::Item>, Self::Error> {
        if src.remaining() == 0 {
            return Ok(None);
        }

        DynamicMessage::decode(self.descriptor.clone(), src)
            .map(Some)
            .map_err(|e| Status::internal(e.to_string()))
    }
}


/* *
* 
*/