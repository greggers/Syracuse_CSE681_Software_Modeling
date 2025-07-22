/**
 * Rust Thread Safety Example - TYPE SAFE
 * 
 * This program demonstrates how Rust prevents data races at compile time
 * through its ownership system and Send/Sync traits, making concurrent
 * programming safe without runtime overhead.
 */

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

#[derive(Debug)]
struct SafeCounter {
    count: AtomicI32,
}

impl SafeCounter {
    fn new() -> Self {
        SafeCounter {
            count: AtomicI32::new(0),
        }
    }
    
    fn increment(&self) {
        // Atomic operation - no race condition possible
        self.count.fetch_add(1, Ordering::SeqCst);
    }
    
    fn get_count(&self) -> i32 {
        self.count.load(Ordering::SeqCst)
    }
}

#[derive(Debug)]
struct SharedData {
    data: Vec<i32>,
    sum: i32,
    processing: bool,
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            data: Vec::new(),
            sum: 0,
            processing: false,
        }
    }
    
    fn add_value(&mut self, value: i32) {
        self.data.push(value);
        self.sum += value;
        self.processing = !self.processing;
    }
    
    fn print_stats(&self) {
        println!("Data size: {}, Sum: {}, Processing: {}", 
                self.data.len(), self.sum, self.processing);
        
        print!("Data: ");
        for value in &self.data {
            print!("{} ", value);
        }
        println!();
    }
}

fn demonstrate_counter_safety() {
    println!("=== Safe Counter with Atomics ===");
    
    let counter = Arc::new(SafeCounter::new());
    let num_threads = 10;
    let increments_per_thread = 1000;
    
    let mut handles = vec![];
    
    // Launch threads that increment counter
    for _ in 0..num_threads {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                counter_clone.increment();  // SAFE: Atomic operation
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let expected = num_threads * increments_per_thread;
    let actual = counter.get_count();
    
    println!("Expected: {}", expected);
    println!("Actual: {}", actual);
    println!("Perfect accuracy - no lost increments!");
    
    assert_eq!(actual, expected, "Counter should be exact with atomic operations");
}

fn demonstrate_mutex_safety() {
    println!("\n=== Safe Shared Data with Mutex ===");
    
    let shared_data = Arc::new(Mutex::new(SharedData::new()));
    
    // Thread 1: Adds data safely
    let shared_data_writer = Arc::clone(&shared_data);
    let writer = thread::spawn(move || {
        for i in 0..10 {
            {
                let mut data = shared_data_writer.lock().unwrap();
                data.add_value(i);  // SAFE: Exclusive access via mutex
            }  // Lock automatically released here
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    // Thread 2: Reads data safely
    let shared_data_reader = Arc::clone(&shared_data);
    let reader = thread::spawn(move || {
        for _ in 0..5 {
            {
                let data = shared_data_reader.lock().unwrap();
                data.print_stats();  // SAFE: Exclusive access via mutex
            }  // Lock automatically released here
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    writer.join().unwrap();
    reader.join().unwrap();
    
    println!("Final stats (guaranteed consistent):");
    let final_data = shared_data.lock().unwrap();
    final_data.print_stats();
}

fn demonstrate_rwlock_safety() {
    println!("\n=== Safe Read-Write Access with RwLock ===");
    
    let shared_data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // Multiple reader threads - can run concurrently
    for i in 0..3 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let data = data_clone.read().unwrap();  // SAFE: Multiple readers allowed
            println!("Reader {}: Data length = {}", i, data.len());
            
            // Simulate some work
            thread::sleep(Duration::from_millis(100));
            
            println!("Reader {}: First element = {}", i, data[0]);
        });
        handles.push(handle);
    }
    
    // Single writer thread - must wait for all readers
    let data_writer = Arc::clone(&shared_data);
    let writer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        
        {
            let mut data = data_writer.write().unwrap();  // SAFE: Exclusive write access
            println!("Writer: Adding element");
            data.push(6);
        }  // Write lock released here
        
        println!("Writer: Done");
    });
    handles.push(writer_handle);
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_data = shared_data.read().unwrap();
    println!("Final data: {:?}", *final_data);
}

fn demonstrate_send_sync_traits() {
    println!("\n=== Send/Sync Trait Safety ===");
    
    // Types that implement Send can be moved between threads
    // Types that implement Sync can be shared between threads
    
    #[derive(Debug)]
    struct NotSync {
        // This type is not Sync - cannot be shared between threads
        data: std::rc::Rc<i32>,
    }
    
    let not_sync = NotSync {
        data: std::rc::Rc::new(42),
    };
    
    // This would cause COMPILE ERROR if uncommented:
    // let handle = thread::spawn(move || {
    //     println!("{:?}", not_sync);  // Error: Rc is not Send
    // });
    
    // Safe alternatives
    let thread_safe_data = Arc::new(42);
    let data_clone = Arc::clone(&thread_safe_data);
    
    let handle = thread::spawn(move || {
        println!("Thread safe data: {}", data_clone);  // SAFE: Arc implements Send+Sync
    });
    
    handle.join().unwrap();
    println!("Original data: {}", thread_safe_data);
}

fn demonstrate_channel_safety() {
    println!("\n=== Safe Message Passing with Channels ===");
    
    use std::sync::mpsc;
    
    let (sender, receiver) = mpsc::channel();
    
    // Producer thread
    let producer = thread::spawn(move || {
        for i in 0..5 {
            sender.send(format!("Message {}", i)).unwrap();  // SAFE: Ownership transferred
            thread::sleep(Duration::from_millis(100));
        }
        // sender is dropped here, signaling end of messages
    });
    
    // Consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(message) = receiver.recv() {  // SAFE: Exclusive ownership
            println!("Received: {}", message);
        }
        println!("All messages received");
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

fn demonstrate_scoped_threads() {
    println!("\n=== Safe Scoped Thread Access ===");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    // Scoped threads can borrow local data safely
    thread::scope(|s| {
        // Spawn thread that reads data
        let reader = s.spawn(|| {
            println!("Reader: Data = {:?}", data);  // SAFE: Borrow guaranteed valid
        });
        
        // Spawn thread that modifies data (requires mutable borrow)
        // This would cause COMPILE ERROR if both threads tried to access mutably:
        // let writer = s.spawn(|| {
        //     data.push(6);  // Error: cannot borrow as mutable
        // });
        
        reader.join().unwrap();
        // All scoped threads finish before scope ends
    });
    
    // Now we can safely modify data
    data.push(6);
    println!("After scoped threads: {:?}", data);
}

fn demonstrate_atomic_operations() {
    println!("\n=== Safe Atomic Operations ===");
    
    let counter = Arc::new(AtomicUsize::new(0));
    let flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    
    let mut handles = vec![];
    
    // Multiple threads doing atomic operations
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let flag_clone = Arc::clone(&flag);
        
        let handle = thread::spawn(move || {
            // Atomic increment
            let old_value = counter_clone.fetch_add(1, Ordering::SeqCst);
            println!("Thread {}: Incremented from {}", i, old_value);
            
            // Atomic compare-and-swap
            if old_value == 2 {
                flag_clone.store(true, Ordering::SeqCst);
                println!("Thread {}: Set flag to true", i);
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter: {}", counter.load(Ordering::SeqCst));
    println!("Final flag: {}", flag.load(Ordering::SeqCst));
}

// Demonstrate that data races are impossible at compile time
fn demonstrate_compile_time_safety() {
    println!("\n=== Compile-time Race Prevention ===");
    
    let mut data = vec![1, 2, 3];
    
    // These would cause COMPILE ERRORS if uncommented:
    
    // Example 1: Cannot share mutable reference
    // let handle = thread::spawn(|| {
    //     data.push(4);  // Error: captured variable cannot be sent between threads safely
    // });
    
    // Example 2: Cannot have multiple mutable references
    // let ref1 = &mut data;
    // let ref2 = &mut data;  // Error: cannot borrow as mutable more than once
    
    // Example 3: Cannot mix mutable and immutable references
    // let immutable_ref = &data;
    // let mutable_ref = &mut data;  // Error: cannot borrow as mutable
    
    // Safe alternative: Use Arc<Mutex<T>>
    let safe_data = Arc::new(Mutex::new(data));
    let safe_data_clone = Arc::clone(&safe_data);
    
    let handle = thread::spawn(move || {
        let mut guard = safe_data_clone.lock().unwrap();
        guard.push(4);  // SAFE: Exclusive access guaranteed
    });
    
    handle.join().unwrap();
    
    let final_data = safe_data.lock().unwrap();
    println!("Safely modified data: {:?}", *final_data);
}

fn main() {
    println!("=== Rust Thread Safety Guarantees ===");
    
    demonstrate_counter_safety();
    demonstrate_mutex_safety();
    demonstrate_rwlock_safety();
    demonstrate_send_sync_traits();
    demonstrate_channel_safety();
    demonstrate_scoped_threads();
    demonstrate_atomic_operations();
    demonstrate_compile_time_safety();
    
    println!("\nRust Threading Safety Summary:");
    println!("- Data races prevented at COMPILE TIME");
    println!("- Send/Sync traits ensure thread safety");
    println!("- Ownership system prevents shared mutable state");
    println!("- Safe alternatives: Arc, Mutex, RwLock, channels");
    println!("- Atomic operations for lock-free programming");
    println!("- Scoped threads for borrowing local data");
    println!("- Zero runtime overhead for safety guarantees");
    println!("- Impossible to accidentally create race conditions");
}
