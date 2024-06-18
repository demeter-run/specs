# Demeter Specs

Shared interface definitions among different components.

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
