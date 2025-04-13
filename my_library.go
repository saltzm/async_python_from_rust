package main

// #cgo LDFLAGS: -L./target/debug -lasync_rust_from_python
// #include "my_library.h"
import "C"
import (
	"fmt"
	"runtime"
	"sync"
	"time"
	"unsafe"
)

// CompletedRequest represents a completed request from the Rust library
type CompletedRequest struct {
	Userdata uint64
}

// MyLibrary wraps the Rust library functionality
type MyLibrary struct {
	instance        *C.struct_MyLibrary
	pendingRequests map[uint64]chan uint64
	mutex           sync.Mutex
	nextUserdata    uint64
	stopPolling     chan struct{}
}

// NewMyLibrary creates a new instance of the library
func NewMyLibrary() *MyLibrary {
	lib := &MyLibrary{
		instance:        C.MyLibrary_Create(),
		pendingRequests: make(map[uint64]chan uint64),
		mutex:           sync.Mutex{},
		nextUserdata:    0,
		stopPolling:     make(chan struct{}),
	}
	
	// Start a goroutine to poll for completed requests
	go lib.pollCompletedRequests()
	
	// Set finalizer to ensure cleanup
	runtime.SetFinalizer(lib, freeMyLibrary)
	
	return lib
}

// Free ensures the library resources are properly cleaned up
func freeMyLibrary(lib *MyLibrary) {
	// Signal the polling goroutine to stop
	close(lib.stopPolling)
	
	// Free the C library
	instance := lib.instance
	C.MyLibrary_Destroy(&instance)
}

// SleepAndAdd performs an asynchronous add operation with a sleep
func (lib *MyLibrary) SleepAndAdd(left, right uint64) (uint64, error) {
	lib.mutex.Lock()
	userdata := lib.nextUserdata
	lib.nextUserdata++
	resultChan := make(chan uint64, 1)
	lib.pendingRequests[userdata] = resultChan
	lib.mutex.Unlock()
	
	// Prepare the result pointer
	var result C.uint64_t
	
	// Call the Rust function
	C.MyLibrary_SleepAndAdd(
		lib.instance,
		C.uint64_t(userdata),
		C.uint64_t(left),
		C.uint64_t(right),
		&result,
	)
	
	// Wait for the result
	select {
	case <-resultChan:
		// Return the result from the C function
		return uint64(result), nil
	case <-time.After(5 * time.Second):
		// Clean up if timed out
		lib.mutex.Lock()
		delete(lib.pendingRequests, userdata)
		lib.mutex.Unlock()
		return 0, fmt.Errorf("operation timed out")
	}
}

// pollCompletedRequests continuously polls for completed requests
func (lib *MyLibrary) pollCompletedRequests() {
	// Allocate a buffer to receive completed requests
	const bufferSize = 10
	completedRequests := make([]C.struct_CompletedRequest, bufferSize)
	
	for {
		select {
		case <-lib.stopPolling:
			return
		default:
			// Get completed requests without waiting
			count := C.MyLibrary_GetCompletedRequests(
				lib.instance,
				(*C.struct_CompletedRequest)(unsafe.Pointer(&completedRequests[0])),
				C.uint64_t(bufferSize),
				C.uint64_t(0), // Don't wait
			)
			
			// Process all completed requests
			for i := 0; i < int(count); i++ {
				userdata := uint64(completedRequests[i].userdata)
				
				lib.mutex.Lock()
				if resultChan, ok := lib.pendingRequests[userdata]; ok {
					// Signal completion without value since we already have the result
					resultChan <- 0
					delete(lib.pendingRequests, userdata)
				}
				lib.mutex.Unlock()
			}
			
			// Sleep briefly to avoid consuming too much CPU
			time.Sleep(1 * time.Millisecond)
		}
	}
} 