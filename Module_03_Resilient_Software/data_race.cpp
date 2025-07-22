/**
 * C++ Data Race Example - TYPE UNSAFE
 * 
 * This program demonstrates dangerous data races that can occur
 * in C++ when multiple threads access shared data without proper
 * synchronization. These lead to undefined behavior and corruption.
 */

#include <iostream>
#include <thread>
#include <vector>
#include <atomic>
#include <mutex>
#include <chrono>

class UnsafeCounter {
private:
    int count = 0;
    
public:
    void increment() {
        // DANGEROUS: Non-atomic read-modify-write operation
        int temp = count;  // Read
        temp++;            // Modify  
        count = temp;      // Write
        // Another thread can interfere between these operations!
    }
    
    int get_count() const {
        return count;  // DANGEROUS: Concurrent read while writing
    }
};

class SharedData {
public:
    std::vector<int> data;
    int sum = 0;
    bool processing = false;
    
    void add_value(int value) {
        // DANGEROUS: Multiple operations without synchronization
        data.push_back(value);     // Can corrupt vector internal state
        sum += value;              // Race condition on sum
        processing = !processing;   // Race condition on flag
    }
    
    void print_stats() {
        // DANGEROUS: Reading while other thread modifies
        std::cout << "Data size: " << data.size() 
                  << ", Sum: " << sum 
                  << ", Processing: " << processing << std::endl;
        
        // Even worse - iterating while vector is being modified
        for (int value : data) {
            std::cout << value << " ";
        }
        std::cout << std::endl;
    }
};

void demonstrate_counter_race() {
    std::cout << "=== Counter Race Condition ===" << std::endl;
    
    UnsafeCounter counter;
    const int num_threads = 10;
    const int increments_per_thread = 1000;
    
    std::vector<std::thread> threads;
    
    // Launch threads that increment counter
    for (int i = 0; i < num_threads; i++) {
        threads.emplace_back([&counter, increments_per_thread]() {
            for (int j = 0; j < increments_per_thread; j++) {
                counter.increment();  // DANGEROUS: Race condition
            }
        });
    }
    
    // Wait for all threads to complete
    for (auto& t : threads) {
        t.join();
    }
    
    int expected = num_threads * increments_per_thread;
    int actual = counter.get_count();
    
    std::cout << "Expected: " << expected << std::endl;
    std::cout << "Actual: " << actual << std::endl;
    std::cout << "Lost increments: " << (expected - actual) << std::endl;
    
    if (actual != expected) {
        std::cout << "DATA RACE DETECTED: Lost increments due to race condition!" << std::endl;
    }
}

void demonstrate_shared_data_race() {
    std::cout << "\n=== Shared Data Race Condition ===" << std::endl;
    
    SharedData shared;
    
    // Thread 1: Adds data
    std::thread writer([&shared]() {
        for (int i = 0; i < 100; i++) {
            shared.add_value(i);  // DANGEROUS: Multiple races
            std::this_thread::sleep_for(std::chrono::microseconds(10));
        }
    });
    
    // Thread 2: Reads data
    std::thread reader([&shared]() {
        for (int i = 0; i < 20; i++) {
            shared.print_stats();  // DANGEROUS: Reading while writing
            std::this_thread::sleep_for(std::chrono::microseconds(50));
        }
    });
    
    writer.join();
    reader.join();
    
    std::cout << "Final stats (may be corrupted):" << std::endl;
    shared.print_stats();
}

void demonstrate_pointer_race() {
    std::cout << "\n=== Pointer Race Condition ===" << std::endl;
    
    int* shared_ptr = nullptr;
    bool data_ready = false;
    
    // Thread 1: Allocates and initializes data
    std::thread producer([&shared_ptr, &data_ready]() {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        
        shared_ptr = new int(42);  // DANGEROUS: Setting pointer
        data_ready = true;         // DANGEROUS: Setting flag
        // Race: Another thread might see data_ready=true but shared_ptr=nullptr
    });
    
    // Thread 2: Consumes data
    std::thread consumer([&shared_ptr, &data_ready]() {
        while (!data_ready) {  // DANGEROUS: Checking flag
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }
        
        // DANGEROUS: Pointer might still be nullptr due to race
        std::cout << "Data value: " << *shared_ptr << std::endl;  // Potential crash
        
        delete shared_ptr;  // DANGEROUS: What if producer is still using it?
        shared_ptr = nullptr;
    });
    
    producer.join();
    consumer.join();
}

void demonstrate_false_sharing() {
    std::cout << "\n=== False Sharing Performance Issue ===" << std::endl;
    
    struct alignas(64) CacheLinePadded {
        int counter1 = 0;
        char padding[60];  // Pad to cache line size
        int counter2 = 0;
    };
    
    struct Unpadded {
        int counter1 = 0;
        int counter2 = 0;  // These will share cache line
    };
    
    // False sharing example
    Unpadded shared_counters;
    
    auto start = std::chrono::high_resolution_clock::now();
    
    std::thread t1([&shared_counters]() {
        for (int i = 0; i < 10000000; i++) {
            shared_counters.counter1++;  // DANGEROUS: False sharing
        }
    });
    
    std::thread t2([&shared_counters]() {
        for (int i = 0; i < 10000000; i++) {
            shared_counters.counter2++;  // DANGEROUS: False sharing
        }
    });
    
    t1.join();
    t2.join();
    
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    
    std::cout << "Time with false sharing: " << duration.count() << "ms" << std::endl;
    std::cout << "Counter1: " << shared_counters.counter1 << std::endl;
    std::cout << "Counter2: " << shared_counters.counter2 << std::endl;
}

void demonstrate_aba_problem() {
    std::cout << "\n=== ABA Problem ===" << std::endl;
    
    struct Node {
        int value;
        Node* next;
        Node(int v) : value(v), next(nullptr) {}
    };
    
    Node* head = new Node(1);
    head->next = new Node(2);
    
    // Thread 1: Remove head, then add it back
    std::thread t1([&head]() {
        Node* old_head = head;
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        
        // Remove head
        head = head->next;
        
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        
        // Add it back (ABA problem)
        old_head->next = head;
        head = old_head;
    });
    
    // Thread 2: Tries to access what it thinks is the original head
    std::thread t2([&head]() {
        Node* observed_head = head;
        std::this_thread::sleep_for(std::chrono::milliseconds(150));
        
        // DANGEROUS: head might have changed and changed back (ABA)
        if (head == observed_head) {
            std::cout << "Head appears unchanged, but ABA might have occurred!" << std::endl;
            std::cout << "Value: " << head->value << std::endl;
        }
    });
    
    t1.join();
    t2.join();
    
    // Cleanup
    while (head) {
        Node* temp = head;
        head = head->next;
        delete temp;
    }
}

int main() {
    std::cout << "=== C++ Data Race Vulnerabilities ===" << std::endl;
    
    demonstrate_counter_race();
    demonstrate_shared_data_race();
    demonstrate_pointer_race();
    demonstrate_false_sharing();
    demonstrate_aba_problem();
    
    std::cout << "\nSummary of C++ Threading Dangers:" << std::endl;
    std::cout << "- No automatic protection against data races" << std::endl;
    std::cout << "- Manual synchronization required (mutexes, atomics)" << std::endl;
    std::cout << "- Easy to forget synchronization in some code paths" << std::endl;
    std::cout << "- Performance issues from false sharing" << std::endl;
    std::cout << "- Complex problems like ABA can occur" << std::endl;
    std::cout << "- Debugging race conditions is extremely difficult" << std::endl;
    
    return 0;
}
