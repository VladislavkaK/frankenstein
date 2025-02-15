use frankenstein::client_reqwest::Bot;
use frankenstein::AsyncTelegramApi;

static TOKEN: &str = "API_TOKEN";

#[tokio::main]
async fn main() {
    let bot = Bot::new(TOKEN);

    match bot.get_me().await {
        Ok(response) => {
            let user = response.result;
            println!(
                "Hello, I'm @{}, https://t.me/{}",
                user.first_name,
                user.username.expect("The bot must have a username.")
            );
        }
        Err(error) => {
            eprintln!("Failed to get me: {error:?}");
        }
    }
}
