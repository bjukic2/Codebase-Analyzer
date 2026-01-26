// use walkdir::WalkDir;

// pub fn collect_files(path: &str) -> Vec<String> {
//     WalkDir::new(path)
//         .into_iter()
//         .filter_map(|e| e.ok())
//         .filter(|e| e.file_type().is_file())
//         .map(|e| e.path().display().to_string())
//         .collect()
// }
