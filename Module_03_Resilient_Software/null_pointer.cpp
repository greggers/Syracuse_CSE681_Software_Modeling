/**
 * C++ Null Pointer Example - TYPE UNSAFE
 * 
 * This program demonstrates how C++ allows dangerous null pointer
 * dereferences that can cause crashes, undefined behavior, and
 * security vulnerabilities.
 */

#include <iostream>
#include <memory>
#include <vector>

class Resource {
public:
    int id;
    std::string name;
    
    Resource(int i, const std::string& n) : id(i), name(n) {
        std::cout << "Created Resource: " << name << " (id: " << id << ")" << std::endl;
    }
    
    void process() {
        std::cout << "Processing resource: " << name << " (id: " << id << ")" << std::endl;
    }
    
    ~Resource() {
        std::cout << "Destroyed Resource: " << name << std::endl;
    }
};

Resource* find_resource_by_id(const std::vector<Resource*>& resources, int target_id) {
    for (Resource* res : resources) {
        if (res && res->id == target_id) {
            return res;
        }
    }
    return nullptr;  // Return null if not found
}

void demonstrate_null_pointer_dereference() {
    std::vector<Resource*> resources;
    resources.push_back(new Resource(1, "Database"));
    resources.push_back(new Resource(2, "FileSystem"));
    resources.push_back(nullptr);  // Oops, null pointer in collection
    resources.push_back(new Resource(3, "Network"));
    
    // Search for existing resource
    Resource* found = find_resource_by_id(resources, 2);
    if (found) {
        found->process();  // This works
    }
    
    // Search for non-existing resource
    Resource* not_found = find_resource_by_id(resources, 999);
    
    // DANGEROUS: No null check - will crash!
    std::cout << "Attempting to use null pointer..." << std::endl;
    not_found->process();  // Segmentation fault / access violation
    
    // Clean up (we won't reach here due to crash above)
    for (Resource* res : resources) {
        delete res;
    }
}

void demonstrate_uninitialized_pointer() {
    Resource* uninitialized;  // Contains garbage value
    
    // DANGEROUS: Using uninitialized pointer
    std::cout << "Using uninitialized pointer..." << std::endl;
    uninitialized->process();  // Undefined behavior - may crash
}

void demonstrate_smart_pointer_nulls() {
    std::shared_ptr<Resource> smart_ptr;  // Default initialized to nullptr
    
    // Check if null (good practice)
    if (smart_ptr) {
        smart_ptr->process();
    } else {
        std::cout << "Smart pointer is null" << std::endl;
    }
    
    // But forgetting to check is still dangerous
    std::shared_ptr<Resource> another_ptr = nullptr;
    
    // DANGEROUS: Forgetting null check
    another_ptr->process();  // Will throw std::bad_weak_ptr or crash
}

void demonstrate_method_returning_null() {
    class ResourceManager {
    public:
        Resource* get_critical_resource() {
            // Simulate failure to acquire resource
            if (rand() % 2 == 0) {
                return nullptr;  // 50% chance of returning null
            }
            return new Resource(999, "Critical");
        }
    };
    
    ResourceManager manager;
    Resource* critical = manager.get_critical_resource();
    
    // DANGEROUS: Assuming non-null return value
    critical->process();  // May crash if null was returned
    
    delete critical;  // Also dangerous if critical is null
}

void demonstrate_null_in_calculations() {
    Resource* ptr = nullptr;
    
    // Pointer arithmetic with null pointer - undefined behavior
    Resource* offset_ptr = ptr + 1;  // DANGEROUS
    
    // Attempting to use the offset pointer
    if (offset_ptr) {  // This check doesn't help - still undefined behavior
        offset_ptr->process();
    }
    
    // Converting null to integer
    std::cout << "Null pointer as integer: " << reinterpret_cast<uintptr_t>(ptr) << std::endl;
}

void demonstrate_array_of_pointers() {
    Resource* resources[5];  // Uninitialized array of pointers
    
    // Initialize some elements
    resources[0] = new Resource(1, "First");
    resources[2] = new Resource(3, "Third");
    // resources[1], resources[3], resources[4] remain uninitialized
    
    // DANGEROUS: Iterating without checking for null/uninitialized
    for (int i = 0; i < 5; i++) {
        std::cout << "Processing resource " << i << std::endl;
        resources[i]->process();  // Will crash on uninitialized elements
    }
    
    // Clean up initialized elements
    delete resources[0];
    delete resources[2];
}

int main() {
    std::cout << "=== C++ Null Pointer Vulnerabilities ===" << std::endl;
    
    try {
        std::cout << "\n1. Null Pointer Dereference:" << std::endl;
        demonstrate_null_pointer_dereference();
        
    } catch (...) {
        std::cout << "Caught exception from null pointer dereference!" << std::endl;
    }
    
    try {
        std::cout << "\n2. Uninitialized Pointer:" << std::endl;
        demonstrate_uninitialized_pointer();
        
    } catch (...) {
        std::cout << "Caught exception from uninitialized pointer!" << std::endl;
    }
    
    try {
        std::cout << "\n3. Smart Pointer Nulls:" << std::endl;
        demonstrate_smart_pointer_nulls();
        
    } catch (...) {
        std::cout << "Caught exception from smart pointer!" << std::endl;
    }
    
    try {
        std::cout << "\n4. Method Returning Null:" << std::endl;
        demonstrate_method_returning_null();
        
    } catch (...) {
        std::cout << "Caught exception from null return!" << std::endl;
    }
    
    try {
        std::cout << "\n5. Null in Calculations:" << std::endl;
        demonstrate_null_in_calculations();
        
    } catch (...) {
        std::cout << "Caught exception from null calculations!" << std::endl;
    }
    
    try {
        std::cout << "\n6. Array of Pointers:" << std::endl;
        demonstrate_array_of_pointers();
        
    } catch (...) {
        std::cout << "Caught exception from pointer array!" << std::endl;
    }
    
    std::cout << "\nNote: C++ provides no automatic protection against null pointer errors." << std::endl;
    std::cout << "Null pointer dereferences are a major source of crashes and security vulnerabilities." << std::endl;
    std::cout << "Developers must manually check for null in all cases." << std::endl;
    
    return 0;
}
