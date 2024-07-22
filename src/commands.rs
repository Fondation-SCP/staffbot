use poise::{Command, Context, CreateReply};
use serenity::all::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};

use fondabots_lib::ErrType;
use fondabots_lib::generic_commands;
use fondabots_lib::tools::alias;

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

/// Change le pôle d’un fil
#[poise::command(slash_command)]
pub async fn pole(ctx: Context<'_, DataType, ErrType>,
                  #[description = "Critère d’identification du fil"] critere: String,
                  #[description = "Nouveau pôle du fil"] pole: Pole) -> Result<(), ErrType> {
    generic_commands::change_field(ctx, critere, pole).await
}

/// Change le statut d’un fil
#[poise::command(slash_command)]
pub async fn statut(ctx: Context<'_, DataType, ErrType>,
                    #[description = "Critère d’identification du fil"] critere: String,
                    #[description = "Nouveau statut du fil"] statut: Status) -> Result<(), ErrType> {
    generic_commands::change_field(ctx, critere, statut).await
}

/// Liste les fils correspondant aux statut et poles demandés.
#[poise::command(slash_command)]
pub async fn lister(ctx: Context<'_, DataType, ErrType>,
                    #[description = "Statut recherché"] statut: Option<Status>,
                    #[description = "Pôle recherché"] pole: Option<Pole>) -> Result<(), ErrType> {
    generic_commands::lister_two(ctx, statut, pole).await
}

/// Affiche la page d’aide du bot.
#[poise::command(slash_command, prefix_command)]
pub async fn aide(ctx: Context<'_, DataType, ErrType>) -> Result<(), ErrType> {
    ctx.send(CreateReply::default().embed(CreateEmbed::new()
        .title("Aide du Staffbot")
        .description("Les paramètres entre crochets sont optionnels, entre accolades obligatoires. La description des options est disponible en description des commandes slash.")
        .fields(vec![
            ("Commandes de base",
             "`/aide` : Cette commande d'aide.\n\
            `/annuler` : Annule la dernière modification effectuée.", false),
            ("Commandes de gestion et d'affichage de la liste",
             "`/ajouter {Nom} {Pole} {Statut} {URL}` : Ajoute manuellement un fil à la liste.\n\
            `/supprimer {Critère}` : Supprime un fil. Le Critère doit être assez fin pour aboutir à un unique fil. __**ATTENTION**__ : Il n'y a pas de confirmation, faites attention à ne pas vous tromper dans le Critère.\n", false),
            ("Commandes de modification des fils",
            "`/statut {Critère} {Statut}` : Modifie le statut d’un fil pour le nouveau statut.\n\
            `/pole {Critère} {Pôle}` : Modifie le pôle d’un fil pour le nouveau pôle.", false),
            ("Commandes de recherche",
             "`/rechercher {Critère}` : Affiche tous les fils contenant {Critère}.\n\
            `/lister {Statut} {Pôle}` : Affiche la liste des fils avec le statut et du pôle demandés.", false),
            ("Commandes d'entretien de la base de données (À utiliser avec précaution)",
             "`/doublons` : Supprime les éventuels doublons.", false),
            ("Code source", "Disponible sur [Github](https://github.com/Fondation-SCP/staffbot).", false)
        ])
        .footer(CreateEmbedFooter::new("Version 0.1"))
        .author(CreateEmbedAuthor::new("Staffbot"))
    )).await?;
    Ok(())
}

pub fn command_list() -> Vec<Command<DataType, ErrType>> {
    vec![ajouter(), pole(), statut(), lister(), aide(), alias("help", aide())]
}
