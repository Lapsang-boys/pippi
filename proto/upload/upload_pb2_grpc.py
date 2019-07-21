# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
import grpc

import upload_pb2 as upload__pb2


class UploadStub(object):
  # missing associated documentation comment in .proto file
  pass

  def __init__(self, channel):
    """Constructor.

    Args:
      channel: A grpc.Channel.
    """
    self.Upload = channel.unary_unary(
        '/upload.Upload/Upload',
        request_serializer=upload__pb2.UploadRequest.SerializeToString,
        response_deserializer=upload__pb2.UploadReply.FromString,
        )


class UploadServicer(object):
  # missing associated documentation comment in .proto file
  pass

  def Upload(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')


def add_UploadServicer_to_server(servicer, server):
  rpc_method_handlers = {
      'Upload': grpc.unary_unary_rpc_method_handler(
          servicer.Upload,
          request_deserializer=upload__pb2.UploadRequest.FromString,
          response_serializer=upload__pb2.UploadReply.SerializeToString,
      ),
  }
  generic_handler = grpc.method_handlers_generic_handler(
      'upload.Upload', rpc_method_handlers)
  server.add_generic_rpc_handlers((generic_handler,))
