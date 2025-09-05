use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::{RtoolsResult, RtoolsError};

/// 配置值类型
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<ConfigValue>),
    Map(HashMap<String, ConfigValue>),
}

impl ConfigValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            ConfigValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ConfigValue::Integer(i) => Some(*i),
            ConfigValue::String(s) => s.parse().ok(),
            _ => None,
        }
    }
    
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ConfigValue::Float(f) => Some(*f),
            ConfigValue::String(s) => s.parse().ok(),
            _ => None,
        }
    }
    
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            ConfigValue::Boolean(b) => Some(*b),
            ConfigValue::String(s) => match s.to_lowercase().as_str() {
                "true" | "yes" | "1" => Some(true),
                "false" | "no" | "0" => Some(false),
                _ => None,
            },
            _ => None,
        }
    }
    
    pub fn as_list(&self) -> Option<&Vec<ConfigValue>> {
        match self {
            ConfigValue::List(l) => Some(l),
            _ => None,
        }
    }
    
    pub fn as_map(&self) -> Option<&HashMap<String, ConfigValue>> {
        match self {
            ConfigValue::Map(m) => Some(m),
            _ => None,
        }
    }
}

/// 配置管理器
#[derive(Debug, Default)]
pub struct ConfigManager {
    data: HashMap<String, ConfigValue>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 从文件加载配置
    pub fn load_from_file(file_path: &str) -> RtoolsResult<Self> {
        let path = Path::new(file_path);
        
        if !path.exists() {
            return Err(RtoolsError::FileNotFound(file_path.to_string()));
        }
        
        let content = fs::read_to_string(path)?;
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        let data = match extension.as_str() {
            "json" => Self::parse_json(&content)?,
            "toml" => Self::parse_toml(&content)?,
            "ini" | "cfg" => Self::parse_ini(&content)?,
            _ => Self::parse_key_value(&content)?,
        };
        
        Ok(Self {
            data,
        })
    }
    
    /// 保存配置到文件
    pub fn save_to_file(&self, file_path: &str) -> RtoolsResult<()> {
        let path = Path::new(file_path);
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        let content = match extension.as_str() {
            "json" => Self::to_json(&self.data)?,
            "toml" => Self::to_toml(&self.data)?,
            _ => Self::to_key_value(&self.data)?,
        };
        
        fs::write(path, content)?;
        Ok(())
    }
    
    /// 获取配置值
    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        if key.contains('.') {
            // 处理嵌套键，如 "app.name"
            let parts: Vec<&str> = key.split('.').collect();
            let mut current = &self.data;
            
            for (i, part) in parts.iter().enumerate() {
                if let Some(value) = current.get(*part) {
                    if i == parts.len() - 1 {
                        return Some(value);
                    } else if let Some(map) = value.as_map() {
                        current = map;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            None
        } else {
            self.data.get(key)
        }
    }
    
    /// 设置配置值
    pub fn set(&mut self, key: String, value: ConfigValue) {
        self.data.insert(key, value);
    }
    
    /// 删除配置项
    pub fn remove(&mut self, key: &str) -> Option<ConfigValue> {
        self.data.remove(key)
    }
    
    /// 检查是否存在配置项
    pub fn has_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    
    /// 获取所有键
    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }
    
    /// 获取配置项数量
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// 打印配置内容
    pub fn print_config(&self) {
        println!("配置内容:");
        if self.is_empty() {
            println!("  (空)");
            return;
        }
        
        for (key, value) in &self.data {
            Self::print_value(key, value, 1);
        }
    }
    
    fn print_value(key: &str, value: &ConfigValue, indent: usize) {
        let indent_str = "  ".repeat(indent);
        match value {
            ConfigValue::String(s) => println!("{}{} = \"{}\"", indent_str, key, s),
            ConfigValue::Integer(i) => println!("{}{} = {}", indent_str, key, i),
            ConfigValue::Float(f) => println!("{}{} = {}", indent_str, key, f),
            ConfigValue::Boolean(b) => println!("{}{} = {}", indent_str, key, b),
            ConfigValue::List(list) => {
                println!("{}{} = [", indent_str, key);
                for item in list {
                    Self::print_value("", item, indent + 1);
                }
                println!("{}]", indent_str);
            }
            ConfigValue::Map(map) => {
                println!("{}{} = {{", indent_str, key);
                for (k, v) in map {
                    Self::print_value(k, v, indent + 1);
                }
                println!("{}}}", indent_str);
            }
        }
    }
    
    /// 解析JSON格式
    fn parse_json(content: &str) -> RtoolsResult<HashMap<String, ConfigValue>> {
        let json_value: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| RtoolsError::ParseError(format!("JSON解析错误: {}", e)))?;
        
        Self::json_to_config_value(json_value)
            .and_then(|v| v.as_map().cloned())
            .ok_or_else(|| RtoolsError::ParseError("无效的JSON格式".to_string()))
    }
    
    /// 解析TOML格式
    fn parse_toml(content: &str) -> RtoolsResult<HashMap<String, ConfigValue>> {
        let toml_value: toml::Value = toml::from_str(content)
            .map_err(|e| RtoolsError::ParseError(format!("TOML解析错误: {}", e)))?;
        
        Self::toml_to_config_value(toml_value)
            .and_then(|v| v.as_map().cloned())
            .ok_or_else(|| RtoolsError::ParseError("无效的TOML格式".to_string()))
    }
    
    /// 解析INI格式
    fn parse_ini(content: &str) -> RtoolsResult<HashMap<String, ConfigValue>> {
        let mut data = HashMap::new();
        let mut current_section = String::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len()-1].to_string();
                continue;
            }
            
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let value = line[pos+1..].trim();
                
                let full_key = if current_section.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", current_section, key)
                };
                
                data.insert(full_key, ConfigValue::String(value.to_string()));
            }
        }
        
        Ok(data)
    }
    
    /// 解析键值对格式
    fn parse_key_value(content: &str) -> RtoolsResult<HashMap<String, ConfigValue>> {
        let mut data = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let value = line[pos+1..].trim();
                
                // 尝试解析不同类型的值
                let config_value = if let Ok(i) = value.parse::<i64>() {
                    ConfigValue::Integer(i)
                } else if let Ok(f) = value.parse::<f64>() {
                    ConfigValue::Float(f)
                } else if let Ok(b) = value.parse::<bool>() {
                    ConfigValue::Boolean(b)
                } else {
                    ConfigValue::String(value.to_string())
                };
                
                data.insert(key.to_string(), config_value);
            }
        }
        
        Ok(data)
    }
    
    /// 转换为JSON格式
    fn to_json(data: &HashMap<String, ConfigValue>) -> RtoolsResult<String> {
        let json_value = Self::config_value_to_json(&ConfigValue::Map(data.clone()));
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| RtoolsError::ParseError(format!("JSON序列化错误: {}", e)))
    }
    
    /// 转换为TOML格式
    fn to_toml(data: &HashMap<String, ConfigValue>) -> RtoolsResult<String> {
        let toml_value = Self::config_value_to_toml(&ConfigValue::Map(data.clone()));
        toml::to_string_pretty(&toml_value)
            .map_err(|e| RtoolsError::ParseError(format!("TOML序列化错误: {}", e)))
    }
    
    /// 转换为键值对格式
    fn to_key_value(data: &HashMap<String, ConfigValue>) -> RtoolsResult<String> {
        let mut lines = Vec::new();
        
        for (key, value) in data {
            let value_str = match value {
                ConfigValue::String(s) => format!("\"{}\"", s),
                ConfigValue::Integer(i) => i.to_string(),
                ConfigValue::Float(f) => f.to_string(),
                ConfigValue::Boolean(b) => b.to_string(),
                ConfigValue::List(_) => "[列表]".to_string(),
                ConfigValue::Map(_) => "{映射}".to_string(),
            };
            lines.push(format!("{} = {}", key, value_str));
        }
        
        Ok(lines.join("\n"))
    }
    
    // 辅助函数：JSON转换
    fn json_to_config_value(value: serde_json::Value) -> Option<ConfigValue> {
        match value {
            serde_json::Value::String(s) => Some(ConfigValue::String(s)),
            serde_json::Value::Number(n) => {
                n.as_i64()
                    .map(ConfigValue::Integer)
                    .or_else(|| n.as_f64().map(ConfigValue::Float))
            }
            serde_json::Value::Bool(b) => Some(ConfigValue::Boolean(b)),
            serde_json::Value::Array(arr) => {
                let mut list = Vec::new();
                for item in arr {
                    if let Some(cv) = Self::json_to_config_value(item) {
                        list.push(cv);
                    }
                }
                Some(ConfigValue::List(list))
            }
            serde_json::Value::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    if let Some(cv) = Self::json_to_config_value(v) {
                        map.insert(k, cv);
                    }
                }
                Some(ConfigValue::Map(map))
            }
            serde_json::Value::Null => None,
        }
    }
    
    fn config_value_to_json(value: &ConfigValue) -> serde_json::Value {
        match value {
            ConfigValue::String(s) => serde_json::Value::String(s.clone()),
            ConfigValue::Integer(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            ConfigValue::Float(f) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap_or_else(|| serde_json::Number::from(0)))
            }
            ConfigValue::Boolean(b) => serde_json::Value::Bool(*b),
            ConfigValue::List(list) => {
                let arr: Vec<serde_json::Value> = list.iter().map(Self::config_value_to_json).collect();
                serde_json::Value::Array(arr)
            }
            ConfigValue::Map(map) => {
                let obj: serde_json::Map<String, serde_json::Value> = map
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::config_value_to_json(v)))
                    .collect();
                serde_json::Value::Object(obj)
            }
        }
    }
    
    // 辅助函数：TOML转换
    fn toml_to_config_value(value: toml::Value) -> Option<ConfigValue> {
        match value {
            toml::Value::String(s) => Some(ConfigValue::String(s)),
            toml::Value::Integer(i) => Some(ConfigValue::Integer(i)),
            toml::Value::Float(f) => Some(ConfigValue::Float(f)),
            toml::Value::Boolean(b) => Some(ConfigValue::Boolean(b)),
            toml::Value::Array(arr) => {
                let mut list = Vec::new();
                for item in arr {
                    if let Some(cv) = Self::toml_to_config_value(item) {
                        list.push(cv);
                    }
                }
                Some(ConfigValue::List(list))
            }
            toml::Value::Table(table) => {
                let mut map = HashMap::new();
                for (k, v) in table {
                    if let Some(cv) = Self::toml_to_config_value(v) {
                        map.insert(k, cv);
                    }
                }
                Some(ConfigValue::Map(map))
            }
            toml::Value::Datetime(_) => None, // 暂时不支持日期时间
        }
    }
    
    fn config_value_to_toml(value: &ConfigValue) -> toml::Value {
        match value {
            ConfigValue::String(s) => toml::Value::String(s.clone()),
            ConfigValue::Integer(i) => toml::Value::Integer(*i),
            ConfigValue::Float(f) => toml::Value::Float(*f),
            ConfigValue::Boolean(b) => toml::Value::Boolean(*b),
            ConfigValue::List(list) => {
                let arr: Vec<toml::Value> = list.iter().map(Self::config_value_to_toml).collect();
                toml::Value::Array(arr)
            }
            ConfigValue::Map(map) => {
                let table: toml::Table = map
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::config_value_to_toml(v)))
                    .collect();
                toml::Value::Table(table)
            }
        }
    }
} 