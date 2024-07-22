use std::env;

use fondabots_lib::affichan::Affichan;
use fondabots_lib::Bot;
use maplit::hashmap;
use serenity::all::{ChannelId, GatewayIntents};

use crate::fil::fields::Status;
use crate::fil::Fil;

mod fil;
mod commands;

type DataType = fondabots_lib::DataType<Fil>;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(token) = args.get(1) {
        match Bot::new(
            token.clone(),
            GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS,
            "./staffbot.yml",
            commands::command_list(),
            vec![
                Affichan::new(ChannelId::new(1265001559373119493), Box::new(| fil | {
                    fil.status == Status::Vote
                }))
            ],
            hashmap! {

            }
        ).await {
            Ok(mut bot) => if let Err(e) = bot.start().await {
                panic!("Erreur lors de l’exécution du bot: {e}");
            }
            Err(e) => panic!("Erreur lors du chargement du bot: {e}")
        }
    }
}