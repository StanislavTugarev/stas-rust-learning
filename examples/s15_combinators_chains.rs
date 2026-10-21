fn main() {
    let user = Some("              Alice                ")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .and_then(fetch_user);

    let result = Some("42")
        .and_then(|s| s.parse::<u32>().ok())
        .and_then(|s| match s {
            42 => Some("answer config".to_string()),
            8080 => Some("web_server_config".to_string()),
            _ => None,
        })
        .map(|config| config.to_uppercase());

    let env_config: Option<String> = None;
    let user_config = Some("home/user/config.conf".to_string());
    let default_config = Some("/default.config".to_string());

    let path = env_config
        .or_else(|| user_config)
        .or_else(|| default_config)
        .unwrap_or("etc/default.conf".to_string());

    let port = Some("8080")
        .filter(|s| s.chars().all(char::is_numeric))
        .map(|s| s.parse::<u16>().unwrap_or(0))
        .unwrap_or_else(|| 3000);
}

fn fetch_user(name: &str) -> Option<String> {
    println!("Looking for user: {name}");
    Some(format!("User({})", name))
}
