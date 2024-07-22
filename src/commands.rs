use poise::{Command, Context};

use fondabots_lib::ErrType;
use fondabots_lib::generic_commands;
use fondabots_lib::generic_commands::gcom;

use crate::DataType;
use crate::fil::fields::Pole;
use crate::fil::fields::Status;
use crate::fil::Fil;

/// Ajoute manuellement un fil à la base de données.
#[poise::command(slash_command)]
pub async fn ajouter(
    ctx: Context<'_, DataType, ErrType>,
    #[description = "Nom du fil"] nom: String,
    #[description = "Pole du fil"] pole: Pole,
    #[description = "Statut du fil"] statut: Status,
    #[description = "Lien forum du fil"] url: String
) -> Result<(), ErrType> {
    let bot = &mut ctx.data().lock().await;
    if let Some(id) = Fil::find_id(&url) {
        bot.database.insert(id, Fil::new(nom.clone(), url, pole, statut));
        ctx.say(format!("Fil « {nom} » ajouté !")).await?;
    } else {
        ctx.say("URL malformée, impossible de déterminer l’identifiant du fil.").await?;
    }
    Ok(())
}

pub fn command_list() -> Vec<Command<DataType, ErrType>> {
    vec![ajouter(),
         gcom(&generic_commands::change_field::<Fil, Pole>,
              "pole",
          "Change le pôle d’un fil.",
                    vec![("critere", "Critère de sélection du fil"),
                            ("pole", "Nouveau pôle")])]
}
