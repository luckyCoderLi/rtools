use std::fs;
use std::path::Path;
use crate::{RtoolsResult, RtoolsError};

/// 获取文件的详细信息
pub fn get_file_info(path: &str) -> RtoolsResult<String> {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        return Err(RtoolsError::FileNotFound(path.to_string()));
    }
    
    let metadata = fs::metadata(path_obj)?;
    let file_size = metadata.len();
    let is_file = metadata.is_file();
    let is_dir = metadata.is_dir();

    path_obj.file_name().
    let file_name = path_obj.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("未知文件名");
    
    let extension = path_obj.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("无扩展名");
    
    let modified_time = metadata.modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    
    let info = format!(
        "文件信息:\n\
         - 名称: {}\n\
         - 路径: {}\n\
         - 大小: {} 字节\n\
         - 类型: {}\n\
         - 扩展名: {}\n\
         - 修改时间: {}",
        file_name,
        path,
        file_size,
        if is_file { "文件" } else if is_dir { "目录" } else { "其他" },
        extension,
        modified_time
    );
    
    Ok(info)
} 