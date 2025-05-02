use std::collections::HashMap;

#[cfg_attr(test, derive(PartialEq, Debug, Clone))]
pub struct StanzaConInfissi {
    pub(crate) id_stanza: u64,
    pub(crate) id_infissi: Vec<(String, u64)>,
}

impl StanzaConInfissi {
    pub fn new(id_stanza: u64, id_infissi: Vec<(String, u64)>) -> Self {
        Self {
            id_stanza,
            id_infissi,
        }
    }

    pub fn new_with_infissi_expanse(id_stanza: u64, id_infissi: Vec<String>) -> Self {
        let mut conteggio = HashMap::new();
        for infissi in id_infissi {
            *conteggio.entry(infissi).or_insert(0) += 1;
        }

        Self::new(
            id_stanza,
            conteggio
                .into_iter()
                .map(|(id, count)| (id, count))
                .collect(),
        )
    }

    pub fn expanse_infissi(&self) -> Vec<String> {
        self.id_infissi
            .iter()
            .flat_map(|(id, count)| std::iter::repeat(id.to_string()).take(*count as usize))
            .collect()
    }
}
