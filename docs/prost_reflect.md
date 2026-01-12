### prost_reflect
**prost_reflect work is to read the AST created by protoc**

AST - Abstract Syntax Tree
IR - 

**Descriptor :** A machine-readable summary of what’s inside a .proto file. (just metadata)'
Proto descriptor = AST for .proto files

```json
{
  "name": "UserService",
  "methods": [
    { "name": "CreateUser", "input": "...", "output": "..." }
  ]
}

```

You don’t manipulate raw JSON text.
You parse it into objects.

the descriptor data contains something like this 

```json
ServiceDescriptor {
  name: "UserService",
  methods: [
    MethodDescriptor {
      name: "CreateUser",
      input: "CreateUserRequest",
      output: "CreateUserResponse"
    }
  ]
}
```


>[!IMPORTANT]
.proto is NOT parsed by prost-reflect
Instead:
A tool called protoc reads the .proto file
It produces a descriptor file (binary)
Your Rust program loads that binary and inspects it


the flow is this:
```
.proto (text)
   ↓
protoc
   ↓
descriptor (binary data)
   ↓
prost-reflect
   ↓
you iterate services & methods
```

when we write thsi
```proto
service UserService {
  rpc CreateUser (Req) returns (Res);
}
```

protoc internally creates something like 
```
ServiceDescriptor {
  name: "UserService",
  methods: [
    MethodDescriptor {
      name: "CreateUser",
      input: "Req",
      output: "Res"
    }
  ]
}
```

prost-reflect gives you safe Rust accessors to that structure.