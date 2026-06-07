use jinja2rs::SandboxedEnvironment;
use serde_json::json;

fn main() {
    let env = SandboxedEnvironment::new();
    
    // Test 1: underscore prefix access
    let result1 = env.render_str("{{ obj._private }}", json!({"obj": {"_private": "secret"}}));
    println!("Test 1 - underscore prefix: {:?}", result1);
    
    // Test 2: chained underscore access
    let result2 = env.render_str("{{ obj.subobj._private }}", 
                                  json!({"obj": {"subobj": {"_private": "secret"}}}));
    println!("Test 2 - chained underscore: {:?}", result2);
    
    // Test 3: division by zero
    let result3 = env.render_str("{{ 1 / 0 }}", json!({}));
    println!("Test 3 - division by zero: {:?}", result3);
    
    // Test 4: Safe underscore like _private not in obj
    let result4 = env.render_str("{{ obj._private }}", json!({"obj": {"a": 1}}));
    println!("Test 4 - missing underscore attr: {:?}", result4);
}
