use bebop_desktop_assistant::Assistant;
use dotenv::dotenv;
use rdev::listen;

fn main() {
    dotenv().ok();
    let mut assistant = Assistant::new("b".to_owned());
    println!("listening");
    // This will block.
    if let Err(error) = listen(move |event| assistant.callback(event)) {
        println!("Error: {:?}", error)
    }
}
