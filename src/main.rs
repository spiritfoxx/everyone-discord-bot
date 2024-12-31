use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use serenity::model::webhook::Webhook;
use std::io::stdin;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

const WEBHOOK_URL: &'static str = "https://discord.com/api/webhooks/1310152581959385089/queNbPs0Czy6yZRDaSZ3ZEVFN5eMUUhUtaHsN_ydqks62leqWMMeYfs_mQYaiKDZdZPR";
const USERNAME: &'static str = "☦️ Nationalism Enjoyer ☦️";
const AVATAR_URL: &'static str = "https://cdn.discordapp.com/attachments/1310152570395951144/1310853034548531200/9H8R8u1nnKYu.png?ex=6746ba85&is=67456905&hm=0a50bf5ffcb9d564b5d1bf8379911d57d5999c00215a285a7de364f2d4129f66&";
const SCHEDULE: &'static str = "0 30 15 * * *";

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    let http = Http::new("");
    let webhook = Webhook::from_url(&http, WEBHOOK_URL)
        .await
        .expect("Could not create Webhook");

    let sched = JobScheduler::new().await?;
    sched
        .add(Job::new_async(SCHEDULE, move |uuid, mut l| {
            Box::pin({
                let http = Http::new("");
                let webhook = webhook.clone();

                async move {
                    let content;
                    if rand::random() {
                        content = "@everyone TODAY WE ARE:\ndown to fuck";
                    } else {
                        content = "@everyone TODAY WE ARE:\nmale to female"
                    }

                    let builder = ExecuteWebhook::new()
                        .content(content)
                        .username(USERNAME)
                        .avatar_url(AVATAR_URL);

                    webhook
                        .execute(&http, false, builder.clone())
                        .await
                        .expect("Could not execute webhook");

                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => println!("Next message will be announced at {:?}", ts),
                        _ => println!("Could not get next tick"),
                    }
                }
            })
        })?)
        .await?;

    sched.start().await?;

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    Ok(())
}
