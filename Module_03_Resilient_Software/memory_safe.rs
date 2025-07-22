/**
 * Rust Memory Safety Example - TYPE SAFE
 * 
 * This program demonstrates how Rust's ownership system prevents
 * use-after-free, double-free, and dangling pointer bugs at compile time.
 * These safety guarantees come with zero runtime overhead.
 */

#[derive(Debug)]
struct DataHolder {
    value: i32,
    name: String,
}

impl DataHolder {
    fn new(value: i32, name: &str) -> Self {
        println!("Created DataHolder: {} = {}", name, value);
        DataHolder {
            value,
            name: name.to_string(),
        }
    }
    
    fn print(&self) {
        println!("DataHolder {} has value: {}", self.name, self.value);
    }
}

impl Drop for DataHolder {
    fn drop(&mut self) {
        println!("Destroyed DataHolder: {}", self.name);
    }
}

fn demonstrate_ownership_safety() {
    let data = DataHolder::new(42, "safe");
    data.print();
    
    // Transfer ownership
    let moved_data = data;
    moved_data.print();
    
    // This would cause a COMPILE ERROR if uncommented:
    // data.print();  // Error: value borrowed here after move
    
    println!("Ownership transferred safely - no use-after-free possible!");
    
    // When moved_data goes out of scope, it's automatically cleaned up
}

fn demonstrate_borrowing_safety() {
    let data = DataHolder::new(123, "borrowed");
    
    // Borrow immutably
    let borrowed_ref = &data;
    borrowed_ref.print();
    data.print();  // Original still usable
    
    // Mutable borrowing
    let mut mutable_data = DataHolder::new(456, "mutable");
    {
        let mutable_ref = &mut mutable_data;
        mutable_ref.value = 999;
        mutable_ref.print();
        
        // This would cause COMPILE ERROR if uncommented:
        // mutable_data.print();  // Error: cannot borrow as immutable while borrowed as mutable
    }
    
    // Now we can use mutable_data again
    mutable_data.print();
    
    println!("Borrowing rules prevent data races and use-after-free!");
}

fn demonstrate_lifetime_safety() {
    let long_lived = DataHolder::new(789, "long_lived");
    
    let reference_to_long_lived = {
        let short_lived = DataHolder::new(100, "short_lived");
        
        // This would cause COMPILE ERROR if we tried to return a reference to short_lived:
        // &short_lived  // Error: borrowed value does not live long enough
        
        &long_lived  // This is fine - long_lived outlives this scope
    };
    
    // We can safely use the reference because the compiler verified lifetimes
    reference_to_long_lived.print();
    
    println!("Lifetime analysis prevents dangling pointers!");
}

fn demonstrate_rc_safety() {
    use std::rc::Rc;
    
    let shared_data = Rc::new(DataHolder::new(555, "shared"));
    
    {
        let another_ref = Rc::clone(&shared_data);
        another_ref.print();
        
        println!("Reference count: {}", Rc::strong_count(&shared_data));
        
        // another_ref goes out of scope here, but data is still alive
    }
    
    println!("Reference count: {}", Rc::strong_count(&shared_data));
    shared_data.print();
    
    // Data is automatically freed when last Rc goes out of scope
    println!("Reference counting prevents premature deallocation!");
}

fn demonstrate_box_safety() {
    let heap_data = Box::new(DataHolder::new(333, "heap_allocated"));
    heap_data.print();
    
    // Transfer ownership
    let moved_box = heap_data;
    moved_box.print();
    
    // This would cause COMPILE ERROR:
    // heap_data.print();  // Error: value borrowed here after move
    
    // No double-free possible - only one owner at a time
    println!("Box ownership prevents double-free errors!");
}

fn demonstrate_vector_safety() {
    let mut vec = Vec::new();
    vec.push(DataHolder::new(1, "first"));
    vec.push(DataHolder::new(2, "second"));
    
    // Safe iteration
    for item in &vec {
        item.print();
    }
    
    // Get reference to first element
    let first_ref = &vec[0];
    
    // This would cause COMPILE ERROR if we tried to modify vec while holding reference:
    // vec.push(DataHolder::new(3, "third"));  // Error: cannot borrow as mutable
    
    first_ref.print();  // Use the reference
    
    // Now we can modify again
    vec.push(DataHolder::new(3, "third"));
    
    println!("Borrow checker prevents iterator invalidation!");
}

// Demonstrate that even unsafe code requires explicit acknowledgment
fn demonstrate_unsafe_blocks() {
    let data = DataHolder::new(777, "unsafe_demo");
    
    // To do potentially dangerous operations, you must use 'unsafe' blocks
    // This makes the danger explicit and localized
    unsafe {
        let ptr: *const DataHolder = &data;
        
        // Even in unsafe blocks, the compiler helps where possible
        // Raw pointer dereferencing requires explicit unsafe
        println!("Unsafe access: {:?}", (*ptr).value);
    }
    
    // The vast majority of Rust code doesn't need unsafe blocks
    println!("Unsafe operations are explicit and isolated!");
}

fn main() {
    println!("=== Rust Memory Safety Guarantees ===");
    
    println!("\n1. Ownership Safety:");
    demonstrate_ownership_safety();
    
    println!("\n2. Borrowing Safety:");
    demonstrate_borrowing_safety();
    
    println!("\n3. Lifetime Safety:");
    demonstrate_lifetime_safety();
    
    println!("\n4. Reference Counting Safety:");
    demonstrate_rc_safety();
    
    println!("\n5. Box Ownership Safety:");
    demonstrate_box_safety();
    
    println!("\n6. Vector Safety:");
    demonstrate_vector_safety();
    
    println!("\n7. Unsafe Blocks:");
    demonstrate_unsafe_blocks();
    
    println!("\nKey Safety Guarantees:");
    println!("- No use-after-free: Ownership prevents using moved values");
    println!("- No double-free: Only one owner can free memory");
    println!("- No dangling pointers: Lifetime analysis ensures references are valid");
    println!("- No data races: Borrowing rules prevent concurrent access violations");
    println!("- Zero overhead: All safety checks happen at compile time");
    println!("- Explicit unsafe: Dangerous operations require explicit acknowledgment");
}
