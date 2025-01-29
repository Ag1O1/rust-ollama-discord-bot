use crate::{Context, Error, HISTORY, SYSTEM, ChatMessage};

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Command list",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn bonk(
    ctx: Context<'_>,
    #[description = "Amount of messages to delete, leave empty or zero to reset all"]
    mut amount: i32
    ) -> Result<(), Error> {
    let system_message = ChatMessage::system(SYSTEM.clone().into());
    if (amount) == 0{
    unsafe {
        HISTORY = vec![system_message];
        }
    } else {while amount > 0 {
        amount -= 1;
        unsafe{
        HISTORY.pop();
    }}}
    let response = format!("OW MY HEAD!");
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn system(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "system message to add"]
    message: Option<String>,
) -> Result<(), Error> {
    let system_message = ChatMessage::system(message.expect("you need to provide a string"));
    unsafe {
        HISTORY.push(system_message);
    }
    let response = format!("sucess!");
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn logs(
    ctx: Context<'_>,
) -> Result<(), Error> {
    println!("not implemented yet");
    let response = format!("printed logs! / not implemented yet");
    ctx.say(response).await?;
    Ok(())
}
