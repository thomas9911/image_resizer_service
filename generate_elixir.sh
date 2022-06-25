python3 -m grpc_tools.protoc -I ./server/proto/resizer --elixir_out=gen_descriptors=true,plugins=grpc:./elixir-client/priv/proto ./server/proto/resizer/resizer.proto
