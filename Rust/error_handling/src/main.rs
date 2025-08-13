fn check_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        Err("Password is too short".to_string())
    }
    else {
        Ok(())
    }
}


fn main() {
    let password = "Password";

    match check_password(password){
        Ok(_) => println!("Password is strong."),
        Err(msg) => println!("{}", msg),
    }
    
}
