//! 基本使用示例
//! 
//! 这个示例展示了如何使用rtools库的各种功能

use rtools::{get_file_info, analyze_text_file, scan_directory, TextStats};

fn main() {
    println!("=== rtools 库使用示例 ===\n");
    
    // 示例1: 文件信息查看
    println!("1. 文件信息查看:");
    match get_file_info("src/main.rs") {
        Ok(info) => println!("{}", info),
        Err(e) => println!("错误: {}", e),
    }
    println!();
    
    // 示例2: 文本统计
    println!("2. 文本统计:");
    let sample_text = "Hello world! This is a sample text.\nIt has multiple lines.\n";
    let mut stats = TextStats::new();
    stats.analyze_text(sample_text);
    stats.print_stats();
    println!();
    
    // 示例3: 目录扫描
    println!("3. 目录扫描 (src目录):");
    match scan_directory("src", Some(1)) {
        Ok(stats) => {
            println!("总文件数: {}", stats.total_files);
            println!("总目录数: {}", stats.total_dirs);
            println!("总大小: {} 字节", stats.total_size);
        }
        Err(e) => println!("错误: {}", e),
    }
    println!();
    
    // 示例4: 错误处理
    println!("4. 错误处理示例:");
    match get_file_info("nonexistent_file.txt") {
        Ok(_) => println!("意外成功"),
        Err(e) => println!("预期的错误: {}", e),
    }
} 