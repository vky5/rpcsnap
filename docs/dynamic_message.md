## Dynamic Message
Dynamic message is a protobuf message whose value is only known in runtime

- Generated struct: compile time known shape
- dynamic message: runtime known shape

| Thing               | What it represents      |
| ------------------- | ----------------------- |
| `MessageDescriptor` | **Schema** (structure)  |
| `DynamicMessage`    | **Value** (actual data) |


>Basically DynamicMessage is the response or the request parameter for the RPC call that is made

