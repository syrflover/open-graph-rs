use chrono::{DateTime, Utc};

use crate::{iso8601, merge, open_graph_nodes_opt, open_graph_nodes_vec, Node};

#[derive(Debug, Clone, Default)]
pub struct Article {
    /// When the article was first published.
    pub published_time: Option<DateTime<Utc>>,

    /// When the article was last changed.
    pub modified_time: Option<DateTime<Utc>>,

    ///  When the article is out of date after.
    pub expiration_time: Option<DateTime<Utc>>,

    /// Writers of the article.
    pub author: Vec<String>,

    /// A high-level section name. E.g. Technology
    pub section: Option<String>,

    /// Tag words associated with this article.
    pub tag: Vec<String>,
}

impl Article {
    pub(crate) fn to_nodes(&self) -> Vec<Node> {
        let Article {
            published_time,
            modified_time,
            expiration_time,
            author,
            section,
            tag,
        } = self;

        iso8601![published_time, modified_time, expiration_time];

        merge(
            open_graph_nodes_opt![
                ("article:published_time", published_time),
                ("article:modified_time", modified_time),
                ("article:expiration_time", expiration_time),
                ("article:section", section),
            ],
            open_graph_nodes_vec![("article:author", author), ("article:tag", tag)],
        )
    }
}

// https://github.com/niallkennedy/open-graph-protocol-examples/blob/master/article-utc.html
