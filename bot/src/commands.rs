use std::vec;
use rand::seq::IndexedRandom;

use crate::{ContextPoise,Error};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: ContextPoise<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ..Default::default()
        }
    ).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn addmanger(
    ctx: ContextPoise<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    if choice.eq(""){
        ctx.say("Need at least 1 args").await?;
        return Ok(())
    }
    let mut vec_miam = ctx.data().vec_miam.lock().await;
    vec_miam.push(choice.clone());

    let response = format!("Successfully add {choice}.");
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn removemanger(
    ctx: ContextPoise<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    if choice.eq(""){
        ctx.say("Need at least 1 args").await?;
        return Ok(())
    }
    let mut vec_miam = ctx.data().vec_miam.lock().await;
    let res = vec_miam.iter().find(|elem| *elem == &choice);
    match res {
        Some(_) => (),
        None => {
                ctx.say(format!("{choice} is not in the list of food")).await?;
                return Ok(())
            },
    }
    vec_miam.retain(|name| !name.eq(&choice));

    let response = format!("Successfully remove {choice}.");
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn seemanger(
    ctx: ContextPoise<'_>,
) -> Result<(), Error> {
    let vec_miam = ctx.data().vec_miam.lock().await;
    
    ctx.say(format!("{vec_miam:?}")).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn rollmanger(
    ctx: ContextPoise<'_>,
) -> Result<(), Error> {
    let vec_miam = ctx.data().vec_miam.lock().await;

    let value = match vec_miam.choose(&mut rand::rng()) {
        Some(i) => i,
        None => "No Elem in the list",
    };
    ctx.say(value).await?;

    Ok(())
}