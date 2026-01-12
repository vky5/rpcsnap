# 📅 Day 1 Task — rpcSnap
## 🎯 Objective

Implement **proto discovery**.

By the end of today, rpcSnap must be able to:

> Load a `.proto` file and print a list of gRPC services and their RPC methods.

Nothing else.

---

## 🧱 Scope (do NOT exceed this)

You will implement **only**:

* Proto file loading
* Service + RPC method discovery
* Console output

### Explicitly out of scope

* UI
* gRPC execution
* Request building
* JSON mapping
* Error polish
* Multiple proto files (single entry file is enough)

If you touch these, you’re over-scoping.

---

## 📁 Files you are allowed to touch today

* `src/main.rs`
* `src/proto/loader.rs`
* `src/proto/model.rs`
* `src/proto/mod.rs`

Nothing else.

---

## 📐 Functional requirements

1. Given a path to a `.proto` file:

   * parse it
   * resolve its imports (best-effort is fine)
2. Discover:

   * service names
   * RPC method names
   * input message name
   * output message name
3. Map the result into **your own internal structs**
4. Print output like:

```
Service: UserService
  RPC: CreateUser (CreateUserRequest -> CreateUserResponse)
  RPC: GetUser (GetUserRequest -> GetUserResponse)
```

---

## 🧠 Constraints (important)

* You **must not** expose prost / descriptor types outside `proto/`
* `proto/model.rs` must define clean structs
* `loader.rs` returns `Vec<Service>`
* No globals
* No unsafe
* No TODOs left behind

---

## 🧪 Input assumptions

You may assume:

* `.proto` compiles
* proto3 syntax
* unary RPCs only

You may **hardcode** the proto path in `main.rs` for now.

---

## ✅ Acceptance criteria (how I’ll judge)

I will consider this task **done** if:

* `cargo run` prints correct services & RPCs
* Code is readable
* No overengineering
* No leaking of prost types
* Clear separation of concerns

If it *works but is messy*, I’ll send you back.

---

## 🧭 Hints (not instructions)

* `prost-reflect` is your friend
* Descriptor sets matter
* Start from “what data do I need?” and work backward
* Clone early, optimize later

---

## 🕕 When to report back

Come back when:

* It compiles
* It prints output
* Or you’re **properly stuck** (not “confused”, *stuck*)

When you return, I’ll:

* review your approach
* point out design mistakes
* assign **Day 2**

Go.
