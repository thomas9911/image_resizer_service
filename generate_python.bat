python -m grpc_tools.protoc -I .\proto\resizer\ --python_out=client --grpc_python_out=client .\proto\resizer\resizer.proto