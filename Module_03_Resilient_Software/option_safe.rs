/**
 * Rust Option Safety Example - TYPE SAFE
 * 
 * This program demonstrates how Rust eliminates null pointer exceptions
 * through the Option<T> type system, making null checks mandatory
 * and preventing null pointer dereferences at compile time.
 */

#[derive(Debug)]
struct Resource {
    id: i32,
    name: String,
}

impl Resource {
    fn new(id: i32, name: &str) -> Self {
        println!("Created Resource: {} (id: {})", name, id);
        Resource {
            id,
            name: name.to_string(),
        }
    }
    
    fn process(&self) {
        println!("Processing resource: {} (id: {})", self.name, self.id);
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Destroyed Resource: {}", self.name);
    }
}

// Function that might not find a resource - returns Option<T>
fn find_resource_by_id(resources: &[Resource], target_id: i32) -> Option<&Resource> {
    resources.iter().find(|res| res.id == target_id)
}

fn demonstrate_option_safety() {
    let resources = vec![
        Resource::new(1, "Database"),
        Resource::new(2, "FileSystem"),
        Resource::new(3, "Network"),
    ];
    
    // Search for existing resource
    match find_resource_by_id(&resources, 2) {
        Some(resource) => {
            println!("Found resource!");
            resource.process();
        },
        None => {
            println!("Resource not found");
        }
    }
    
    // Search for non-existing resource
    match find_resource_by_id(&resources, 999) {
        Some(resource) => {
            resource.process();
        },
        None => {
            println!("Resource 999 not found - safely handled!");
        }
    }
    
    // The compiler FORCES us to handle the None case
    // This would cause COMPILE ERROR if uncommented:
    // let found = find_resource_by_id(&resources, 999);
    // found.process();  // Error: cannot call method on Option<&Resource>
}

fn demonstrate_option_methods() {
    let resources = vec![
        Resource::new(10, "Cache"),
        Resource::new(20, "Logger"),
    ];
    
    // Using if let for cleaner syntax
    if let Some(resource) = find_resource_by_id(&resources, 10) {
        resource.process();
    } else {
        println!("Resource not found with if let");
    }
    
    // Using unwrap_or_else for default behavior
    let resource_or_default = find_resource_by_id(&resources, 999)
        .unwrap_or_else(|| {
            println!("Using default resource");
            &Resource::new(0, "Default")
        });
    
    // Using map to transform the Option
    let resource_name = find_resource_by_id(&resources, 20)
        .map(|res| &res.name)
        .unwrap_or(&"Unknown".to_string());
    
    println!("Resource name: {}", resource_name);
    
    // Using and_then for chaining operations
    let processed = find_resource_by_id(&resources, 10)
        .and_then(|res| {
            if res.id > 5 {
                Some(format!("Processed: {}", res.name))
            } else {
                None
            }
        });
    
    match processed {
        Some(msg) => println!("{}", msg),
        None => println!("Processing conditions not met"),
    }
}

fn demonstrate_result_safety() {
    // Result<T, E> for operations that can fail with error information
    fn try_create_resource(id: i32, name: &str) -> Result<Resource, String> {
        if id <= 0 {
            Err("Invalid ID: must be positive".to_string())
        } else if name.is_empty() {
            Err("Invalid name: cannot be empty".to_string())
        } else {
            Ok(Resource::new(id, name))
        }
    }
    
    // Handle Result with match
    match try_create_resource(5, "ValidResource") {
        Ok(resource) => {
            println!("Successfully created resource");
            resource.process();
        },
        Err(error) => {
            println!("Failed to create resource: {}", error);
        }
    }
    
    // Handle error case
    match try_create_resource(-1, "InvalidResource") {
        Ok(resource) => resource.process(),
        Err(error) => println!("Creation failed: {}", error),
    }
    
    // Using unwrap_or_else with Result
    let resource = try_create_resource(0, "")
        .unwrap_or_else(|_| Resource::new(1, "Fallback"));
    
    resource.process();
}

fn demonstrate_option_collections() {
    // Vec<Option<T>> for collections that might contain missing values
    let maybe_resources: Vec<Option<Resource>> = vec![
        Some(Resource::new(1, "First")),
        None,  // Missing resource
        Some(Resource::new(3, "Third")),
        None,  // Another missing resource
        Some(Resource::new(5, "Fifth")),
    ];
    
    // Safe iteration over Option values
    for (index, maybe_resource) in maybe_resources.iter().enumerate() {
        match maybe_resource {
            Some(resource) => {
                println!("Slot {}: Found resource", index);
                resource.process();
            },
            None => {
                println!("Slot {}: Empty slot", index);
            }
        }
    }
    
    // Filter out None values and collect Some values
    let existing_resources: Vec<&Resource> = maybe_resources
        .iter()
        .filter_map(|opt| opt.as_ref())
        .collect();
    
    println!("Found {} existing resources", existing_resources.len());
    
    // Using flatten to remove None values
    let resource_names: Vec<&String> = maybe_resources
        .iter()
        .flatten()  // Removes None values
        .map(|res| &res.name)
        .collect();
    
    println!("Resource names: {:?}", resource_names);
}

fn demonstrate_no_null_dereference() {
    // Rust has no null pointers - only Option<T>
    let maybe_resource: Option<Resource> = None;
    
    // This is IMPOSSIBLE to compile - no direct access to value:
    // maybe_resource.process();  // COMPILE ERROR: cannot call method
    
    // Must explicitly handle the None case
    match maybe_resource {
        Some(resource) => resource.process(),
        None => println!("No resource to process - safely handled!"),
    }
    
    // Even with references, no null pointers exist
    let resources = vec![Resource::new(100, "Safe")];
    let resource_ref: &Resource = &resources[0];  // Always valid
    
    // No way to create a "null reference" in safe Rust
    resource_ref.process();  // Always safe
}

fn demonstrate_option_chaining() {
    struct Container {
        resource: Option<Resource>,
    }
    
    impl Container {
        fn get_resource_name(&self) -> Option<&String> {
            self.resource.as_ref().map(|res| &res.name)
        }
        
        fn get_resource_id(&self) -> Option<i32> {
            self.resource.as_ref().map(|res| res.id)
        }
    }
    
    let containers = vec![
        Container { resource: Some(Resource::new(1, "First")) },
        Container { resource: None },
        Container { resource: Some(Resource::new(3, "Third")) },
    ];
    
    for (index, container) in containers.iter().enumerate() {
        // Safe chaining of Option operations
        let info = container.get_resource_name()
            .zip(container.get_resource_id())
            .map(|(name, id)| format!("Resource '{}' has ID {}", name, id))
            .unwrap_or_else(|| "No resource in container".to_string());
        
        println!("Container {}: {}", index, info);
    }
}

fn main() {
    println!("=== Rust Option Safety System ===");
    
    println!("\n1. Basic Option Safety:");
    demonstrate_option_safety();
    
    println!("\n2. Option Methods:");
    demonstrate_option_methods();
    
    println!("\n3. Result Safety:");
    demonstrate_result_safety();
    
    println!("\n4. Option Collections:");
    demonstrate_option_collections();
    
    println!("\n5. No Null Dereference Possible:");
    demonstrate_no_null_dereference();
    
    println!("\n6. Option Chaining:");
    demonstrate_option_chaining();
    
    println!("\nKey Safety Features:");
    println!("- No null pointers exist in safe Rust");
    println!("- Option<T> makes absence explicit and type-safe");
    println!("- Compiler forces handling of None cases");
    println!("- Result<T, E> provides rich error information");
    println!("- Method chaining allows safe composition");
    println!("- Zero runtime overhead - all checks at compile time");
    println!("- Impossible to accidentally dereference null");
}
