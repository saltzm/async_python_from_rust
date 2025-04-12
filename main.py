import ctypes
import os

# Load the shared library
lib = ctypes.CDLL(os.path.abspath("target/debug/libasync_rust_from_python.dylib"))

# Define the C types
class CompletedRequest(ctypes.Structure):
    _fields_ = [
        ("userdata", ctypes.c_uint64),
        ("result", ctypes.c_uint32),
    ]

# Define the function signatures
lib.MyCLibrary_Create.restype = ctypes.c_void_p

lib.MyCLibrary_Destroy.argtypes = [ctypes.POINTER(ctypes.c_void_p)]

lib.MyCLibrary_SleepAndAdd.argtypes = [
    ctypes.c_void_p,  # my_library
    ctypes.c_uint64,  # userdata
    ctypes.c_uint64,  # left
    ctypes.c_uint64,  # right
    ctypes.POINTER(ctypes.c_uint64),  # result
]

lib.MyCLibrary_GetCompletedRequests.argtypes = [
    ctypes.c_void_p,  # my_library
    ctypes.POINTER(CompletedRequest),  # completed_requests
    ctypes.c_uint64,  # completed_requests_len
    ctypes.c_uint64,  # wait_num
]
lib.MyCLibrary_GetCompletedRequests.restype = ctypes.c_uint64

def main():
    # Create the library instance
    my_library = lib.MyCLibrary_Create()
    
    # Create variables to store results
    result1 = ctypes.c_uint64()
    result2 = ctypes.c_uint64()
    
    # Call sleep_and_add twice
    lib.MyCLibrary_SleepAndAdd(my_library, 1, 1, 2, ctypes.byref(result1))
    lib.MyCLibrary_SleepAndAdd(my_library, 2, 3, 4, ctypes.byref(result2))
    
    # Create array to store completed requests
    completed_requests = (CompletedRequest * 2)()
    
    # Get completed requests, waiting for both to complete
    count = lib.MyCLibrary_GetCompletedRequests(my_library, completed_requests, 2, 2)
    
    # Print results
    print(f"Completed request 1: Userdata: {completed_requests[0].userdata}, "
          f"Return code: {completed_requests[0].result}, Result: {result1.value}")
    print(f"Completed request 2: Userdata: {completed_requests[1].userdata}, "
          f"Return code: {completed_requests[1].result}, Result: {result2.value}")
    
    # Clean up
    lib.MyCLibrary_Destroy(ctypes.byref(ctypes.c_void_p(my_library)))

if __name__ == "__main__":
    main() 