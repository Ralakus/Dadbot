# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: backend.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import builder as _builder
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\rbackend.proto\x12\x07\x62\x61\x63kend\"\xe3\x02\n\x0bTaskRequest\x12\r\n\x05model\x18\x01 \x01(\t\x12\r\n\x05input\x18\x02 \x01(\t\x12\x14\n\x0ctoken_window\x18\x03 \x01(\x03\x12\x12\n\nmin_length\x18\x04 \x01(\x03\x12\x12\n\nmax_length\x18\x05 \x01(\x03\x12\x11\n\tdo_sample\x18\x06 \x01(\x08\x12\x16\n\x0e\x65\x61rly_stopping\x18\x07 \x01(\x08\x12\r\n\x05top_p\x18\x08 \x01(\x01\x12\r\n\x05top_k\x18\t \x01(\x03\x12\x13\n\x0btemperature\x18\n \x01(\x01\x12\x1a\n\x12repetition_penalty\x18\x0b \x01(\x01\x12\x16\n\x0elength_penalty\x18\x0c \x01(\x01\x12\x11\n\tnum_beams\x18\r \x01(\x03\x12\x17\n\x0fnum_beam_groups\x18\x0e \x01(\x03\x12\x1c\n\x14num_return_sequences\x18\x0f \x01(\x03\x12\x1c\n\x14no_repeat_ngram_size\x18\x10 \x01(\x03\"\x19\n\tTaskReply\x12\x0c\n\x04\x64\x61ta\x18\x01 \x03(\t\"!\n\x10LoadModelRequest\x12\r\n\x05model\x18\x01 \x01(\t\"!\n\x0eLoadModelReply\x12\x0f\n\x07success\x18\x01 \x01(\x08\x32\xbf\x01\n\x04Task\x12\x33\n\x07RunTask\x12\x14.backend.TaskRequest\x1a\x12.backend.TaskReply\x12?\n\tLoadModel\x12\x19.backend.LoadModelRequest\x1a\x17.backend.LoadModelReply\x12\x41\n\x0bUnloadModel\x12\x19.backend.LoadModelRequest\x1a\x17.backend.LoadModelReplyb\x06proto3')

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, globals())
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'backend_pb2', globals())
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _TASKREQUEST._serialized_start=27
  _TASKREQUEST._serialized_end=382
  _TASKREPLY._serialized_start=384
  _TASKREPLY._serialized_end=409
  _LOADMODELREQUEST._serialized_start=411
  _LOADMODELREQUEST._serialized_end=444
  _LOADMODELREPLY._serialized_start=446
  _LOADMODELREPLY._serialized_end=479
  _TASK._serialized_start=482
  _TASK._serialized_end=673
# @@protoc_insertion_point(module_scope)
