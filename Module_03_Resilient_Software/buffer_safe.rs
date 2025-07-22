/**
 * Rust Buffer Safety Example - TYPE SAFE
 * 
 * This program demonstrates how Rust prevents buffer overflows
 * and array bounds violations at compile time and runtime,
 * ensuring memory safety without performance overhead.
 */

fn demonstrate_buffer_safety() {
    // Rust arrays know their size and are bounds-checked
    let mut buffer: [u8; 10] = [0; 10];
    
    println!("Buffer size: {} bytes", buffer.len());
    
    // Safe string handling with automatic bounds checking
    let input = "This string is much longer than 10 characters and would overflow in C++!";
    println!("Input size: {} characters", input.len());
    
    // Rust prevents buffer overflow by using safe methods
    // Option 1: Take only what fits safely
    let safe_bytes = input.as_bytes();
    let copy_len = std::cmp::min(buffer.len(), safe_bytes.len());
    
    buffer[..copy_len].copy_from_slice(&safe_bytes[..copy_len]);
    
    println!("Safely copied {} bytes", copy_len);
    println!("Buffer contents: {:?}", &buffer);
    
    // Option 2: Use Vec<u8> for dynamic sizing
    let mut dynamic_buffer = Vec::new();
    dynamic_buffer.extend_from_slice(input.as_bytes());
    println!("Dynamic buffer size: {} bytes", dynamic_buffer.len());
}

fn array_bounds_safety() {
    let arr = [1, 2, 3, 4, 5];
    
    // Safe access using indexing
    println!("Valid access: arr[4] = {}", arr[4]);
    
    // Rust prevents bounds violations with runtime checks
    // These would panic with clear error messages:
    
    // println!("This would panic: arr[10] = {}", arr[10]);
    
    // Safe alternatives using get() method
    match arr.get(10) {
        Some(value) => println!("arr[10] = {}", value),
        None => println!("Index 10 is out of bounds - safely handled!"),
    }
    
    match arr.get(4) {
        Some(value) => println!("arr[4] = {} (safe access)", value),
        None => println!("Index 4 is out of bounds"),
    }
    
    // Iterators provide safe access to all elements
    println!("Safe iteration through array:");
    for (index, value) in arr.iter().enumerate() {
        println!("  arr[{}] = {}", index, value);
    }
}

fn slice_safety() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Safe slicing with bounds checking
    let safe_slice = &data[2..5];  // This is checked at runtime
    println!("Safe slice [2..5]: {:?}", safe_slice);
    
    // Using get() for optional slicing
    if let Some(slice) = data.get(2..5) {
        println!("Optional slice [2..5]: {:?}", slice);
    }
    
    // This would panic if uncommented (bounds checked):
    // let unsafe_slice = &data[2..20];
    
    // Safe alternative:
    let end_index = std::cmp::min(20, data.len());
    let safe_slice2 = &data[2..end_index];
    println!("Safe slice with clamped bounds [2..{}]: {:?}", end_index, safe_slice2);
}

// Demonstrate compile-time safety
fn compile_time_safety() {
    let arr = [1, 2, 3, 4, 5];
    
    // These would cause COMPILE-TIME ERRORS if uncommented:
    
    // let ptr = arr.as_ptr();
    // unsafe {
    //     // Even in unsafe blocks, Rust encourages explicit acknowledgment
    //     println!("Dangerous access: {}", *ptr.offset(100));
    // }
    
    // Safe iteration instead
    for item in &arr {
        println!("Safe access: {}", item);
    }
}

fn main() {
    println!("=== Rust Type Safe Buffer Operations ===");
    
    println!("\n1. Buffer Safety Demonstration:");
    demonstrate_buffer_safety();
    
    println!("\n2. Array Bounds Safety:");
    array_bounds_safety();
    
    println!("\n3. Slice Safety:");
    slice_safety();
    
    println!("\n4. Compile-time Safety:");
    compile_time_safety();
    
    println!("\nKey Points:");
    println!("- Rust prevents buffer overflows at compile time and runtime");
    println!("- Array bounds are always checked");
    println!("- Safe alternatives (get(), iterators) are provided");
    println!("- Performance is maintained through zero-cost abstractions");
    println!("- Unsafe operations require explicit 'unsafe' blocks");
}
