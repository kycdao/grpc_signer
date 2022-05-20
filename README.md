# GRPC Signer for kycDAO

GRPC API for extracting Crypto signing functionality into a separate microservice

## Codegen:

The contents of `signer.rs` are generated from the `.proto` file(s). Use `cargo run` in the `codegen` directory to re-generate them.

Requires Protobuf compiler, which can be installed with the following command:

`apt install protobuf-compiler`

Note: If the Protobuf compiler is not installed, `protos` will try to compile it from source, which requires `cmake`. If you get an error about missing `cmake`, install the compiler instead with the above command. This way the build will be much quicker.
