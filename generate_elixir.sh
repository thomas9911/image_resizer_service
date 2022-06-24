python3 -m grpc_tools.protoc -I ./proto/resizer --elixir_out=gen_descriptors=true,plugins=grpc:./elixir-client/priv/proto ./proto/resizer/resizer.proto
