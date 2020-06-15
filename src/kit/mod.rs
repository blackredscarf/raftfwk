use std::env;

pub fn check_addr(addr: &String) -> bool {
    let v: Vec<&str> = addr.split(":").collect();
    v.len() == 2
}

pub fn no_port(ip: &String) -> String {
    let v: Vec<&str> = ip.split(":").collect();
    v[0].to_string()
}

pub fn get_env_var(key: &str, default: &str) -> String {
    match env::var(String::from(key)) {
        Ok(n) => n,
        Err(err) => String::from(default)
    }
}
