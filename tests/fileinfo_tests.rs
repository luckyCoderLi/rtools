use rtools::get_file_info;
use std::fs;
use std::path::Path;

#[test]
fn test_get_file_info_existing_file() {
    // 创建一个临时文件用于测试
    let test_file = "test_file.txt";
    fs::write(test_file, "Hello, World!").unwrap();
    
    let result = get_file_info(test_file);
    assert!(result.is_ok());
    
    let info = result.unwrap();
    assert!(info.contains("test_file.txt"));
    assert!(info.contains("文件"));
    assert!(info.contains("13 字节")); // "Hello, World!" 是13个字节
    
    // 清理测试文件
    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_get_file_info_nonexistent_file() {
    let result = get_file_info("nonexistent_file.txt");
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(error.to_string().contains("文件不存在"));
}

#[test]
fn test_get_file_info_directory() {
    let test_dir = "test_dir";
    fs::create_dir(test_dir).unwrap();
    
    let result = get_file_info(test_dir);
    assert!(result.is_ok());
    
    let info = result.unwrap();
    assert!(info.contains("目录"));
    
    // 清理测试目录
    fs::remove_dir(test_dir).unwrap();
} 