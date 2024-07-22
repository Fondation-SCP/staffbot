use std::str::FromStr;

use chrono::DateTime;
use poise::serenity_prelude as serenity;
use regex::Regex;
use rss::Channel;
use serenity::all::{ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, EditMessage, Timestamp};
use serenity::Context as SerenityContext;
use yaml_rust2::{Yaml, yaml};

use fields::Pole;
use fields::Status;
use fondabots_lib::{Bot, DataType, ErrType, Object, try_loop};

pub mod fields;

#[derive(Clone, PartialEq, Debug)]
pub struct Fil {
    name: String,
    lien: String,
    pub pole: Pole,
    pub status: Status,
    pub last_update: Timestamp,
    id: u64,
    modified: bool
}

impl Fil {
    pub fn new(name: String, lien: String, pole: Pole, status: Status) -> Self {
        Fil {
            name,
            pole,
            status,
            id: Self::find_id(&lien).unwrap(),
            lien,
            last_update: Timestamp::now(),
            modified: false
        }
    }

    pub fn find_id(url: &String) -> Option<u64> {
        let regex_id = Regex::new(r"t-(\d+)/?").unwrap();
        if let Some(v) = regex_id.captures(url.as_str()) {
            if let Some(w) = v.extract::<1>().1.get(0) {
                if let Ok(ret) = w.parse() {
                    return Some(ret);
                }
            }
        }
        None
    }
}

impl Object for Fil {
    fn new() -> Self {
        Fil {
            name: String::new(),
            lien: String::new(),
            pole: Pole::Autre,
            status: Status::Inconnu,
            last_update: Timestamp::now(),
            id: 0,
            modified: false
        }
    }

    fn get_id(&self) -> u64 {
        self.id
    }

    fn from_yaml(data: &Yaml) -> Result<Self, ErrType> {
        let lien = data["lien"].as_str().ok_or(ErrType::YamlParseError("Erreur de yaml dans un champ lien.".to_string()))?.to_string();
        Ok(Self {
            name: data["nom"].as_str().ok_or(ErrType::YamlParseError("Erreur de yaml dans un champ nom.".to_string()))?.to_string(),
            status: Status::from(Status::from_str(data["status"].as_str().ok_or(ErrType::YamlParseError("Erreur de yaml dans un status.".to_string()))?)?),
            pole: Pole::from(Pole::from_str(data["pole"].as_str().ok_or(ErrType::YamlParseError("Erreur de yaml dans un status.".to_string()))?)?),
            last_update: Timestamp::from_unix_timestamp(data["lastUpdate"].as_i64()
                .ok_or(ErrType::YamlParseError("Erreur de yaml dans un last_update.".to_string()))?.try_into()?)?,
            id: Self::find_id(&lien).ok_or(ErrType::NoneError)?,
            modified: false,
            lien
        })
    }

    fn serialize(&self) -> Yaml {
        let mut yaml_out = yaml::Hash::new();
        yaml_out.insert(Yaml::String("nom".to_string()), Yaml::String(self.name.clone()));
        yaml_out.insert(Yaml::String("lien".to_string()), Yaml::String(self.lien.clone()));
        yaml_out.insert(Yaml::String("pole".to_string()), Yaml::String(self.pole.to_string()));
        yaml_out.insert(Yaml::String("status".to_string()), Yaml::String(self.status.to_string()));
        yaml_out.insert(Yaml::String("lastUpdate".to_string()), Yaml::Integer(self.last_update.timestamp()));
        yaml_out.insert(Yaml::String("edited".to_string()), Yaml::Boolean(self.modified.clone()));
        Yaml::Hash(yaml_out)
    }

    fn is_modified(&self) -> bool {
        self.modified
    }

    fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    fn get_embed(&self) -> CreateEmbed {
        let fields = vec![
            ("Pôle", self.pole.to_string(), false),
            ("Statut", self.status.to_string(), false),
        ];
        CreateEmbed::new()
            .title(self.name.clone())
            .url(self.lien.clone())
            .fields(fields)
            .footer(CreateEmbedFooter::new(self.id.to_string()))
            .timestamp(&self.last_update)
            .color(self.pole.get_color())
    }

    fn get_buttons(&self) -> CreateActionRow {
        let id = &self.id;
        let termine = CreateButton::new(format!("f-{id}-t")).style(ButtonStyle::Success).label("Terminé");
        let vote = CreateButton::new(format!("f-{id}-v")).style(ButtonStyle::Secondary).label("Passage au vote");
        let dev = CreateButton::new(format!("f-{id}-d")).style(ButtonStyle::Secondary).label("Passage en développement");
        let no = CreateButton::new(format!("f-{id}-0")).style(ButtonStyle::Primary).label("Aucune action possible").disabled(true);
        let mut buttons = Vec::new();

        match self.status {
            Status::Discussion => {
                buttons.push(vote.clone());
                buttons.push(dev.clone());
                buttons.push(termine.clone().style(ButtonStyle::Danger));
            }
            Status::Vote => {
                buttons.push(dev.clone());
                buttons.push(termine.clone());
            }
            Status::EnDev => {
                buttons.push(vote.clone());
                buttons.push(termine.clone().style(ButtonStyle::Danger));
            }
            Status::Termine | Status::Inconnu => {
                buttons.push(no.clone());
            }
        }
        CreateActionRow::Buttons(buttons)
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, s: String) {
        self.name = s;
    }

    fn get_list_entry(&self) -> String {
        format!("[**{}**]({})\n{}\n{}\n\n", self.name, self.lien, self.pole, self.status)
    }

    fn up(&mut self) {
        self.last_update = Timestamp::now();
    }

    fn get_date(&self) -> &Timestamp {
        &self.last_update
    }

    fn set_date(&mut self, t: Timestamp) {
        self.last_update = t;
    }

    async fn buttons(ctx: &SerenityContext, interaction: &mut ComponentInteraction, bot: &mut Bot<Self>) -> Result<(), ErrType> {
        let parts: Vec<&str> = interaction.data.custom_id.split("-").collect();
        let button_type = *parts.get(0)
            .ok_or(ErrType::InteractionIDError(interaction.data.custom_id.clone(), interaction.message.id.get()))?;
        match button_type {
            "f" => {
                let id: u64 = parts.get(1)
                    .ok_or(ErrType::InteractionIDError(interaction.data.custom_id.clone(), interaction.message.id.get()))?.parse()?;
                let action = *parts.get(2).ok_or(ErrType::InteractionIDError(interaction.data.custom_id.clone(), interaction.message.id.get()))?;
                match action {
                    "t" | "v" | "d" => {
                        interaction.create_response(ctx, CreateInteractionResponse::Acknowledge).await?;
                        if bot.database.contains_key(&id) {
                            bot.archive(vec![id]);
                            bot.database.get_mut(&id).unwrap().status = match action {
                                "t" => Status::Termine,
                                "v" => Status::Vote,
                                "d" => Status::EnDev,
                                _ => panic!() /* Impossible */
                            };
                            bot.database.get_mut(&id).unwrap().modified = true;
                        } else {
                            return Err(ErrType::ObjectNotFound(id.to_string()));
                        }
                    }
                    _ => {
                        interaction.create_response(ctx, CreateInteractionResponse::Acknowledge).await?;
                        eprintln!("Action inconnue pressée sur un bouton: f-{id}-{action}");
                    }
                }
                let fil = bot.database.get(&id);
                interaction.message.edit(ctx, EditMessage::new().embed(
                    fil.ok_or(ErrType::ObjectNotFound(id.to_string()))?.get_embed()
                ).components(vec![fil.unwrap().get_buttons()])).await?;
                bot.update_affichans(ctx).await?;
                bot.save()?;
            }
            _ => { interaction.create_response(ctx, CreateInteractionResponse::Acknowledge).await?; }
        }

        Ok(())
    }

    async fn maj_rss(bot: &DataType<Self>) -> Result<(), ErrType> {
        let mut last_date = DateTime::from_timestamp(0, 0).unwrap();
        let bot = &mut bot.lock().await;
        for (pole, url) in [
            (Pole::Interne, "http://commandemento5.wikidot.com/feed/forum/ct-6827498.xml"),
            (Pole::Ambassade, "http://commandemento5.wikidot.com/feed/forum/ct-1905805.xml"),
            (Pole::RetD, "http://commandemento5.wikidot.com/feed/forum/ct-6827479.xml"),
            (Pole::Technique, "http://commandemento5.wikidot.com/feed/forum/ct-6827478.xml"),
            (Pole::Evenementiel, "http://commandemento5.wikidot.com/feed/forum/ct-6827484.xml"),
            (Pole::Creation, "http://commandemento5.wikidot.com/feed/forum/ct-7643353.xml"),
            (Pole::Legal, "http://commandemento5.wikidot.com/feed/forum/ct-1905799.xml")] {

            let regex_balises = Regex::new(r##"\s*\[([^\[]*)]"##).unwrap();
            let regex_titres = Regex::new(r##"(?i)\s*(?:\s*[\[(][^\[]*?[])][\s/\\\-]*)*[\s:\-"]*([^"]*?(?:"[^"]+"?[^"]*?)*)[\s".]*[\s".]*$"##).unwrap();
            let rss = Channel::read_from(&reqwest::get(url).await?.bytes().await?[..])?;
            for entry in &rss.items {
                let date = try_loop!(DateTime::parse_from_rfc2822(entry.pub_date.as_ref().unwrap().as_str()), "Erreur lors de la récupération des flux RSS: pas de date.").to_utc();
                if date > bot.last_rss_update {
                    if entry.title.as_ref().is_some_and(|str| { str.contains("]") }) {
                        let mut status = Status::Discussion;
                        for balise in regex_balises.captures_iter(entry.title.as_ref().unwrap()) {
                            let balise = balise.extract::<1>().0.trim().to_lowercase();
                            if balise.contains("vote") {
                                status = Status::Vote;
                            } else if balise.contains("terminé") {
                                status = Status::Vote;
                            } else if balise.contains("développement") {
                                status = Status::EnDev;
                            }
                        }
                        let mut title = try_loop!(try_loop!(regex_titres.captures(entry.title.as_ref().unwrap())
                                .ok_or(ErrType::NoneError), "Erreur lors de l’interprétation du titre.").extract::<1>().1.get(0)
                                .ok_or(ErrType::NoneError), "Erreur lors de l’interprétation du titre.").to_string();
                        if title.is_empty() {
                            title = format!("(sans nom {})", bot.search("sans nom").len());
                        }

                        let lien = try_loop!(entry.link.clone().ok_or(ErrType::NoneError), "Pas de lien dans une entrée RSS.");
                        let id: u64 = try_loop!(Fil::find_id(&lien).ok_or(ErrType::NoneError), "Lien mal formé dans une entrée RSS.");

                        let fil = Fil {
                            status,
                            pole: pole.clone(),
                            name: title,
                            lien,
                            last_update: Timestamp::now(),
                            modified: false,
                            id,
                        };
                        if !bot.database.contains_key(&id) {
                            bot.database.insert(id, fil);
                        } else {
                            eprintln!("Ajout RSS d’un écrit déjà ajouté. Informations : écrit [{}] - last_rss_update [{}] - last_date [{}] - date>last_rss_update [{}]", date, bot.last_rss_update, last_date, date > bot.last_rss_update);
                        }

                    }
                }
                if date > last_date {
                    last_date = date;
                }
            }
        }

        bot.last_rss_update = last_date;
        bot.update_affichans = true;
        Ok(())
    }
}