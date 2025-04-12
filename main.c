#include <stdio.h>
#include "lib.h"
#include <unistd.h>

int main() {
    MyCLibrary *my_library = MyCLibrary_Create();
    uint64_t userdata1 = 1;
    uint64_t userdata2 = 2;
    uint64_t result1;
    uint64_t result2;
    MyCLibrary_SleepAndAdd(my_library, userdata1, 1, 2, &result1);
    MyCLibrary_SleepAndAdd(my_library, userdata2, 3, 4, &result2);
    CompletedRequest completed_requests[2];
    uint64_t wait_num = 2;
    MyCLibrary_GetCompletedRequests(my_library, completed_requests, 2, wait_num);
    printf("Completed request 1: Userdata: %llu, Return code: %d, Result: %llu\n", completed_requests[0].userdata, completed_requests[0].result, result1);
    printf("Completed request 2: Userdata: %llu, Return code: %d, Result: %llu\n", completed_requests[1].userdata, completed_requests[1].result, result2);
    MyCLibrary_Destroy(&my_library);
    return 0;
}
