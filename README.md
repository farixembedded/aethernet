# Aethernet - Redis based IPC for Rust

Aethernet is an IPC system for Rust that provides RPC through the [ReqRep
pattern](https://en.wikipedia.org/wiki/Request%E2%80%93response) pattern, and event driven updates
through the [PubSub pattern](https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern). It is
built on top of Redis which serves and the transport and broker mechanism between processes.

Prior to v1.0 this library is being published for visibility and should be used with thoughtful
caution. This library was developed by [Matt of Farix Embedded LLC](https://www.farixembedded.com/)
while consulting for [studio 3e8](https://www.studio3e8.com/) for internal use, but we felt it would
be cool to open source it in the hopes that it might be helpful is some form to the community.

# Quick Start

Docs aren't really complete (or started) right now. Check out the included examples in
`aethernet/examples/`. That is the best way to get a rough idea for the usage intent.

TODO: should focus more on Rust style docs in the code.

# Architecture

Aethernet was implemented after looking for a for an existing IPC solution for our embedded Linux
microservice architecture. Several options were considered (ZBus/DBus, Varlink, Cap'n Proto, etc.),
but each one fell short of the design goals outlined above. Ultimately this library was built. It is
not intended to replace any other existing IPC systems, but may be useful to others with similar
design goals in an IPC system.

## Design Goals

* Optimize for developer ergonomics.
  * Adding and changing the IPC definition should be painless and touch as few files as possible.
  * Any code generation if needed should be handled by the build system, and changes picked up on
    automatically.
  * Changes to the ICD should result in compiler errors where code changes are required to support
    them.
* Rust is the first class citizen, but the protocol and interface is designed to not preclude
  implementations in other langues.
* Don't prematurely optimize at the expense of the above design goals. (De)serialization, data
  format size, and context switches are not going to be hyper optimized.
* Current anti-goals include run-time introspection and self-describing wire formats.

## ICD (Interface Control Document)

We hijack a sub-set of Rust for out [ICD](https://en.wikipedia.org/wiki/Interface_control_document).
A proc macro is used to do code generation in Rust, and this can be extended to emit bindings for
other langues as well as a standalone tool.

## IPC Protocol

### Namespacing and Key Naming

The Redis keys are namespaced under the top level `aethernet:*` to not preclude use of redis for
other non Aethernet uses. Namespace components are separated by the colon charcater `:`.

Namespaces:
* `[Global]`: Nominal `Aethernet`, may be configurable in the future.
  * `[Service]`: The runtime name of the service hosting RPC/Pubsub
    * `[Interface]`: The name of the interface being implmented
      * `reqrep`: The ReqRep name space
        * `rpc_request`: The incoming ordered queue of requests for this interface on this service.
        * `rpc_response:[Request ID]`: A one deep queue for the response to a single RPC request.
      * `pubsub`: The Pubsub namespace
        * `[Topic name]`: A pubsub channel for the server to publish pubssub updates for a topic to
        * `latest:[Topic name]`: A normal Redis value holding the latest published value for async
          fetching of the value. May be unset until first publish of the topci.

### Message Wire Format

At the core of the wire format is the notion of an Aethernet message. A message can be a request's
arguments, the response values, or the content of a Pubsub update. Messages are structs with fields
where the name is a string of the field, and the value is one of the above valid types described.

TODO: Get explicit about how the field names and topic/method names get coerced (i.e. to Pascal
case) in different places from the ICD.

* [serde_json's data model](https://serde.rs/data-model.html) is used as the ground truth for the
  message serialization format.
* Currently there aren't restrictions on any datatypes that are normally in scope in Rust. This will
  be reduced down to a supported list prior to v1.0. The intention is to support:
    * signed and unsigned integers: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`
    * Floating point: `f32`, `f64`
    * boolean: `bool`
    * Strings: `String`/`&str`
    * Vectors: `Vec<T>`/`&[T]`
    * Fixed length arrays: `[T; N]`/`&[T; N]`
    * Rusts built-in Result enum: `Result<T, E>`
    * Rusts nullable Option: `Option<T>`
    * Tuples: unnamed ordered compound of other types `(T1, T2, T3, ...)`
    * Arbitrary Enums and Structs (not currently implemented)
      * Define arbitrary enums and structs composed of the above data types
      * Only can use arbitrary compound types within the interface they are defined in so the ICD
        for any interface is self-contained without dependencies.
* Internal Aethernet Error messages take the format of and Enum with optional string value for some
  error types

A quick note about 64 bit integer types and json serialization is needed. Right now they are emitted
by `serde_json` as number literals which is not compatible with Javascript, but not strictly
incompatible with json itself. For Rust <-> Rust communication, serde_json will handle 64-bit
integers with no loss of precision. Other languages that might be expanded to include Python and
C++. Python's json (de)serialization and built-in integer type can handle arbitrarily large
integers. For C++ using nlohmann::json, a nice header only lib, will parse all ints to 64 bit out of
the box. The plan for now is to stick with this, but in the future 64-bit and larger ints could be
encoded as strings for maximal compatibility.

### Req/Rep RPC

ReqRep is implemented by having a single ordered queue of incoming requests for any given interface.
The RPC server pulls request out in order, handles them, and then posts the reply to a new key with
the request ID encoded in it. Replies have a Redis TTL set on them to prevent leaking if the client
fails remove them.

#### Request wire format

Requests are packaged in a request envelope:

* `Request ID`: A unique request ID. A UUID4 as a string to uniquely identify the request and
  associate the response to it.
* `Request Type`: A string identifying the method associated with the request of the format
  `[INTERFACE_NAME].[METHOD_NAME]`
* `Request body`: A valid message holding the request arguments associated with the request.

#### Response wire format

Responses also are packaged in an envelope represented as a Rust result: `Result<Message,
AethernetError>`

### Pub/Sub

When a Pubsub topic is published to it is both published to a Redis pubsub channel, and the value is
set in a normal Redis key for async fetching of the latest value later by the client. Clients must
handle the case of an unset value when fetching async.

The wire format for pubsub is an Aethernet message with no envelope.

# Language Implementations

## Rust

A proc macro is used to generate the code for intreating with Aethernet. Currently the
implementation only targets the Tokio async runtime.

## Python

There are plans to extend an implementation to Python. This will include a stand-alone library for
doing the heavy lifting of message serialization, publishing, and fielding the Reqrep pattern. In
addition, the current proc macro for generating Rust code will be split up and reused to also emit
python code describing the interface and giving first class Python objects for interfaces with them.


# License
Licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

Pull requests, issues and discussion are all welcomed. We expect all contributors to act with
respect and foster an inclusive, welcoming environment for everyone.
