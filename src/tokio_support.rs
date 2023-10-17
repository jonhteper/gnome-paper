use std::io;
use tokio::time::{self, Duration};
use tokio::sync::oneshot;
use chrono::Local;

async fn background_task(stop_signal: oneshot::Receiver<()>) {
    loop {
        tokio::select! {
            _ = time::sleep(Duration::from_millis(5000)) => {
                let now = Local::now();
                println!("async task running at {:?}", now);
            }
            _ = stop_signal => {
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (stop_sender, stop_receiver) = oneshot::channel();
    let handle = tokio::spawn(background_task(stop_receiver));

    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading input");

        match buffer.as_str().trim() {
            "close" => {
                let _ = stop_sender.send(());
                break;
            },
            _ => println!("el usuario dice: {}", &buffer),
        }
    }

    handle.await.unwrap();
}
