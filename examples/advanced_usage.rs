//! 高级使用示例
//! 
//! 这个示例展示了如何使用rtools库的所有功能

use rtools::{
    get_file_info, analyze_text_file, scan_directory,
    search_files, SearchCriteria, analyze_log_file,
    ConfigManager, ConfigValue
};

fn main() {
    println!("=== rtools 高级使用示例 ===\n");
    
    // 示例1: 文件搜索
    println!("1. 文件搜索示例:");
    let criteria = SearchCriteria::new()
        .with_extension("rs".to_string())
        .with_max_depth(1);
    
    match search_files("src", criteria) {
        Ok(result) => {
            println!("找到 {} 个Rust文件", result.total_count);
            for file in result.files.iter().take(3) {
                println!("  - {}", file.file_name().unwrap().to_str().unwrap());
            }
        }
        Err(e) => println!("搜索错误: {}", e),
    }
    println!();
    
    // 示例2: 日志分析
    println!("2. 日志分析示例:");
    match analyze_log_file("example.log") {
        Ok(analysis) => {
            println!("日志条目数: {}", analysis.total_entries);
            if let Some((start, end)) = analysis.time_range {
                println!("时间范围: {} 到 {}", 
                         start.format("%H:%M:%S"), 
                         end.format("%H:%M:%S"));
            }
            
            // 显示错误级别统计
            for (level, count) in &analysis.level_distribution {
                if level.severity() >= 3 { // Error级别及以上
                    println!("  {:?}: {}次", level, count);
                }
            }
        }
        Err(e) => println!("日志分析错误: {}", e),
    }
    println!();
    
    // 示例3: 配置管理
    println!("3. 配置管理示例:");
    match ConfigManager::load_from_file("example_config.json") {
        Ok(config) => {
            println!("配置项数量: {}", config.len());
            
            // 获取嵌套配置
            if let Some(app_name) = config.get("app.name") {
                println!("应用名称: {:?}", app_name);
            }
            
            if let Some(db_port) = config.get("database.port") {
                println!("数据库端口: {:?}", db_port);
            }
            
            // 显示所有顶级键
            println!("顶级配置项:");
            for key in config.keys() {
                println!("  - {}", key);
            }
        }
        Err(e) => println!("配置加载错误: {}", e),
    }
    println!();
    
    // 示例4: 创建新配置
    println!("4. 创建新配置示例:");
    let mut new_config = ConfigManager::new();
    new_config.set("server.host".to_string(), ConfigValue::String("127.0.0.1".to_string()));
    new_config.set("server.port".to_string(), ConfigValue::Integer(8080));
    new_config.set("server.ssl".to_string(), ConfigValue::Boolean(true));
    new_config.set("server.timeout".to_string(), ConfigValue::Float(30.5));
    
    match new_config.save_to_file("new_config.json") {
        Ok(_) => println!("新配置已保存到 new_config.json"),
        Err(e) => println!("保存配置错误: {}", e),
    }
    println!();
    
    // 示例5: 综合使用
    println!("5. 综合使用示例:");
    
    // 扫描目录
    match scan_directory("src", Some(1)) {
        Ok(stats) => {
            println!("src目录统计:");
            println!("  文件数: {}", stats.total_files);
            println!("  总大小: {:.2} KB", stats.total_size as f64 / 1024.0);
            
            // 搜索大文件
            let large_file_criteria = SearchCriteria::new()
                .with_size_range(Some(1000), None); // 大于1KB的文件
            
            match search_files("src", large_file_criteria) {
                Ok(search_result) => {
                    println!("  大文件数: {}", search_result.total_count);
                }
                Err(e) => println!("  搜索错误: {}", e),
            }
        }
        Err(e) => println!("目录扫描错误: {}", e),
    }
    
    println!("\n=== 示例完成 ===");
} 