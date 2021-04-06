use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let token = &args[1];

    println!("Token passed: {}", &token);

    let client = reqwest::ClientBuilder::new().build()?;

    let resp = client
        .get("https://canary.discordapp.com/api/v8/users/@me")
        .header("Authorization", token.clone())
        .header("User-Agent", "Mozilla/5.0'")
        .send()
        .await?;

    if resp.status().is_success() {
        let profile_data = json::parse(&resp.text().await?).unwrap();

        println!("User Profile:");
        println!("E-Mail: {}", profile_data["email"]);
        println!("ID: {}", &profile_data["id"]);
        println!("Username: {}", &profile_data["username"]);
        println!("Discriminator: {}", &profile_data["discriminator"]);
        println!("Locale: {}", &profile_data["locale"]);
        println!("Verified: {}", &profile_data["verified"]);
        println!("Phone Number: {}", &profile_data["phone"]);
        println!("NSFW Allowed: {}", &profile_data["nsfw_allowed"]);
        println!("MFA Enabled: {}", &profile_data["mfa_enabled"]);
    } else {
        println!("This token is invalid.");
    }

    Ok(())
}
