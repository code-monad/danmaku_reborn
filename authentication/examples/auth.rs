use bili_api_authentication::*;

fn main() {
    if let Some(auther) = Authenticator::new("a", "b") {
        println!(
            "AccessKey: {}, AccessSecret: {}",
            auther.get_access_key(),
            auther.get_access_secret()
        );
        println!("{:?}", auther.build_header("無限大の夢の中で".as_bytes()));
    }
}
