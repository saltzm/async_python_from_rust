use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::watch;
use crate::MyLibrary;

#[repr(C)]
#[derive(Clone)]
pub struct CompletedRequest {
    userdata: u64,
    result: u32,
}

pub struct MyCLibrary {
    completed_requests_sender: watch::Sender<Vec<CompletedRequest>>,
    completed_requests_receiver: watch::Receiver<Vec<CompletedRequest>>,
    library: MyLibrary,
    runtime: Runtime,
}

#[no_mangle]
pub extern "C" fn MyCLibrary_Create() -> *mut MyCLibrary {
    let (sender, receiver) = watch::channel(Vec::new());
    Box::into_raw(Box::new(MyCLibrary {
        completed_requests_sender: sender,
        completed_requests_receiver: receiver,
        library: MyLibrary::new(),
        runtime: Runtime::new().expect("Failed to create Tokio runtime"),
    }))
}

#[no_mangle]
pub extern "C" fn MyCLibrary_Destroy(my_library: *mut *mut MyCLibrary) {
    unsafe {
        if !my_library.is_null() {
            let boxed = Box::from_raw(*my_library);
            drop(boxed);
            *my_library = std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub extern "C" fn MyCLibrary_SleepAndAdd(my_library: *mut MyCLibrary, userdata: u64, left: u64, right: u64, result: *mut u64) {
    let my_library = unsafe { &mut *my_library };
    let result = unsafe { &mut *result };
    
    let userdata_clone = userdata;
    let completed_requests_sender = my_library.completed_requests_sender.clone();
    let library = &my_library.library;
    
    my_library.runtime.spawn(async move {
        *result = library.sleep_and_add(left, right).await;
        completed_requests_sender.send_modify(|requests| {
            requests.push(CompletedRequest {
                userdata: userdata_clone,
                result: 0,
            });
        });
    });
    
    println!("Request added for userdata: {}", userdata);
}

#[no_mangle]
pub extern "C" fn MyCLibrary_GetCompletedRequests(
    my_library: *mut MyCLibrary,
    completed_requests: *mut CompletedRequest,
    completed_requests_len: u64,
    wait_num: u64,
) -> u64 {
    let my_library = unsafe { &mut *my_library };
    
    // If wait_num > 0, wait for that many events
    if wait_num > 0 {
        let mut receiver = my_library.completed_requests_receiver.clone();
        my_library.runtime.block_on(async move {
            loop {
                let len = receiver.borrow().len() as u64;
                if len >= wait_num {
                    break;
                }
                receiver.changed().await.unwrap();
            }
        });
    }
    
    // Get the current requests and prepare to update them
    let mut processed_count = 0;
    
    // We need to modify the vector through the sender
    my_library.completed_requests_sender.send_modify(|requests| {
        // Get number of items we'll process
        let count = std::cmp::min(completed_requests_len, requests.len() as u64);
        processed_count = count;
        
        // Copy the requests to the provided array
        if !completed_requests.is_null() && count > 0 {
            let dest_slice = unsafe { std::slice::from_raw_parts_mut(completed_requests, count as usize) };
            for (i, request) in requests.iter().take(count as usize).enumerate() {
                dest_slice[i] = CompletedRequest {
                    userdata: request.userdata,
                    result: request.result,
                };
            }
            
            // Remove the consumed requests (drain the first 'count' elements)
            requests.drain(0..count as usize);
        }
    });
    
    processed_count
} 