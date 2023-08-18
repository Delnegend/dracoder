pub fn replace_ext(path: &str, ext: &str) -> String {
    let mut path = path.to_string();
    let mut ext = ext.to_string();
    if !ext.starts_with('.') {
        ext = ".".to_string() + &ext;
    }
    if let Some(pos) = path.rfind('.') {
        path.replace_range(pos.., &ext);
    }
    path
}
pub fn from_bin_get_ext<'a>() -> &'a str {
    let bin_name = std::env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    match bin_name.as_str() {
        case if case.contains("webp") => "webp",
        case if case.contains("avif") => "avif",
        _ => {
            println!("Unknown binary name, press any key to exit...");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            panic!("Unknown binary name")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_ext() {
        assert_eq!(replace_ext("test.png", "webp"), "test.webp".to_string());
    }
}
