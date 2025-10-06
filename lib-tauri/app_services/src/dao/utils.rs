use app_utils::app_error::DomainError;
use diesel::result::Error;

pub(crate) fn map_error_annotazione(e: Error) -> DomainError {
    map_error_for_entity(e, EntityType::Annotazione)
}

#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    Annotazione,
    Edificio,
    Stanza,
    Infisso,
}

pub(crate) fn map_error_for_entity(e: Error, entity: EntityType) -> DomainError {
    match e {
        Error::NotFound => match entity {
            EntityType::Annotazione => DomainError::AnnotazioneNotFound,
            EntityType::Edificio => DomainError::EdificioNotFound,
            EntityType::Stanza => DomainError::StanzaNotFound,
            EntityType::Infisso => DomainError::InfissoNotFound,
        },
        Error::DatabaseError(kind, ..) => {
            if matches!(kind, diesel::result::DatabaseErrorKind::UniqueViolation) {
                match entity {
                    EntityType::Annotazione => DomainError::AnnotazioneAlreadyExists,
                    EntityType::Edificio => DomainError::EdificioAlreadyExists,
                    EntityType::Stanza => DomainError::StanzaAlreadyExists,
                    EntityType::Infisso => DomainError::InfissoAlreadyExists,
                }
            } else {
                DomainError::from(e)
            }
        }
        _ => DomainError::Unexpected(e),
    }
}
