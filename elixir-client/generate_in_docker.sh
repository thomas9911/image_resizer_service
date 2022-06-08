python3 -m grpc_tools.protoc -I . --elixir_out=gen_descriptors=true,plugins=grpc:./out --plugin /root/.mix/escripts/protoc-gen-elixir ./resizer.proto
