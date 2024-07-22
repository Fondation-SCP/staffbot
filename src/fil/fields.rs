use std::fmt::{Display, Formatter};
use std::str::FromStr;

use poise::ChoiceParameter;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use fondabots_lib::ErrType;
use fondabots_lib::object::Field;

use super::Fil;

#[derive(EnumIter, Clone, PartialEq, Eq, ChoiceParameter, Debug)]
pub enum Pole {
    Disciplinaire,
    Ambassade,
    #[name = "R&D"]
    RetD,
    Technique,
    #[name = "Évènementiel"]
    Evenementiel,
    #[name = "Création"]
    Creation,
    Traduction,
    #[name = "Légal"]
    Legal,
    Interne,
    Autre
}

impl Pole {
    pub fn get_color(&self) -> i32 {
        match self {
            Pole::Disciplinaire => 0xFFFFFF,
            Pole::Ambassade => 0xFFFFFF,
            Pole::RetD => 0xFFFFFF,
            Pole::Technique => 0xFFFFFF,
            Pole::Evenementiel => 0xFFFFFF,
            Pole::Creation => 0xFFFFFF,
            Pole::Traduction => 0xFFFFFF,
            Pole::Legal => 0xFFFFFF,
            Pole::Interne => 0xFF0000,
            Pole::Autre => 0xFFFFFF
        }
    }
}

impl Display for Pole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Pole {
    type Err = ErrType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for e in Self::iter() {
            if e.to_string().as_str() == s {
                return Ok(e);
            }
        }
        Err(ErrType::ObjectNotFound(format!("Pole {s} inexistant.")))
    }
}

impl Field<Fil> for Pole {
    fn comply_with(obj: &Fil, field: &Option<Self>) -> bool {
        if let Some(field) = field {
            return obj.pole == *field
        } else {
            true
        }
    }

    fn set_for(obj: &mut Fil, field: &Self) {
        obj.pole = field.clone();
    }

    fn field_name() -> &'static str {
        "Pôle"
    }
}

#[derive(EnumIter, Clone, PartialEq, Eq, ChoiceParameter, Debug)]
pub enum Status {
    Discussion,
    Vote,
    #[name = "En développement"]
    EnDev,
    #[name = "Terminé"]
    Termine,
    Inconnu
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Status {
    type Err = ErrType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for e in Self::iter() {
            if e.to_string().as_str() == s {
                return Ok(e);
            }
        }
        Err(ErrType::ObjectNotFound(format!("Pole {s} inexistant.")))
    }
}

impl Field<Fil> for Status {
    fn comply_with(obj: &Fil, field: &Option<Self>) -> bool {
        if let Some(field) = field {
            return obj.status == *field
        } else {
            true
        }
    }

    fn set_for(obj: &mut Fil, field: &Self) {
        obj.status = field.clone();
    }

    fn field_name() -> &'static str {
        "Statut"
    }
}