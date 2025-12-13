
pub fn escape(input: &str) -> String {
    input
        .replace("'", "''")
        .replace("\"", "")
        .replace(";", "")
        .replace("--", "")
        .replace("*", "%")  // Turns dinamic searchs (like) can use '*' in place of '%'
        .chars()
        .filter(|c| !c.is_control())
        .collect::<String>()
}