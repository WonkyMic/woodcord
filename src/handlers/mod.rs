use reqwest::StatusCode;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use crate::domain;

const LIST_AUTHORS_COMMAND: &str = "!listauthors";
const AMI_HEALTH_COMMAND: &str = "!amihealth";
const TEST_COMMAND: &str = "!test";
// const HELP_COMMAND: &str = "!help";

pub struct WoodcordHandler;

#[async_trait]
impl EventHandler for WoodcordHandler {

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        match &msg.content[..] {
            LIST_AUTHORS_COMMAND => {
                let client = reqwest::Client::builder().build().unwrap();
                let resp = client.get("http://localhost:8080/ami/author").send();
                match resp.await {
                    Ok(resp) => {
                        if resp.status() == StatusCode::OK {
                            let author_list = resp.json::<Vec<domain::ami::Author>>().await.unwrap();
                            println!("Author list: {:?}", author_list);
                            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Unformatted Author List: {:?}", author_list)).await {
                                println!("Error sending message: {:?}", why);
                            }
                        } else {
                            if let Err(why) = msg.channel_id.say(&ctx.http, "Error retrieving or parsing response.").await {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                    }
                    Err(_) => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Error :: service might be off").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
            },
            AMI_HEALTH_COMMAND => {
                let client = reqwest::Client::builder()
                    .build().unwrap();
                let resp = client.get("http://localhost:8080/ami/health").send();
                match resp.await {
                    Ok(resp) => {
                        if resp.status() == StatusCode::OK {
                            if let Err(why) = msg.channel_id.say(&ctx.http, "We good!").await {
                                println!("Error sending message: {:?}", why);
                            }
                        } else {
                            if let Err(why) = msg.channel_id.say(&ctx.http, "We not good :/").await {
                                println!("Error sending message: {:?}", why);
                            }
                        }   
                    },
                    Err(_) => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "We not good :: service might be off").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
            }, 
            TEST_COMMAND => {
                let channel = match msg.channel_id.to_channel(&ctx).await {
                    Ok(channel) => channel,
                    Err(why) => {
                        println!("Error getting channel: {:?}", why);
    
                        return;
                    },
                };
    
                let response = MessageBuilder::new()
                    .push("User Name: ")
                    .push_bold_safe(&msg.author.name)
                    .push("\n MessageId: ")
                    .push_bold_safe(&msg.id)
                    .push("\n Channel:")
                    .mention(&channel)
                    .push("\n Content: ")
                    .push_bold_safe(&msg.content)
                    .build();
    
                if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
            },
            _ => println!("Message ain't special")
        }
    }
}