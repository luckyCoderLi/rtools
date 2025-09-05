use rtools::scan_directory;
use std::fs;
use std::path::Path;

#[test]
fn test_scan_directory() {
    // 创建测试目录结构
    let test_dir = "test_scan_dir";
    let sub_dir = format!("{}/subdir", test_dir);
    let test_file1 = format!("{}/file1.txt", test_dir);
    let test_file2 = format!("{}/file2.rs", sub_dir);
    
    fs::create_dir_all(&sub_dir).unwrap();
    fs::write(&test_file1, "Hello").unwrap();
    fs::write(&test_file2, "fn main() {}").unwrap();
    
    let result = scan_directory(test_dir, Some(2));
    assert!(result.is_ok());
    
    let stats = result.unwrap();
    assert_eq!(stats.total_files, 2);
    assert_eq!(stats.total_dirs, 1);
    assert!(stats.total_size > 0);
    
    // 检查文件类型统计
    assert_eq!(stats.extension_stats.get("txt"), Some(&1));
    assert_eq!(stats.extension_stats.get("rs"), Some(&1));
    
    // 检查最大文件列表
    assert_eq!(stats.largest_files.len(), 2);
    
    // 清理测试目录
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
fn test_scan_directory_with_depth_limit() {
    // 创建嵌套目录结构
    let test_dir = "test_nested_dir";
    let level1 = format!("{}/level1", test_dir);
    let level2 = format!("{}/level2", level1);
    let level3 = format!("{}/level3", level2);
    
    fs::create_dir_all(&level3).unwrap();
    fs::write(format!("{}/file.txt", test_dir), "test").unwrap();
    fs::write(format!("{}/file.txt", level1), "test").unwrap();
    fs::write(format!("{}/file.txt", level2), "test").unwrap();
    fs::write(format!("{}/file.txt", level3), "test").unwrap();
    
    // 测试深度限制为1
    let result = scan_directory(test_dir, Some(1));
    assert!(result.is_ok());
    
    let stats = result.unwrap();
    // 应该只包含根目录和level1的内容
    assert!(stats.total_files >= 2);
    
    // 清理测试目录
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
fn test_scan_nonexistent_directory() {
    let result = scan_directory("nonexistent_dir", None);
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(error.to_string().contains("目录不存在"));
}

#[test]
fn test_scan_file_as_directory() {
    // 创建一个文件
    let test_file = "test_file_for_scan.txt";
    fs::write(test_file, "test").unwrap();
    
    let result = scan_directory(test_file, None);
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(error.to_string().contains("路径不是目录"));
    
    // 清理测试文件
    fs::remove_file(test_file).unwrap();
} 