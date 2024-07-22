use std::env;

use maplit::hashmap;
use serenity::all::GatewayIntents;

use fondabots_lib::Bot;

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