use rtools::{analyze_text_file, TextStats};
use std::fs;

#[test]
fn test_analyze_text_file() {
    // 创建测试文件
    let test_content = "Hello world! This is a test file.\nIt has multiple lines.\n";
    let test_file = "test_text.txt";
    fs::write(test_file, test_content).unwrap();
    
    let result = analyze_text_file(test_file);
    assert!(result.is_ok());
    
    let stats = result.unwrap();
    assert_eq!(stats.line_count, 2);
    assert_eq!(stats.word_count, 11); // "Hello world! This is a test file.\nIt has multiple lines.\n" 有11个单词
    assert!(stats.char_count > 0);
    assert!(stats.byte_count > 0);
    
    // 检查词频统计
    assert!(stats.word_frequency.contains_key("hello"));
    assert!(stats.word_frequency.contains_key("world"));
    
    // 清理测试文件
    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_text_stats_analyze_text() {
    let mut stats = TextStats::new();
    let test_text = "Hello world! This is a test.";
    
    stats.analyze_text(test_text);
    
    assert_eq!(stats.word_count, 6);
    assert_eq!(stats.line_count, 1);
    assert!(stats.char_count > 0);
    assert!(stats.byte_count > 0);
    assert!(stats.avg_word_length > 0.0);
    
    // 检查词频
    assert_eq!(stats.word_frequency.get("hello"), Some(&1));
    assert_eq!(stats.word_frequency.get("world"), Some(&1));
}

#[test]
fn test_text_stats_empty_text() {
    let mut stats = TextStats::new();
    stats.analyze_text("");
    
    assert_eq!(stats.word_count, 0);
    assert_eq!(stats.line_count, 0);
    assert_eq!(stats.char_count, 0);
    assert_eq!(stats.byte_count, 0);
    assert_eq!(stats.avg_word_length, 0.0);
    assert!(stats.word_frequency.is_empty());
} 