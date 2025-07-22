/**
 * C++ Buffer Overflow Example - TYPE UNSAFE
 * 
 * This program demonstrates how C++ allows dangerous buffer overflows
 * that can lead to undefined behavior, security vulnerabilities,
 * and program crashes.
 */

#include <iostream>
#include <cstring>

void demonstrate_buffer_overflow() {
    // Create a small buffer
    char buffer[10];
    
    // This is DANGEROUS - writing beyond buffer bounds
    // C++ compiler allows this, but it's undefined behavior
    const char* dangerous_input = "This string is much longer than 10 characters and will overflow!";
    
    std::cout << "Buffer size: " << sizeof(buffer) << " bytes" << std::endl;
    std::cout << "Input size: " << strlen(dangerous_input) << " characters" << std::endl;
    
    // UNSAFE: strcpy doesn't check bounds
    strcpy(buffer, dangerous_input);
    
    std::cout << "Buffer contents (if program didn't crash): " << buffer << std::endl;
    
    // Try to access beyond buffer bounds
    for (int i = 0; i < 20; i++) {
        std::cout << "buffer[" << i << "] = " << (int)buffer[i] << std::endl;
    }
}

void array_bounds_violation() {
    int arr[5] = {1, 2, 3, 4, 5};
    
    // C++ allows accessing beyond array bounds - UNSAFE!
    std::cout << "Valid access: arr[4] = " << arr[4] << std::endl;
    std::cout << "UNSAFE access: arr[10] = " << arr[10] << std::endl;  // Undefined behavior
    std::cout << "UNSAFE access: arr[-1] = " << arr[-1] << std::endl; // Undefined behavior
    
    // This can corrupt memory or crash the program
    arr[100] = 42;  // Writing to random memory location
}

int main() {
    std::cout << "=== C++ Type Unsafe Buffer Operations ===" << std::endl;
    
    try {
        std::cout << "\n1. Buffer Overflow Demonstration:" << std::endl;
        demonstrate_buffer_overflow();
        
        std::cout << "\n2. Array Bounds Violation:" << std::endl;
        array_bounds_violation();
        
    } catch (...) {
        std::cout << "Program crashed due to undefined behavior!" << std::endl;
    }
    
    std::cout << "\nNote: This program may crash, corrupt memory, or produce unpredictable results." << std::endl;
    std::cout << "C++ provides no compile-time or runtime protection against these errors." << std::endl;
    
    return 0;
}
