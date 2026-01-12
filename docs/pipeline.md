## 

```
.proto
  ↓
FileDescriptorProto   (descriptor)
  ↓
language-specific codegen
  ↓
Go / Rust / Java files

```

so the specific language codegen creates that files in that language from `FileDescriptorProto` that means 