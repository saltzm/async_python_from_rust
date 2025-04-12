use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use crate::MyLibrary as MyLib;

#[repr(C)]
#[derive(Clone)]
pub struct CompletedRequest {
    userdata: u64,
}

pub struct MyLibrary {
    completed_requests_sender: mpsc::Sender<CompletedRequest>,
    completed_requests_receiver: mpsc::Receiver<CompletedRequest>,
    library: MyLib,
    runtime: Runtime,
}

#[no_mangle]
pub extern "C" fn MyLibrary_Create() -> *mut MyLibrary {
    let (sender, receiver) = mpsc::channel(128); // Channel with capacity of 128
    Box::into_raw(Box::new(MyLibrary {
        completed_requests_sender: sender,
        completed_requests_receiver: receiver,
        library: MyLib::new(),
        runtime: Runtime::new().expect("Failed to create Tokio runtime"),
    }))
}

#[no_mangle]
pub extern "C" fn MyLibrary_Destroy(my_library: *mut *mut MyLibrary) {
    unsafe {
        if !my_library.is_null() {
            let boxed = Box::from_raw(*my_library);
            drop(boxed);
            *my_library = std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub extern "C" fn MyLibrary_SleepAndAdd(my_library: *mut MyLibrary, userdata: u64, left: u64, right: u64, result: *mut u64) {
    let my_library = unsafe { &mut *my_library };
    let result = unsafe { &mut *result };
    
    let userdata_clone = userdata;
    let completed_requests_sender = my_library.completed_requests_sender.clone();
    let library = &my_library.library;
    
    my_library.runtime.spawn(async move {
        *result = library.sleep_and_add(left, right).await;
        // Simply send the completed request directly to the channel
        if let Err(e) = completed_requests_sender.send(CompletedRequest {
            userdata: userdata_clone,
        }).await {
            eprintln!("Failed to send completed request: {}", e);
        }
    });
    
    println!("Request added for userdata: {}", userdata);
}

#[no_mangle]
pub extern "C" fn MyLibrary_GetCompletedRequests(
    my_library: *mut MyLibrary,
    completed_requests: *mut CompletedRequest,
    completed_requests_len: u64,
    wait_num: u64,
) -> u64 {
    let my_library = unsafe { &mut *my_library };
    
    let mut count = 0;
    
    // Use try_recv to get available messages without waiting
    if !completed_requests.is_null() && completed_requests_len > 0 {
        let dest_slice = unsafe { std::slice::from_raw_parts_mut(completed_requests, completed_requests_len as usize) };
        
        // If wait_num > 0, we'll wait for at least that many requests
        if wait_num > 0 {
            my_library.runtime.block_on(async {
                // We need to receive exactly wait_num requests or until the channel is empty
                while count < wait_num && count < completed_requests_len {
                    match my_library.completed_requests_receiver.recv().await {
                        Some(request) => {
                            dest_slice[count as usize] = request;
                            count += 1;
                        },
                        None => break, // Channel closed
                    }
                }
            });
        } else {
            // Don't wait, just get what's available
            my_library.runtime.block_on(async {
                // Collect up to completed_requests_len requests without waiting
                while count < completed_requests_len {
                    match my_library.completed_requests_receiver.try_recv() {
                        Ok(request) => {
                            dest_slice[count as usize] = request;
                            count += 1;
                        },
                        Err(_) => break, // No more messages available or channel closed
                    }
                }
            });
        }
    }
    
    count
} 