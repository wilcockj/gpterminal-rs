use dotenvy::dotenv;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    models::ModelID,
};
use std::env;
use std::io::{stdin, stdout, Write};
use termimad::crossterm::{execute, style::Color::*, terminal};
use termimad::*;

#[tokio::main]
async fn main() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();
    let system_prompt =
        env::var("OPENAI_SYSMSG").unwrap_or("You are a helpful assistant".to_owned());

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: system_prompt.to_string(),
        name: None,
    }];

    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
    skin.quote_mark.set_fg(Yellow);
    println!("System Prompt: {}\n", system_prompt.to_string());
    loop {
        print!("User: ");
        stdout().flush().unwrap();

        let mut user_message_content = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: user_message_content,
            name: None,
        });

        println!("");
        let chat_completion = ChatCompletion::builder(ModelID::Gpt3_5Turbo, messages.clone())
            .create()
            .await
            .unwrap_or_default()
            .unwrap_or_default();
        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        let combined_response = format!(
            "{:#?}: {}",
            &returned_message.role,
            &returned_message.content.trim()
        );
        skin.print_text(&combined_response);
        println!("");
        messages.push(returned_message);
    }
}
