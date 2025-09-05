use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::{RtoolsResult, RtoolsError};

#[derive(Debug)]
pub struct TextStats {
    pub char_count: usize,
    pub word_count: usize,
    pub line_count: usize,
    pub byte_count: usize,
    pub word_frequency: HashMap<String, usize>,
    pub avg_word_length: f64,
}

impl Default for TextStats {
    fn default() -> Self {
        Self {
            char_count: 0,
            word_count: 0,
            line_count: 0,
            byte_count: 0,
            word_frequency: HashMap::new(),
            avg_word_length: 0.0,
        }
    }
}

impl TextStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn analyze_text(&mut self, text: &str) {
        self.byte_count = text.len();
        self.char_count = text.chars().count();
        self.line_count = text.lines().count();
        
        // 使用迭代器优化，避免多次分配
        let mut total_length = 0;
        let mut word_count = 0;
        
        for word in text.split_whitespace() {
            if !word.is_empty() {
                word_count += 1;
                
                // 清理单词并计算长度
                let clean_word: String = word
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect();
                
                if !clean_word.is_empty() {
                    total_length += clean_word.len();
                    
                    // 转换为小写并统计词频
                    let clean_word_lower = clean_word.to_lowercase();
                    *self.word_frequency.entry(clean_word_lower).or_insert(0) += 1;
                }
            }
        }
        
        self.word_count = word_count;
        
        // 计算平均词长
        if self.word_count > 0 {
            self.avg_word_length = total_length as f64 / self.word_count as f64;
        }
    }
    
    pub fn print_stats(&self) {
        println!("文本统计信息:");
        println!("- 字符数: {}", self.char_count);
        println!("- 单词数: {}", self.word_count);
        println!("- 行数: {}", self.line_count);
        println!("- 字节数: {}", self.byte_count);
        println!("- 平均词长: {:.2}", self.avg_word_length);
        
        if !self.word_frequency.is_empty() {
            println!("\n最常用的10个单词:");
            let mut sorted_words: Vec<(&String, &usize)> = self.word_frequency.iter().collect();
            sorted_words.sort_by(|a, b| b.1.cmp(a.1));
            
            for (word, count) in sorted_words.iter().take(10) {
                println!("  {}: {}次", word, count);
            }
        }
    }
}

pub fn analyze_file(file_path: &str) -> RtoolsResult<TextStats> {
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err(RtoolsError::FileNotFound(file_path.to_string()));
    }
    
    let content = fs::read_to_string(path)?;
    let mut stats = TextStats::new();
    stats.analyze_text(&content);
    
    Ok(stats)
} 