defmodule Resizer.ResizeMethod do
  @moduledoc false
  use Protobuf, enum: true, protoc_gen_elixir_version: "0.10.0", syntax: :proto3

  def descriptor do
    # credo:disable-for-next-line
    %Google.Protobuf.EnumDescriptorProto{
      __unknown_fields__: [],
      name: "ResizeMethod",
      options: nil,
      reserved_name: [],
      reserved_range: [],
      value: [
        %Google.Protobuf.EnumValueDescriptorProto{
          __unknown_fields__: [],
          name: "FILL",
          number: 0,
          options: nil
        },
        %Google.Protobuf.EnumValueDescriptorProto{
          __unknown_fields__: [],
          name: "FIT",
          number: 1,
          options: nil
        },
        %Google.Protobuf.EnumValueDescriptorProto{
          __unknown_fields__: [],
          name: "LIMIT",
          number: 2,
          options: nil
        },
        %Google.Protobuf.EnumValueDescriptorProto{
          __unknown_fields__: [],
          name: "PAD",
          number: 3,
          options: nil
        }
      ]
    }
  end

  field(:FILL, 0)
  field(:FIT, 1)
  field(:LIMIT, 2)
  field(:PAD, 3)
end

defmodule Resizer.ResizeReply.Status do
  @moduledoc false
  use Protobuf, enum: true, protoc_gen_elixir_version: "0.10.0", syntax: :proto3

  def descriptor do
    # credo:disable-for-next-line
    %Google.Protobuf.EnumDescriptorProto{
      __unknown_fields__: [],
      name: "Status",
      options: nil,
      reserved_name: [],
      reserved_range: [],
      value: [
        %Google.Protobuf.EnumValueDescriptorProto{
          __unknown_fields__: [],
          name: "OK",
          number: 0,
          options: nil
        },
        %Google.Protobuf.EnumValueDescriptorProto{
          __unknown_fields__: [],
          name: "ERROR",
          number: 1,
          options: nil
        }
      ]
    }
  end

  field(:OK, 0)
  field(:ERROR, 1)
end

defmodule Resizer.ResizeBinaryReply.Status do
  @moduledoc false
  use Protobuf, enum: true, protoc_gen_elixir_version: "0.10.0", syntax: :proto3

  def descriptor do
    # credo:disable-for-next-line
    %Google.Protobuf.EnumDescriptorProto{
      __unknown_fields__: [],
      name: "Status",
      options: nil,
      reserved_name: [],
      reserved_range: [],
      value: [
        %Google.Protobuf.EnumValueDescriptorProto{
          __unknown_fields__: [],
          name: "OK",
          number: 0,
          options: nil
        },
        %Google.Protobuf.EnumValueDescriptorProto{
          __unknown_fields__: [],
          name: "ERROR",
          number: 1,
          options: nil
        }
      ]
    }
  end

  field(:OK, 0)
  field(:ERROR, 1)
end

defmodule Resizer.ResizeRequest do
  @moduledoc false
  use Protobuf, protoc_gen_elixir_version: "0.10.0", syntax: :proto3

  def descriptor do
    # credo:disable-for-next-line
    %Google.Protobuf.DescriptorProto{
      __unknown_fields__: [],
      enum_type: [],
      extension: [],
      extension_range: [],
      field: [
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "bucket",
          label: :LABEL_OPTIONAL,
          name: "bucket",
          number: 1,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_STRING,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "input",
          label: :LABEL_OPTIONAL,
          name: "input",
          number: 2,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_STRING,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "output",
          label: :LABEL_OPTIONAL,
          name: "output",
          number: 3,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_STRING,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "width",
          label: :LABEL_OPTIONAL,
          name: "width",
          number: 4,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_UINT32,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "height",
          label: :LABEL_OPTIONAL,
          name: "height",
          number: 5,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_UINT32,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "method",
          label: :LABEL_OPTIONAL,
          name: "method",
          number: 6,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_ENUM,
          type_name: ".resizer.ResizeMethod"
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "config",
          label: :LABEL_OPTIONAL,
          name: "config",
          number: 9,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_BYTES,
          type_name: nil
        }
      ],
      name: "ResizeRequest",
      nested_type: [],
      oneof_decl: [],
      options: nil,
      reserved_name: [],
      reserved_range: []
    }
  end

  field(:bucket, 1, type: :string)
  field(:input, 2, type: :string)
  field(:output, 3, type: :string)
  field(:width, 4, type: :uint32)
  field(:height, 5, type: :uint32)
  field(:method, 6, type: Resizer.ResizeMethod, enum: true)
  field(:config, 9, type: :bytes)
end

defmodule Resizer.ResizeReply do
  @moduledoc false
  use Protobuf, protoc_gen_elixir_version: "0.10.0", syntax: :proto3

  def descriptor do
    # credo:disable-for-next-line
    %Google.Protobuf.DescriptorProto{
      __unknown_fields__: [],
      enum_type: [
        %Google.Protobuf.EnumDescriptorProto{
          __unknown_fields__: [],
          name: "Status",
          options: nil,
          reserved_name: [],
          reserved_range: [],
          value: [
            %Google.Protobuf.EnumValueDescriptorProto{
              __unknown_fields__: [],
              name: "OK",
              number: 0,
              options: nil
            },
            %Google.Protobuf.EnumValueDescriptorProto{
              __unknown_fields__: [],
              name: "ERROR",
              number: 1,
              options: nil
            }
          ]
        }
      ],
      extension: [],
      extension_range: [],
      field: [
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "message",
          label: :LABEL_OPTIONAL,
          name: "message",
          number: 1,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_STRING,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "status",
          label: :LABEL_OPTIONAL,
          name: "status",
          number: 2,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_ENUM,
          type_name: ".resizer.ResizeReply.Status"
        }
      ],
      name: "ResizeReply",
      nested_type: [],
      oneof_decl: [],
      options: nil,
      reserved_name: [],
      reserved_range: []
    }
  end

  field(:message, 1, type: :string)
  field(:status, 2, type: Resizer.ResizeReply.Status, enum: true)
end

defmodule Resizer.ResizeBinaryRequest do
  @moduledoc false
  use Protobuf, protoc_gen_elixir_version: "0.10.0", syntax: :proto3

  def descriptor do
    # credo:disable-for-next-line
    %Google.Protobuf.DescriptorProto{
      __unknown_fields__: [],
      enum_type: [],
      extension: [],
      extension_range: [],
      field: [
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "format",
          label: :LABEL_OPTIONAL,
          name: "format",
          number: 1,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_STRING,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "width",
          label: :LABEL_OPTIONAL,
          name: "width",
          number: 4,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_UINT32,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "height",
          label: :LABEL_OPTIONAL,
          name: "height",
          number: 5,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_UINT32,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "method",
          label: :LABEL_OPTIONAL,
          name: "method",
          number: 6,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_ENUM,
          type_name: ".resizer.ResizeMethod"
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "image",
          label: :LABEL_OPTIONAL,
          name: "image",
          number: 9,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_BYTES,
          type_name: nil
        }
      ],
      name: "ResizeBinaryRequest",
      nested_type: [],
      oneof_decl: [],
      options: nil,
      reserved_name: [],
      reserved_range: []
    }
  end

  field(:format, 1, type: :string)
  field(:width, 4, type: :uint32)
  field(:height, 5, type: :uint32)
  field(:method, 6, type: Resizer.ResizeMethod, enum: true)
  field(:image, 9, type: :bytes)
end

defmodule Resizer.ResizeBinaryReply do
  @moduledoc false
  use Protobuf, protoc_gen_elixir_version: "0.10.0", syntax: :proto3

  def descriptor do
    # credo:disable-for-next-line
    %Google.Protobuf.DescriptorProto{
      __unknown_fields__: [],
      enum_type: [
        %Google.Protobuf.EnumDescriptorProto{
          __unknown_fields__: [],
          name: "Status",
          options: nil,
          reserved_name: [],
          reserved_range: [],
          value: [
            %Google.Protobuf.EnumValueDescriptorProto{
              __unknown_fields__: [],
              name: "OK",
              number: 0,
              options: nil
            },
            %Google.Protobuf.EnumValueDescriptorProto{
              __unknown_fields__: [],
              name: "ERROR",
              number: 1,
              options: nil
            }
          ]
        }
      ],
      extension: [],
      extension_range: [],
      field: [
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "message",
          label: :LABEL_OPTIONAL,
          name: "message",
          number: 1,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_STRING,
          type_name: nil
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "status",
          label: :LABEL_OPTIONAL,
          name: "status",
          number: 2,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_ENUM,
          type_name: ".resizer.ResizeBinaryReply.Status"
        },
        %Google.Protobuf.FieldDescriptorProto{
          __unknown_fields__: [],
          default_value: nil,
          extendee: nil,
          json_name: "image",
          label: :LABEL_OPTIONAL,
          name: "image",
          number: 9,
          oneof_index: nil,
          options: nil,
          proto3_optional: nil,
          type: :TYPE_BYTES,
          type_name: nil
        }
      ],
      name: "ResizeBinaryReply",
      nested_type: [],
      oneof_decl: [],
      options: nil,
      reserved_name: [],
      reserved_range: []
    }
  end

  field(:message, 1, type: :string)
  field(:status, 2, type: Resizer.ResizeBinaryReply.Status, enum: true)
  field(:image, 9, type: :bytes)
end

defmodule Resizer.Resizer.Service do
  @moduledoc false
  use GRPC.Service, name: "resizer.Resizer", protoc_gen_elixir_version: "0.10.0"

  def descriptor do
    # credo:disable-for-next-line
    %Google.Protobuf.ServiceDescriptorProto{
      __unknown_fields__: [],
      method: [
        %Google.Protobuf.MethodDescriptorProto{
          __unknown_fields__: [],
          client_streaming: false,
          input_type: ".resizer.ResizeRequest",
          name: "Resize",
          options: %Google.Protobuf.MethodOptions{
            __pb_extensions__: %{},
            __unknown_fields__: [],
            deprecated: false,
            idempotency_level: :IDEMPOTENCY_UNKNOWN,
            uninterpreted_option: []
          },
          output_type: ".resizer.ResizeReply",
          server_streaming: false
        }
      ],
      name: "Resizer",
      options: nil
    }
  end

  rpc(:Resize, Resizer.ResizeRequest, Resizer.ResizeReply)
end

defmodule Resizer.Resizer.Stub do
  @moduledoc false
  use GRPC.Stub, service: Resizer.Resizer.Service
end

defmodule Resizer.ResizerBinary.Service do
  @moduledoc false
  use GRPC.Service, name: "resizer.ResizerBinary", protoc_gen_elixir_version: "0.10.0"

  def descriptor do
    # credo:disable-for-next-line
    %Google.Protobuf.ServiceDescriptorProto{
      __unknown_fields__: [],
      method: [
        %Google.Protobuf.MethodDescriptorProto{
          __unknown_fields__: [],
          client_streaming: false,
          input_type: ".resizer.ResizeBinaryRequest",
          name: "ResizeBinary",
          options: %Google.Protobuf.MethodOptions{
            __pb_extensions__: %{},
            __unknown_fields__: [],
            deprecated: false,
            idempotency_level: :IDEMPOTENCY_UNKNOWN,
            uninterpreted_option: []
          },
          output_type: ".resizer.ResizeBinaryReply",
          server_streaming: false
        }
      ],
      name: "ResizerBinary",
      options: nil
    }
  end

  rpc(:ResizeBinary, Resizer.ResizeBinaryRequest, Resizer.ResizeBinaryReply)
end

defmodule Resizer.ResizerBinary.Stub do
  @moduledoc false
  use GRPC.Stub, service: Resizer.ResizerBinary.Service
end
