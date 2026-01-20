use crate::{as_ref, open_graph_nodes_opt, Node};

#[derive(Debug, Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

impl AsRef<str> for Gender {
    fn as_ref(&self) -> &str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Profile {
    /// A name normally given to an individual by a parent or self-chosen.
    pub first_name: Option<String>,

    /// A name inherited from a family or marriage and by which the individual is commonly known.
    pub last_name: Option<String>,

    /// A short unique string to identify them.
    pub username: Option<String>,

    /// Their gender.
    pub gender: Option<Gender>,
}

impl Profile {
    pub(crate) fn to_nodes(&self) -> Vec<Node<'_>> {
        let Profile {
            first_name,
            last_name,
            username,
            gender,
        } = self;

        let gender = as_ref(gender);

        open_graph_nodes_opt![
            ("profile:first_name", first_name),
            ("profile:last_name", last_name),
            ("profile:username", username),
            ("profile:gender", gender)
        ]
    }
}
