/**
 * C++ Use After Free Example - TYPE UNSAFE
 * 
 * This program demonstrates dangerous use-after-free vulnerabilities
 * that are possible in C++ due to manual memory management.
 * These can lead to crashes, corruption, or security exploits.
 */

#include <iostream>
#include <memory>
#include <vector>

class DataHolder {
public:
    int value;
    std::string name;
    
    DataHolder(int v, const std::string& n) : value(v), name(n) {
        std::cout << "Created DataHolder: " << name << " = " << value << std::endl;
    }
    
    ~DataHolder() {
        std::cout << "Destroyed DataHolder: " << name << std::endl;
    }
    
    void print() {
        std::cout << "DataHolder " << name << " has value: " << value << std::endl;
    }
};

void demonstrate_use_after_free() {
    DataHolder* ptr = new DataHolder(42, "dangerous");
    
    // Use the object normally
    ptr->print();
    
    // Delete the object
    delete ptr;
    std::cout << "Object deleted!" << std::endl;
    
    // DANGEROUS: Use after free - undefined behavior!
    std::cout << "Attempting to use deleted object..." << std::endl;
    ptr->print();  // This may crash, corrupt memory, or appear to work
    
    // Even more dangerous - modifying freed memory
    ptr->value = 999;
    std::cout << "Modified freed memory: " << ptr->value << std::endl;
    
    // ptr is now a "dangling pointer" pointing to freed memory
}

void demonstrate_double_free() {
    DataHolder* ptr = new DataHolder(123, "double_trouble");
    
    ptr->print();
    
    // Delete once
    delete ptr;
    
    // DANGEROUS: Delete again - undefined behavior!
    delete ptr;  // Double free - may crash or corrupt heap
}

void demonstrate_dangling_reference() {
    DataHolder* original = new DataHolder(456, "original");
    DataHolder* copy = original;  // Both point to same object
    
    original->print();
    copy->print();
    
    // Delete through original pointer
    delete original;
    
    // DANGEROUS: copy is now a dangling pointer
    std::cout << "Accessing through dangling pointer:" << std::endl;
    copy->print();  // Undefined behavior
}

void demonstrate_vector_invalidation() {
    std::vector<DataHolder> vec;
    vec.emplace_back(1, "first");
    vec.emplace_back(2, "second");
    
    // Get pointer to first element
    DataHolder* ptr = &vec[0];
    ptr->print();
    
    // Force vector reallocation (this invalidates ptr)
    for (int i = 0; i < 100; i++) {
        vec.emplace_back(i, "item" + std::to_string(i));
    }
    
    // DANGEROUS: ptr now points to freed memory
    std::cout << "Accessing invalidated pointer:" << std::endl;
    ptr->print();  // Undefined behavior - vector reallocated
}

void demonstrate_smart_pointer_issues() {
    std::shared_ptr<DataHolder> shared1 = std::make_shared<DataHolder>(789, "shared");
    
    // Get raw pointer from shared_ptr
    DataHolder* raw = shared1.get();
    
    // Reset shared_ptr (destroys object if this was the last reference)
    shared1.reset();
    
    // DANGEROUS: raw pointer now points to destroyed object
    std::cout << "Accessing raw pointer after shared_ptr reset:" << std::endl;
    raw->print();  // Use after free
}

int main() {
    std::cout << "=== C++ Use After Free Vulnerabilities ===" << std::endl;
    
    try {
        std::cout << "\n1. Basic Use After Free:" << std::endl;
        demonstrate_use_after_free();
        
        std::cout << "\n2. Double Free:" << std::endl;
        demonstrate_double_free();
        
        std::cout << "\n3. Dangling Reference:" << std::endl;
        demonstrate_dangling_reference();
        
        std::cout << "\n4. Vector Invalidation:" << std::endl;
        demonstrate_vector_invalidation();
        
        std::cout << "\n5. Smart Pointer Issues:" << std::endl;
        demonstrate_smart_pointer_issues();
        
    } catch (...) {
        std::cout << "Program crashed due to memory safety violation!" << std::endl;
    }
    
    std::cout << "\nNote: This program demonstrates serious memory safety issues." << std::endl;
    std::cout << "C++ provides no automatic protection against use-after-free bugs." << std::endl;
    std::cout << "These vulnerabilities are a major source of security exploits." << std::endl;
    
    return 0;
}
