// 1. 参数名前加 mut - 函数内部可变，但调用者不需要可变
fn adds_hungary_param_mut(mut country: String) {
    println!("函数内部修改前: {}", country);
    country.push_str("-Hungary");
    println!("函数内部修改后: {}", country);
    // country 在这里被释放
}

// 2. 参数类型前加 mut - 可变借用，调用者需要可变
fn adds_hungary_type_mut(country: &mut String) {
    println!("函数内部修改前: {}", country);
    country.push_str("-Hungary");
    println!("函数内部修改后: {}", country);
    // country 在这里不会被释放
}

// 3. 参数名前和类型前都加 mut - 可变借用，函数内部重新绑定为可变
fn adds_hungary_both_mut(mut country: &mut String) {
    println!("函数内部修改前: {}", country);
    country.push_str("-Hungary");
    println!("函数内部修改后: {}", country);
    // 可以重新绑定引用（但要注意生命周期）
    // country = &mut new_string; // 这会导致生命周期问题
}

fn main() {
    println!("=== 1. 参数名前加 mut ===");
    let country1 = String::from("Austria"); // 不需要 mut
    println!("调用前: {}", country1);
    adds_hungary_param_mut(country1);
    // println!("调用后: {}", country1); // ❌ 编译错误！country1已被移动
    
    println!("\n=== 2. 参数类型前加 mut ===");
    let mut country2 = String::from("Austria"); // 需要 mut
    println!("调用前: {}", country2);
    adds_hungary_type_mut(&mut country2);
    println!("调用后: {}", country2); // ✅ 可以继续使用
    
    println!("\n=== 3. 参数名前和类型前都加 mut ===");
    let mut country3 = String::from("Austria");
    println!("调用前: {}", country3);
    adds_hungary_both_mut(&mut country3);
    println!("调用后: {}", country3);
    
    println!("\n=== 4. 对比不同用法 ===");
    
    // 测试1: 参数名前加 mut
    let test1 = String::from("Test1");
    adds_hungary_param_mut(test1);
    // test1 在这里已经失效
    
    // 测试2: 参数类型前加 mut
    let mut test2 = String::from("Test2");
    adds_hungary_type_mut(&mut test2);
    println!("test2 仍然可用: {}", test2);
    
    // 测试3: 多次调用
    let mut test3 = String::from("Test3");
    adds_hungary_type_mut(&mut test3);
    adds_hungary_type_mut(&mut test3);
    adds_hungary_type_mut(&mut test3);
    println!("多次调用后: {}", test3);
}

// 演示在结构体方法中的应用
struct Country {
    name: String,
}

impl Country {
    // 方法1: self 前加 mut - 方法内部可以修改结构体
    fn add_suffix(&mut self, suffix: &str) {
        self.name.push_str(suffix);
    }
    
    // 方法2: 参数前加 mut - 方法内部可以修改参数
    fn process_name(mut name: String) -> String {
        name.push_str("-processed");
        name
    }
    
    // 方法3: 两者结合
    fn update_and_process(&mut self, mut new_name: String) {
        new_name.push_str("-updated");
        self.name = new_name;
    }
}

fn demo_struct_methods() {
    println!("\n=== 结构体方法中的 mut 用法 ===");
    
    let mut country = Country {
        name: String::from("Germany"),
    };
    
    println!("原始名称: {}", country.name);
    country.add_suffix("-Empire");
    println!("添加后缀后: {}", country.name);
    
    let processed_name = Country::process_name(String::from("France"));
    println!("处理后的名称: {}", processed_name);
    
    country.update_and_process(String::from("Italy"));
    println!("更新并处理后: {}", country.name);
} 