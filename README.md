# rpcSnap 📅

`rpcSnap` is a lightweight, reflection-based CLI tool for making dynamic gRPC calls using `.proto` descriptor files. Unlike traditional gRPC clients, it doesn't require generated code for every service; it discovers services and methods at runtime.

## 🎯 Features

- **Proto Discovery**: Load `.proto` descriptors and inspect available services and methods.
- **Dynamic Messaging**: Build and send gRPC requests dynamically based on the descriptor.
- **Unary Calls**: Supports standard unary gRPC request-response cycles.
- **Custom Codec**: Features a `DynamicProstCodec` that handles runtime encoding and decoding of Protobuf messages.

## 🧱 Architecture

The project is built with Rust and leverages several key libraries:

- **[prost-reflect](https://github.com/tokio-rs/prost-reflect)**: Used for runtime induction of Protobuf messages and descriptors.
- **[tonic](https://github.com/hyperium/tonic)**: Provides the gRPC transport layer.
- **[tokio](https://tokio.rs/)**: Powers the asynchronous runtime.

### Project Structure

- `src/proto/`: Logic for loading descriptors and building dynamic messages.
- `src/grpc/`: gRPC client implementation and custom dynamic codec.
- `src/app.rs`: Main application orchestration logic.

## 🚀 Getting Started

### Prerequisites

- **Protoc**: You need the Protocol Buffers compiler to generate descriptor files.
- **Rust**: Ensure you have the latest stable Rust toolchain installed.

### 1. Generate a Descriptor File

`rpcSnap` operates on binary descriptor files (`.bin`). You can generate one from your `.proto` file using `protoc`:

```bash
protoc --include_imports --descriptor_set_out=descriptor.bin your_service.proto
```

For the included `ping.proto`:

```bash
protoc --include_imports --descriptor_set_out=ping.bin ping.proto
```

### 2. Run the Tool

Use `cargo run` to execute a gRPC call:

```bash
cargo run -- \
  --descriptor <path_to_descriptor.bin> \
  --service <package.ServiceName> \
  --method <MethodName> \
  --addr <http://host:port>
```

#### Example (using `ping.proto`)

If you have a server running locally on port 50051:

```bash
cargo run -- \
  --descriptor ping.bin \
  --service demo.v1.PingService \
  --method Ping \
  --addr http://127.0.0.1:50051
```

## 🧠 Technical Highlights

- **Dynamic Decoding**: The tool can decode any gRPC response without knowing the structure at compile time by using the response message descriptor provided in the proto file.
- **Type-Safe Internal Models**: While using reflection, `rpcSnap` maps Protobuf descriptors into clean internal Rust structs for easier manipulation and printing.

---

_Created by Vaibhav_
