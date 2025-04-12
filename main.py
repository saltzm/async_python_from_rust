from my_library import MyLibrary, CompletedRequest
import asyncio

async def main():
    lib = MyLibrary()
    
    # Use gather to run operations concurrently
    result1, result2 = await asyncio.gather(
        lib.sleep_and_add(1, 2),
        lib.sleep_and_add(2, 4)
    )
    
    print(f"Completed request 1: {result1}")
    print(f"Completed request 2: {result2}")

    # Run sequentially
    result3 = await lib.sleep_and_add(1, 7)
    result4 = await lib.sleep_and_add(2, 9)
    
    print(f"Completed request 3: {result3}")
    print(f"Completed request 4: {result4}")

if __name__ == "__main__":
    asyncio.run(main()) 