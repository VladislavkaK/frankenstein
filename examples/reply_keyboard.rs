use frankenstein::api_params::{ReplyMarkup, SendMessageParams};
use frankenstein::client_ureq::Bot;
use frankenstein::objects::{KeyboardButton, ReplyKeyboardMarkup};
use frankenstein::TelegramApi;

// replace with your token
static TOKEN: &str = "TOKEN";
// replace with your chat id
static CHAT_ID: i64 = 275_808_073;

fn main() {
    let bot = Bot::new(TOKEN);

    let mut keyboard: Vec<Vec<KeyboardButton>> = Vec::new();

    for i in 1..5 {
        let mut row: Vec<KeyboardButton> = Vec::new();

        for j in 1..5 {
            let name = format!("{i}{j}");
            let button = KeyboardButton::builder().text(name).build();

            row.push(button);
        }

        keyboard.push(row);
    }

    let keyboard_markup = ReplyKeyboardMarkup::builder().keyboard(keyboard).build();

    let send_message_params = SendMessageParams::builder()
        .chat_id(CHAT_ID)
        .text("hello!")
        .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(keyboard_markup))
        .build();

    bot.send_message(&send_message_params).unwrap();
}
