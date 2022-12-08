pub fn hex_md5(s: String) -> String {
    format!("{:x}", md5::compute(s))
}