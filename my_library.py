import ctypes
import os
from typing import List, Tuple
import asyncio

class CompletedRequest:
    def __init__(self, userdata: int, result: int):
        self.userdata = userdata
        self.result = result

    def __str__(self):
        return f"Userdata: {self.userdata}, Return code: {self.result}"

class MyLibrary:
    def __init__(self):
        # Load the shared library
        self.lib = ctypes.CDLL(os.path.abspath("target/debug/libasync_rust_from_python.dylib"))
        
        # Define the C types
        class CCompletedRequest(ctypes.Structure):
            _fields_ = [
                ("userdata", ctypes.c_uint64),
                ("result", ctypes.c_uint32),
            ]
        self.CCompletedRequest = CCompletedRequest
        
        # Define the function signatures
        self.lib.MyLibrary_Create.restype = ctypes.c_void_p
        
        self.lib.MyLibrary_Destroy.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
        
        self.lib.MyLibrary_SleepAndAdd.argtypes = [
            ctypes.c_void_p,  # my_library
            ctypes.c_uint64,  # userdata
            ctypes.c_uint64,  # left
            ctypes.c_uint64,  # right
            ctypes.POINTER(ctypes.c_uint64),  # result
        ]
        
        self.lib.MyLibrary_GetCompletedRequests.argtypes = [
            ctypes.c_void_p,  # my_library
            ctypes.POINTER(CCompletedRequest),  # completed_requests
            ctypes.c_uint64,  # completed_requests_len
            ctypes.c_uint64,  # wait_num
        ]
        self.lib.MyLibrary_GetCompletedRequests.restype = ctypes.c_uint64
        
        # Create the library instance
        self.my_library = self.lib.MyLibrary_Create()

        self.outstanding_requests = {}
        self.userdata_counter = 0
        asyncio.create_task(self.check_completed_requests())
        
    def __del__(self):
        # Clean up when the object is garbage collected
        if hasattr(self, 'my_library'):
            self.lib.MyLibrary_Destroy(ctypes.byref(ctypes.c_void_p(self.my_library)))
    
    async def sleep_and_add(self,  left: int, right: int) -> int:
        userdata = self.userdata_counter
        self.userdata_counter += 1

        # Submit the request to the library
        result = ctypes.c_uint64()
        self.lib.MyLibrary_SleepAndAdd(self.my_library, userdata, left, right, ctypes.byref(result))

        # Wait for the result
        self.outstanding_requests[userdata] = { 'return_code': None, 'event': asyncio.Event() }
        await self.outstanding_requests[userdata]['event'].wait()

        # Get the result and delete the request
        return_code = self.outstanding_requests[userdata]['return_code']
        del self.outstanding_requests[userdata]
        return result.value
    
    # Background task to check for completed requests and set the result
    async def check_completed_requests(self):
        # Create an array to hold completed requests
        completed_requests = (self.CCompletedRequest * 10)()  # Allocate space for up to 10 requests
        while True:
            count = self.lib.MyLibrary_GetCompletedRequests(
                self.my_library,
                completed_requests,
                10,  # completed_requests_len
                0    # wait_num - don't wait, just check
            )
            for i in range(count):
                req = completed_requests[i]
                self.outstanding_requests[req.userdata]['return_code'] = req.result
                self.outstanding_requests[req.userdata]['event'].set()
            # Yield to allow other tasks to run
            await asyncio.sleep(0)