use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::{RtoolsResult, RtoolsError};

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub modified: SystemTime,
    pub extension: Option<String>,
}

#[derive(Debug, Default)]
pub struct DirectoryStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
    pub extension_stats: HashMap<String, usize>,
    pub largest_files: Vec<FileInfo>,
    pub oldest_files: Vec<FileInfo>,
}

impl DirectoryStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_file(&mut self, file_info: FileInfo) {
        if file_info.is_dir {
            self.total_dirs += 1;
        } else {
            self.total_files += 1;
            self.total_size += file_info.size;
            
            // 统计文件扩展名
            if let Some(ext) = &file_info.extension {
                *self.extension_stats.entry(ext.clone()).or_insert(0) += 1;
            }
            
            // 优化：只在需要时克隆
            self.update_largest_files(&file_info);
            self.update_oldest_files(&file_info);
        }
    }
    
    fn update_largest_files(&mut self, file_info: &FileInfo) {
        // 如果列表未满，直接添加
        if self.largest_files.len() < 10 {
            self.largest_files.push(file_info.clone());
            self.largest_files.sort_by(|a, b| b.size.cmp(&a.size));
        } else {
            // 如果新文件比最小的还大，替换最小的
            if let Some(smallest) = self.largest_files.iter_mut().min_by_key(|f| f.size) {
                if file_info.size > smallest.size {
                    *smallest = file_info.clone();
                    self.largest_files.sort_by(|a, b| b.size.cmp(&a.size));
                }
            }
        }
    }
    
    fn update_oldest_files(&mut self, file_info: &FileInfo) {
        // 如果列表未满，直接添加
        if self.oldest_files.len() < 10 {
            self.oldest_files.push(file_info.clone());
            self.oldest_files.sort_by(|a, b| a.modified.cmp(&b.modified));
        } else {
            // 如果新文件比最新的还旧，替换最新的
            if let Some(newest) = self.oldest_files.iter_mut().max_by_key(|f| f.modified) {
                if file_info.modified < newest.modified {
                    *newest = file_info.clone();
                    self.oldest_files.sort_by(|a, b| a.modified.cmp(&b.modified));
                }
            }
        }
    }
    
    pub fn print_stats(&self) {
        println!("目录统计信息:");
        println!("- 总文件数: {}", self.total_files);
        println!("- 总目录数: {}", self.total_dirs);
        println!("- 总大小: {} 字节 ({:.2} MB)", 
                 self.total_size, 
                 self.total_size as f64 / 1024.0 / 1024.0);
        
        if !self.extension_stats.is_empty() {
            println!("\n文件类型统计:");
            let mut sorted_extensions: Vec<(&String, &usize)> = self.extension_stats.iter().collect();
            sorted_extensions.sort_by(|a, b| b.1.cmp(a.1));
            
            for (ext, count) in sorted_extensions.iter().take(10) {
                println!("  .{}: {}个文件", ext, count);
            }
        }
        
        if !self.largest_files.is_empty() {
            println!("\n最大的10个文件:");
            for (i, file) in self.largest_files.iter().enumerate() {
                println!("  {}. {} ({} 字节)", 
                         i + 1, 
                         file.name, 
                         file.size);
            }
        }
        
        if !self.oldest_files.is_empty() {
            println!("\n最旧的10个文件:");
            for (i, file) in self.oldest_files.iter().enumerate() {
                let duration = SystemTime::now()
                    .duration_since(file.modified)
                    .unwrap_or_default();
                println!("  {}. {} ({}天前)", 
                         i + 1, 
                         file.name, 
                         duration.as_secs() / 86400);
            }
        }
    }
}

impl Clone for FileInfo {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            path: self.path.clone(),
            size: self.size,
            is_dir: self.is_dir,
            modified: self.modified,
            extension: self.extension.clone(),
        }
    }
}

pub fn scan_directory(dir_path: &str, max_depth: Option<usize>) -> RtoolsResult<DirectoryStats> {
    let path = Path::new(dir_path);
    
    if !path.exists() {
        return Err(RtoolsError::DirectoryNotFound(dir_path.to_string()));
    }
    
    if !path.is_dir() {
        return Err(RtoolsError::NotADirectory(dir_path.to_string()));
    }
    
    let mut stats = DirectoryStats::new();
    scan_directory_recursive(path, &mut stats, 0, max_depth.unwrap_or(usize::MAX))?;
    
    Ok(stats)
}

fn scan_directory_recursive(
    dir_path: &Path, 
    stats: &mut DirectoryStats, 
    current_depth: usize, 
    max_depth: usize
) -> RtoolsResult<()> {
    if current_depth > max_depth {
        return Ok(());
    }
    
    let entries = fs::read_dir(dir_path)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        let metadata = fs::metadata(&path)?;
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("未知")
            .to_string();
        
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string());
        
        let file_info = FileInfo {
            name,
            path: path.clone(),
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            modified: metadata.modified()?,
            extension,
        };
        
        stats.add_file(file_info);
        
        // 递归扫描子目录
        if metadata.is_dir() && current_depth < max_depth {
            scan_directory_recursive(&path, stats, current_depth + 1, max_depth)?;
        }
    }
    
    Ok(())
} 