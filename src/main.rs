mod Message;

fn main() {
    let _message = Message::make("SAKI".to_owned());

    println!("{}", _message.content);
}