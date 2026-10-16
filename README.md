# Demeter Specs

Shared interface definitions among different components.

## Registration and claim

`ProvisionalService` (`proto/demeter/ops/v1alpha/provisional.proto`) and the
`ProjectService.FetchClaim` / `ClaimProject` RPCs implement the protocol through
which a coding agent with no Demeter account registers a provisional testnet
project, and a person later claims it. Their behaviour, authentication and
status codes are described by the registration-and-claim contract
(`solution/devex/contracts/registration-and-claim.md` in the Demeter domain),
section "gRPC in fabric", which is normative for field names and status codes.

## Contributing

To run the project will need to have the [buf CLI](https://buf.build/docs/installation) installed. The buf will generate the code from the proto files.

### Required plugins

Buf will require some plugins to generate the rust code. Execute the command below to install the plugins.

```sh
cargo install \
protobuf-codegen \
protoc-gen-prost-crate \
protoc-gen-tonic \
protoc-gen-prost-serde \
protoc-gen-prost
```

### Generate code

After the requirements are installed, use the buf to generate the code. The buf will use the `buf.gen.yaml` configuration.

```sh
buf generate
```
