#[cfg(test)]
mod test_path {
    use std::path::{Path, PathBuf};

    #[test]
    fn test_path() {
        // 从字符串转为path
        let path = Path::new("src/file/test_txt.txt");
        let from_path = Path::new(&path);
        assert_eq!(path, from_path);
        // 获取path的父路径
        let parent = path.parent().unwrap();
        assert_eq!(parent, Path::new("src/file"));

        let file_name = path.file_stem().unwrap();
        assert_eq!(file_name, "test_txt");
    }

    #[test]
    fn test_path_buf() {
        // 创建一个空的 `PathBuf`
        let empty_path = PathBuf::new();
        assert_eq!(empty_path.to_str().unwrap(), "");

        // 根据字符串切片创建 `PathBuf`
        let path = PathBuf::from(r"src/file/test_txt.txt");
        assert_eq!(path.as_path(), Path::new("src/file/test_txt.txt"));
        assert_eq!(path.to_str().unwrap(), "src/file/test_txt.txt");
    }
}
