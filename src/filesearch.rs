
use std::fs;
use std::path::{Path, PathBuf};
use crate::{RtoolsResult, RtoolsError};

/// 搜索条件
#[derive(Debug, Clone, Default)]
pub struct SearchCriteria {
    pub name_pattern: Option<String>,
    pub extension: Option<String>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub max_depth: Option<usize>,
}

impl SearchCriteria {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_name_pattern(mut self, pattern: String) -> Self {
        self.name_pattern = Some(pattern);
        self
    }
    
    pub fn with_extension(mut self, ext: String) -> Self {
        self.extension = Some(ext);
        self
    }
    
    pub fn with_size_range(mut self, min: Option<u64>, max: Option<u64>) -> Self {
        self.min_size = min;
        self.max_size = max;
        self
    }
    
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }
}

/// 搜索结果
#[derive(Debug, Default)]
pub struct SearchResult {
    pub files: Vec<PathBuf>,
    pub total_count: usize,
    pub total_size: u64,
    pub search_time_ms: u128,
}

impl SearchResult {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn print_results(&self) {
        println!("搜索结果:");
        println!("- 找到文件数: {}", self.total_count);
        println!("- 总大小: {} 字节 ({:.2} MB)", 
                 self.total_size, 
                 self.total_size as f64 / 1024.0 / 1024.0);
        println!("- 搜索耗时: {} ms", self.search_time_ms);
        
        if !self.files.is_empty() {
            println!("\n找到的文件:");
            for (i, file) in self.files.iter().enumerate() {
                let file_name = file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("未知");
                println!("  {}. {}", i + 1, file_name);
            }
        }
    }
}

/// 在指定目录中搜索文件
pub fn search_files(dir_path: &str, criteria: SearchCriteria) -> RtoolsResult<SearchResult> {
    let start_time = std::time::Instant::now();
    let path = Path::new(dir_path);
    
    if !path.exists() {
        return Err(RtoolsError::DirectoryNotFound(dir_path.to_string()));
    }
    
    if !path.is_dir() {
        return Err(RtoolsError::NotADirectory(dir_path.to_string()));
    }
    
    let mut result = SearchResult::new();
    let max_depth = criteria.max_depth.unwrap_or(usize::MAX);
    
    search_files_recursive(path, &criteria, &mut result, 0, max_depth)?;
    
    result.search_time_ms = start_time.elapsed().as_millis();
    result.total_count = result.files.len();
    
    Ok(result)
}

fn search_files_recursive(
    dir_path: &Path,
    criteria: &SearchCriteria,
    result: &mut SearchResult,
    current_depth: usize,
    max_depth: usize,
) -> RtoolsResult<()> {
    if current_depth > max_depth {
        return Ok(());
    }
    
    let entries = fs::read_dir(dir_path)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if matches_criteria(&path, criteria)? {
                result.files.push(path.clone());
                let metadata = fs::metadata(&path)?;
                result.total_size += metadata.len();
            }
        } else if path.is_dir() && current_depth < max_depth {
            search_files_recursive(&path, criteria, result, current_depth + 1, max_depth)?;
        }
    }
    
    Ok(())
}

fn matches_criteria(path: &Path, criteria: &SearchCriteria) -> RtoolsResult<bool> {
    // 首先检查扩展名（最快）
    if let Some(ref expected_ext) = criteria.extension {
        if let Some(actual_ext) = path.extension() {
            if let Some(ext_str) = actual_ext.to_str() {
                if ext_str.to_lowercase() != expected_ext.to_lowercase() {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
    }
    
    // 然后检查文件名模式
    if let Some(ref pattern) = criteria.name_pattern {
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                if !name_str.to_lowercase().contains(&pattern.to_lowercase()) {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
    }
    
    // 最后检查文件大小（需要文件系统访问）
    if criteria.min_size.is_some() || criteria.max_size.is_some() {
        let metadata = fs::metadata(path)?;
        let file_size = metadata.len();
        
        if let Some(min_size) = criteria.min_size {
            if file_size < min_size {
                return Ok(false);
            }
        }
        
        if let Some(max_size) = criteria.max_size {
            if file_size > max_size {
                return Ok(false);
            }
        }
    }
    
    Ok(true)
} 