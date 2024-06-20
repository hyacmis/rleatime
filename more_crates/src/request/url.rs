#[cfg(test)]
mod test_url {
    use url::Url;

    #[test]
    fn test_url_path() {
        let url_path =
            "https://github.com/usebruno/bruno/releases/download/v1.18.0/bruno_1.18.0_x64_win.exe";

        let url = Url::parse(url_path).expect("这不是一个有效的url");

        let file_name = url.path().split('/').last().unwrap();
        assert_eq!(file_name, "bruno_1.18.0_x64_win.exe")
    }
}
