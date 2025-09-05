use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rtools::{
    get_file_info, analyze_text_file, scan_directory,
    search_files, SearchCriteria, analyze_log_file,
    ConfigManager, ConfigValue
};
use std::fs;

fn bench_fileinfo(c: &mut Criterion) {
    // 创建测试文件
    let test_file = "bench_test.txt";
    fs::write(test_file, "Hello, World! This is a benchmark test file.").unwrap();
    
    c.bench_function("fileinfo", |b| {
        b.iter(|| get_file_info(black_box(test_file)))
    });
    
    // 清理
    fs::remove_file(test_file).unwrap();
}

fn bench_textstats(c: &mut Criterion) {
    // 创建测试文件
    let test_content = "Hello world! This is a benchmark test.\n".repeat(100);
    let test_file = "bench_text.txt";
    fs::write(test_file, &test_content).unwrap();
    
    c.bench_function("textstats", |b| {
        b.iter(|| analyze_text_file(black_box(test_file)))
    });
    
    // 清理
    fs::remove_file(test_file).unwrap();
}

fn bench_dirscan(c: &mut Criterion) {
    c.bench_function("dirscan_src", |b| {
        b.iter(|| scan_directory(black_box("src"), Some(1)))
    });
}

fn bench_filesearch(c: &mut Criterion) {
    let criteria = SearchCriteria::new()
        .with_extension("rs".to_string())
        .with_max_depth(1);
    
    c.bench_function("filesearch", |b| {
        b.iter(|| search_files(black_box("src"), criteria.clone()))
    });
}

fn bench_loganalyzer(c: &mut Criterion) {
    // 创建测试日志文件
    let log_content = "[2024-01-15 10:30:00] [INFO] Test log entry\n".repeat(100);
    let test_log = "bench_log.log";
    fs::write(test_log, &log_content).unwrap();
    
    c.bench_function("loganalyzer", |b| {
        b.iter(|| analyze_log_file(black_box(test_log)))
    });
    
    // 清理
    fs::remove_file(test_log).unwrap();
}

fn bench_config(c: &mut Criterion) {
    // 创建测试配置文件
    let config_content = r#"{
        "app": {
            "name": "benchmark",
            "version": "1.0.0"
        },
        "database": {
            "host": "localhost",
            "port": 5432
        }
    }"#;
    let test_config = "bench_config.json";
    fs::write(test_config, config_content).unwrap();
    
    c.bench_function("config_load", |b| {
        b.iter(|| ConfigManager::load_from_file(black_box(test_config)))
    });
    
    // 清理
    fs::remove_file(test_config).unwrap();
}

criterion_group!(
    benches,
    bench_fileinfo,
    bench_textstats,
    bench_dirscan,
    bench_filesearch,
    bench_loganalyzer,
    bench_config
);
criterion_main!(benches); 