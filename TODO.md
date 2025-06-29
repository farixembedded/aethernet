# TODO

Prior to v1.0 this library is in early development flux, and SemVer is not guaranteed and breaking
changes wil happen. As this list is widdled down and things mature, these should get ported into
GitHub issues.

* Get explicit about supported types
  * Document the types in the protocol docs
  * Add checks in the proc macro to only allow supported types are used. Need to write code for
    handling nested container types.
* Add support for custom structs and enums inside the mod for use in the interface.
* Handle robust Redis re-connection logic. Can likely lean on existing implementations within the
  `redis-rs` library for this.
* Remove all `.unwrap()` within generated and library code and handle and/or propagate all errors.
* Look into using [CoW](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
    * to cut down on the complexity of the generated message structs in the context of handling both
      references and by value
    * In the call/publish API to give the caller flexibility on pass by value or reference
    * Give more thought for compound and container data type how the type pairs lok for call/pub vs
      how they come in the handlers (i.e. `String` -> `&str`, `Vec<T>` -> `&[T]`). Need to think
      about other cases like fixed length array, tuples, enums, structs, etc.
* Add rust docstrings at least for all public APIs. Get docs together to go up to docs.rs.
* Take another look at the proc macro and see if there is any more code that can be factored out of
  the code generation and into actual Rust in the main library.
* Python implementation. Use the code generator to emit Python, write a python lib beneath this.
  * Likely this will result in a refactor of the proc macro to get clearer about the ICD extraction
    phase (generic of target language), and emission of the interface code.
* Architecture thoughts
  * Should move to a single Req queue per a service, not per service and interface? This would mean
    needing to give multiple Reqrep handlers to a single server instance (That's a good thing,
    right?). Would also want to investigate clients for multiple interfaces on the same service.
    * Could have another proc macro like #[aethernet::service] that takes the interfaces to generate
      a service for.
  * Is there a way in the subscriber to skip subbing to topics that have the default noop impl (i.e.
    they are ignored.). This isn't critical.
  * Add a state free interface for publishing and calling RPC that uses a connection pool under the
    hood. The allows calling RPC or publishing values with out constructing a client/publishing
    struct.
  * Support non-Tokio/asyc environments?
  * Support other async runtime envs than tokio? Right now that'd just be exposing the async handler
    function and letting the user spawn a task in their runtime and calling it I think. Could still
    have a feature to provide convenience functions for Tokio.
* Add CI w/ checks (clippy, udep, etc.) and build. Add automation for releases as well.
* Keep the examples up to date along the way and clean them up to demonstrate usage and features of
  the library. Add more comments along the way to help people understand usage.
