// 所有权转移版本
fn adds_hungary_ownership(mut country: String) {
    country.push_str("-Hungary");
    println!("所有权版本: {}", country);
    // country在这里被释放
}

// 可变借用版本
fn adds_hungary_borrow(country: &mut String) {
    country.push_str("-Hungary");
    println!("借用版本: {}", country);
    // country在这里不会被释放
}

fn main() {
    println!("=== 所有权转移示例 ===");
    
    // 测试所有权转移
    let mut country1 = String::from("Germany");
    println!("调用前: {}", country1);
    
    adds_hungary_ownership(country1);
    // ❌ 这里会编译错误！country1已经被移动
    // println!("调用后: {}", country1); // 编译错误！
    
    println!("\n=== 可变借用示例 ===");
    
    // 测试可变借用
    let mut country2 = String::from("Germany");
    println!("调用前: {}", country2);
    
    adds_hungary_borrow(&mut country2);
    // ✅ 这里可以正常使用！country2仍然有效
    println!("调用后: {}", country2);
    
    println!("\n=== 多次调用对比 ===");
    
    // 所有权转移 - 只能调用一次
    let country3 = String::from("France");
    adds_hungary_ownership(country3);
    // adds_hungary_ownership(country3); // ❌ 编译错误！
    
    // 可变借用 - 可以多次调用
    let mut country4 = String::from("France");
    adds_hungary_borrow(&mut country4);
    adds_hungary_borrow(&mut country4);
    adds_hungary_borrow(&mut country4);
    println!("最终结果: {}", country4);
}

// 演示函数返回值的区别
fn demo_return_values() {
    println!("\n=== 返回值演示 ===");
    
    // 所有权转移 - 可以返回修改后的值
    fn process_ownership(mut data: String) -> String {
        data.push_str("-processed");
        data // 返回修改后的数据
    }
    
    let original = String::from("data");
    let result = process_ownership(original);
    println!("处理结果: {}", result);
    
    // 可变借用 - 不需要返回值，直接修改原数据
    fn process_borrow(data: &mut String) {
        data.push_str("-processed");
        // 不需要返回值
    }
    
    let mut original2 = String::from("data");
    process_borrow(&mut original2);
    println!("处理结果: {}", original2);
} 