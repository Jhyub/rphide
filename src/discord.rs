use std::sync::{Arc, mpsc, Mutex};
use std::time::SystemTime;
use crate::config::Config;
use crate::trigger::Status;
use discord_sdk as ds;

pub const APP_ID: ds::AppId = 886532512833228801;

pub async fn connect(config: Arc<Mutex<Config>>, rx: mpsc::Receiver<Status>) {
    let (details, status) = {
        let config = config.lock().unwrap();
        (config.discord.details.clone(), config.discord.status.clone())
    };
    let client = make_client(ds::Subscriptions::ACTIVITY).await;

    let mut current_status = Status::Show;

    loop {
        let res = rx.recv().unwrap();
        match &res {
            Status::Hide => {
                match &current_status {
                    Status::Show => {
                        let rp = ds::activity::ActivityBuilder::default()
                            .details(&details)
                            .state(&status)
                            /*
                            .assets(
                                ds::activity::Assets::default()
                                    .large("the".to_owned(), Some("u mage".to_owned()))
                                    .small("the".to_owned(), Some("i mage".to_owned())),
                            )
                            */
                            .start_timestamp(SystemTime::now());

                        current_status = Status::Hide;
                        client.discord.update_activity(rp).await.unwrap();
                    },
                    _ => {}
                }
            },
            Status::Show => {
                match &current_status {
                    Status::Hide => {
                        current_status = Status::Show;
                        client.discord.clear_activity().await.unwrap();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

struct Client {
    discord: ds::Discord,
    user: ds::user::User,
    wheel: ds::wheel::Wheel
}

async fn make_client(subs: ds::Subscriptions) -> Client {
    let(wheel, handler) = ds::wheel::Wheel::new(Box::new(|err| {panic!("{}", err)}));
    let mut user = wheel.user();
    let discord = ds::Discord::new(ds::DiscordApp::PlainId(APP_ID), subs, Box::new(handler)).unwrap();

    user.0.changed().await.unwrap();

    let user = match &*user.0.borrow() {
        ds::wheel::UserState::Connected(user) => user.clone(),
        ds::wheel::UserState::Disconnected(err) => panic!("{}", err)
    };

    Client {
        discord,
        user,
        wheel
    }
}