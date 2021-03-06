# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: resizer.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database

# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(
    b'\n\rresizer.proto\x12\x07resizer"m\n\rResizeRequest\x12\x0e\n\x06\x62ucket\x18\x01 \x01(\t\x12\r\n\x05input\x18\x02 \x01(\t\x12\x0e\n\x06output\x18\x03 \x01(\t\x12\r\n\x05width\x18\x04 \x01(\r\x12\x0e\n\x06height\x18\x05 \x01(\r\x12\x0e\n\x06\x63onfig\x18\t \x01(\x0c"h\n\x0bResizeReply\x12\x0f\n\x07message\x18\x01 \x01(\t\x12+\n\x06status\x18\x02 \x01(\x0e\x32\x1b.resizer.ResizeReply.Status"\x1b\n\x06Status\x12\x06\n\x02OK\x10\x00\x12\t\n\x05\x45RROR\x10\x01\x32\x43\n\x07Resizer\x12\x38\n\x06Resize\x12\x16.resizer.ResizeRequest\x1a\x14.resizer.ResizeReply"\x00\x62\x06proto3'
)


_RESIZEREQUEST = DESCRIPTOR.message_types_by_name["ResizeRequest"]
_RESIZEREPLY = DESCRIPTOR.message_types_by_name["ResizeReply"]
_RESIZEREPLY_STATUS = _RESIZEREPLY.enum_types_by_name["Status"]
ResizeRequest = _reflection.GeneratedProtocolMessageType(
    "ResizeRequest",
    (_message.Message,),
    {
        "DESCRIPTOR": _RESIZEREQUEST,
        "__module__": "resizer_pb2"
        # @@protoc_insertion_point(class_scope:resizer.ResizeRequest)
    },
)
_sym_db.RegisterMessage(ResizeRequest)

ResizeReply = _reflection.GeneratedProtocolMessageType(
    "ResizeReply",
    (_message.Message,),
    {
        "DESCRIPTOR": _RESIZEREPLY,
        "__module__": "resizer_pb2"
        # @@protoc_insertion_point(class_scope:resizer.ResizeReply)
    },
)
_sym_db.RegisterMessage(ResizeReply)

_RESIZER = DESCRIPTOR.services_by_name["Resizer"]
if _descriptor._USE_C_DESCRIPTORS == False:

    DESCRIPTOR._options = None
    _RESIZEREQUEST._serialized_start = 26
    _RESIZEREQUEST._serialized_end = 135
    _RESIZEREPLY._serialized_start = 137
    _RESIZEREPLY._serialized_end = 241
    _RESIZEREPLY_STATUS._serialized_start = 214
    _RESIZEREPLY_STATUS._serialized_end = 241
    _RESIZER._serialized_start = 243
    _RESIZER._serialized_end = 310
# @@protoc_insertion_point(module_scope)
