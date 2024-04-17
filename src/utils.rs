pub fn format_bytes(bytes: u64) -> String {
    let units = ["bytes", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    format!("{:.2} {}", size, units[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(10000), "9.77 KB");
        assert_eq!(format_bytes(100006688), "95.37 MB");
    }
}
