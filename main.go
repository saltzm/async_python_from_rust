package main

import (
	"fmt"
	"sync"
)

func main() {
	// Create a new instance of MyLibrary
	lib := NewMyLibrary()
	
	// Run operations concurrently with WaitGroup
	var wg sync.WaitGroup
	
	// Slice to store results
	results := make([]uint64, 2)
	
	// Run first two operations concurrently
	wg.Add(2)
	
	go func() {
		defer wg.Done()
		result, err := lib.SleepAndAdd(1, 2)
		if err != nil {
			fmt.Printf("Error in concurrent operation 1: %v\n", err)
			return
		}
		results[0] = result
	}()
	
	go func() {
		defer wg.Done()
		result, err := lib.SleepAndAdd(2, 4)
		if err != nil {
			fmt.Printf("Error in concurrent operation 2: %v\n", err)
			return
		}
		results[1] = result
	}()
	
	// Wait for both operations to complete
	wg.Wait()
	
	fmt.Printf("Completed concurrent request 1: %d\n", results[0])
	fmt.Printf("Completed concurrent request 2: %d\n", results[1])
	
	// Run sequentially
	result3, err := lib.SleepAndAdd(1, 7)
	if err != nil {
		fmt.Printf("Error in sequential operation 1: %v\n", err)
		return
	}
	
	result4, err := lib.SleepAndAdd(2, 9)
	if err != nil {
		fmt.Printf("Error in sequential operation 2: %v\n", err)
		return
	}
	
	fmt.Printf("Completed sequential request 1: %d\n", result3)
	fmt.Printf("Completed sequential request 2: %d\n", result4)
} 