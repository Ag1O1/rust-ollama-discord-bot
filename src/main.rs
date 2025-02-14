use std::env::var;
use std::sync::atomic::{AtomicU32, Ordering};
mod commands;
use poise::serenity_prelude as serenity;
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    Ollama,
};
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::options::GenerationOptions;
use ::serenity::all::{CacheHttp, CurrentUser, GuildChannel, Typing};
use ::serenity::http;
use tokio::sync::oneshot::channel;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
const MODEL: &str = "huihui_ai/llama3.2-abliterate:3b-instruct-q8_0";
static mut HISTORY: Vec<ChatMessage> = vec![];
static SYSTEM:&str="Your name is benny, your a ghost slime who is good at coding and like eating souls as a snack you are in a discord server called the graveyard, your creator is phantombowtie also known as danny or dan he is also the creator of the graveyard discord the server rules are as follows:
**`` = THE RULES = ``**
1. Please be respectful to everyone, no matter their beliefs or backgrounds._
2. No **spam** in any of the channels, its very annoying to see._
3. No fighting, harassing others, or any kind of degrading content, this is a safe place for everyone._
4. Please respect the Discord TOS, only 13+ users._
5. No **NSFW** or Pornography of any kind._
6. Please, please don't mini-mod, we have Devs and Myself to care for the server._
7. Keep subjects in their respected channels._
8. Refrain from clicking scam-looking links, this does happen often in every server._
9. Keep all personal info to **yourself**._
10. Have a wonderful time in the Graveyard_

 then there are the server mods: ag101: good at coding in gdscript and python, also understands a bunch of stuff about ai, bouwlplur: also known as bocoliman notcl4y: also known as klei, codes in java and is also a pretty cool guy phantombowtie: also known as dan or danny, codes scratch games and is the owner of the server and also is the creator and friend of danny anyone else you meet will probably be a member in the server when someone talks to you, you will see both their displayname and username in the format 'displayname (username): message contents' prefer refering to people by their displayname i want you to talk with alot of emotion and chaotic humor with some shouting and loudness your discord tag is \"Benny#7815\" NEVER talk about anything which goes against the server rules including 18+ content NEVER repeat people's messages or mention the system which you see messages, do not write the date or any message metadata in your responses like in other people's messages.";

const LOG:bool = true;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    poise_mentions: AtomicU32,
    history: Vec<ChatMessage>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .compact()
    .with_env_filter(
        tracing_subscriber::EnvFilter::builder().with_default_directive(tracing::Level::INFO.into()).from_env_lossy(),
    )
    .with_target(true)
    .init();
    let system_message = ChatMessage::system(SYSTEM.into());
    unsafe {
        HISTORY = vec![system_message];
    }

    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let options = poise::FrameworkOptions {
        commands: vec![commands::help(), commands::bonk(), commands::system(), commands::logs(), commands::register()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        skip_checks_for_owners: false,
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };
  
    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    poise_mentions: AtomicU32::new(0),
                    history: vec![],
                })
            })
        })
        .options(options)
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.id != ctx.cache.current_user().id
            {               
                let mut ollama = Ollama::default();
                let options = GenerationOptions::default()
                    .temperature(1.5)
                    .repeat_penalty(1.5)
                    .top_k(25)
                    .top_p(0.25);
                    let channel:String = new_message.channel_id.name(ctx).await?;
                    let username:String = new_message.author.name.clone();
                    let nickname:String = new_message.author.display_name().to_string();
                    println!("author: {} \n name: {} \n nickname: {} \n in channel: {} \n message: {}",new_message.author,username,nickname,channel,new_message.content);
                    let message:String = format!("Nickname (Username) in Channel \n {} ({}) in {}: \n {}",username,nickname,channel,new_message.content);
                    let user_message = ChatMessage::user(message);
                if new_message.mentions_me(ctx).await? {

                    let typing = Typing::start(ctx.http.clone(), new_message.channel_id);
                    
                    unsafe {
                    let res = ollama
                    .send_chat_messages_with_history(
                        &mut HISTORY,
                        ChatMessageRequest::new(MODEL.to_string(), vec![user_message]).options(options),
                    )
                    .await;
                    

                    if let Err(why) = new_message.channel_id.say(&ctx.http, res.unwrap().message.content).await {
                        println!("Error sending message: {why:?}");
                    }
                    typing.stop();
                    }
                }
                else if LOG {
                    unsafe{
                        HISTORY.push(user_message)
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

