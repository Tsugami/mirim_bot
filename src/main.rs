use poise::serenity_prelude::{self as serenity};

extern crate dotenv;
use dotenv::dotenv;

struct Data {
  pub member_role_id: serenity::RoleId,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn listener(
  ctx: &serenity::Context,
  event: &poise::Event<'_>,
  state: &Data,
) -> Result<(), Error> {
  match event {
    poise::Event::GuildMemberAddition { new_member } => {
      let guild_id = new_member.guild_id;
      let member_role_id = state.member_role_id;

      ctx
        .http
        .add_member_role(
          new_member.guild_id.0,
          new_member.user.id.0,
          member_role_id.0,
          None,
        )
        .await?;
      println!(
        "{} joined to {} guild and was assigned {} role",
        new_member.user.name, guild_id, member_role_id
      );
      Ok(())
    }
    _ => Ok(()),
  }
}

pub mod hh_melody;

#[derive(Debug, poise::ChoiceParameter)]
pub enum MyStringChoice {
  #[name = "W - Nota branca"]
  White,
  #[name = "V - Nota roxa"]
  Violet,
  #[name = "B - Nota azul"]
  Blue,
  #[name = "Y - Nota amarelho"]
  Yellow,
  #[name = "G - Nota verde"]
  Green,
  #[name = "S - Nota azul claro"]
  Sky,
  #[name = "R - Nota vermelha"]
  Red,
  #[name = "P - Nota rosa"]
  Pink,
}

/// Display your or another user's account creation date
#[poise::command(prefix_command, slash_command, track_edits)]
async fn hh_buffs(
  ctx: Context<'_>,
  #[description = "HH Note"] note1: MyStringChoice,
  #[description = "HH Note"] note2: Option<MyStringChoice>,
  #[description = "HH Note"] note3: Option<MyStringChoice>,
) -> Result<(), Error> {
  ctx
    .say(format!(
      "selected notes: {:?}, {:?}, {:?}",
      note1, note2, note3,
    ))
    .await?;

  Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
  poise::builtins::register_application_commands_buttons(ctx).await?;
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  dotenv().ok();

  let discord_token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN not found");
  let member_role_id = std::env::var("MEMBER_ROLE_ID")
    .expect("MEMBER_ROLE_ID not found")
    .parse::<u64>()
    .unwrap();

  let intents = serenity::GatewayIntents::MESSAGE_CONTENT
    | serenity::GatewayIntents::GUILD_MEMBERS
    | serenity::GatewayIntents::GUILD_MESSAGES;

  poise::Framework::build()
    .token(&discord_token)
    .intents(intents)
    .user_data_setup(move |_ctx, data_about_bot, _framework| {
      println!("{} is ready!", data_about_bot.user.name);

      Box::pin(async move {
        Ok(Data {
          member_role_id: serenity::RoleId(member_role_id),
        })
      })
    })
    .options(poise::FrameworkOptions {
      // configure framework here
      prefix_options: poise::PrefixFrameworkOptions {
        prefix: Some("~".into()),
        ..Default::default()
      },
      listener: |ctx, event, _framework, state| Box::pin(listener(ctx, event, state)),
      commands: vec![hh_buffs(), register()],
      ..Default::default()
    })
    .run()
    .await
    .unwrap();

  Ok(())
}
