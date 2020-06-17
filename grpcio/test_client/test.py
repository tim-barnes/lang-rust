import pytest
import grpc
import os

import rustserver_pb2
import rustserver_pb2_grpc


@pytest.fixture("session")
def channel():
    return grpc.insecure_channel('localhost:50008', options=[
        ('grpc.max_send_message_length', 1024**3), 
        ('grpc.max_receive_message_length', 1024**3)
    ])
    
@pytest.fixture("session")
def stub(channel):
    return rustserver_pb2_grpc.rustserverStub(channel)

@pytest.fixture
def request_factory():
    def func(sz):
        request = rustserver_pb2.Request(
            original_file_name="test.bin",
            content=os.urandom(sz)
        )
        return request
    return func


# Tests a range of asset sizes from 1kB to 1GB
@pytest.mark.parametrize("asset_size", [ 1024 * 2 ** x for x in range(0, 21, 1) ])
def test_payload_size(stub, request_factory, asset_size):
    request = request_factory(asset_size)
    response = stub.Process(request)

    assert isinstance(response, rustserver_pb2.Response)
    assert len(response.econtent) == asset_size

