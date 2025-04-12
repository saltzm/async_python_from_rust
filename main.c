#include <stdio.h>
#include "my_library.h"
#include <unistd.h>

int main() {
    MyLibrary *my_library = MyLibrary_Create();
    uint64_t userdata1 = 1;
    uint64_t userdata2 = 2;
    uint64_t result1;
    uint64_t result2;
    MyLibrary_SleepAndAdd(my_library, userdata1, 1, 2, &result1);
    MyLibrary_SleepAndAdd(my_library, userdata2, 3, 4, &result2);
    CompletedRequest completed_requests[2];
    uint64_t wait_num = 2;
    MyLibrary_GetCompletedRequests(my_library, completed_requests, 2, wait_num);
    printf("Completed request 1: Userdata: %llu, Result: %llu\n", completed_requests[0].userdata, result1);
    printf("Completed request 2: Userdata: %llu, Result: %llu\n", completed_requests[1].userdata, result2);

    MyLibrary_SleepAndAdd(my_library, 42, 1, 2, &result1);
    MyLibrary_SleepAndAdd(my_library, 43, 3, 4, &result2);
    MyLibrary_GetCompletedRequests(my_library, completed_requests, 2, wait_num);
    printf("Completed request 1: Userdata: %llu, Result: %llu\n", completed_requests[0].userdata, result1);
    printf("Completed request 2: Userdata: %llu, Result: %llu\n", completed_requests[1].userdata, result2);
    
    MyLibrary_Destroy(&my_library);
    return 0;
}
