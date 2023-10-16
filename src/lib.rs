#[derive(Debug, Clone, Default)]
pub struct OpenGraph {
    /// The title of your object as it should appear within the graph, e.g., "The Rock".
    pub title: Option<String>,

    /// The type of your object, e.g., "video.movie". Depending on the type you specify, other properties may also be required.
    pub kind: Option<String>,

    /// The canonical URL of your object that will be used as its permanent ID in the graph, e.g., "https://www.imdb.com/title/tt0117500/".
    pub url: Option<String>,

    /// An image URL which should represent your object within the graph.
    pub image: Option<String>,

    /// A URL to an audio file to accompany this object.
    pub audio: Option<String>,

    /// A URL to a video file that complements this object.
    pub video: Option<String>,

    /// A one to two sentence description of your object.
    pub description: Option<String>,

    /// The word that appears before this object's title in a sentence. An enum of (a, an, the, "", auto).
    /// If auto is chosen, the consumer of your data should chose between "a" or "an". Default is "" (blank).
    pub determiner: Option<String>,

    /// The locale these tags are marked up in. Of the format language_TERRITORY. Default is en_US.
    pub locale: Option<String>,

    /// An array of other locales this page is available in.
    pub alternate_locale: Vec<String>,

    /// If your object is part of a larger web site, the name which should be displayed for the overall site. e.g., "IMDb".
    pub site_name: Option<String>,
}

macro_rules! open_graph_nodes {
    [$(($og:expr, $x:ident)$(,)?)*] => {
        {
            let mut open_graph_nodes = Vec::new();
            $(
            if let Some($x) = $x.as_ref() {
                let node = Node {
                    name: "meta",
                    attr: vec![("property", $og), ("content", $x)],
                    children: Vec::new(),
                };
                open_graph_nodes.push(node);
            }
            )*
            open_graph_nodes
        }
    };
}

impl OpenGraph {
    pub fn to_html(&self) -> String {
        self.to_node().to_html()
    }

    fn to_node(&self) -> Node {
        let OpenGraph {
            title,
            kind,
            url,
            image,
            audio,
            video,
            description,
            determiner,
            locale,
            alternate_locale,
            site_name,
        } = self;

        let mut open_graph_nodes = open_graph_nodes![
            ("og:title", title),
            ("og:type", kind),
            ("og:url", url),
            ("og:image", image),
            ("og:audio", audio),
            ("og:video", video),
            ("og:description", description),
            ("og:determiner", determiner),
            ("og:locale", locale),
            ("og:site_name", site_name),
        ];

        for locale in alternate_locale {
            let node = Node {
                name: "meta",
                attr: vec![("property", "og:locale:alternate"), ("content", locale)],
                children: Vec::new(),
            };
            open_graph_nodes.push(node);
        }

        Node {
            name: "html",
            attr: vec![("prefix", "og: https://ogp.me/ns#")],
            children: vec![Node {
                name: "head",
                attr: Vec::new(),
                children: open_graph_nodes,
            }],
        }
    }
}

struct Node<'a> {
    name: &'static str,
    attr: Vec<(&'static str, &'a str)>,
    children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    fn to_html(&self) -> String {
        let mut r = String::new();

        r.push('<');
        r.push_str(self.name);
        for (key, value) in self.attr.iter() {
            r.push(' ');
            r.push_str(key);
            r.push('=');
            r.push('\"');
            r.push_str(value);
            r.push('\"');
        }

        if self.children.is_empty() {
            r.push_str("/>");
        } else {
            r.push('>');

            for children in self.children.iter() {
                r.push_str(&children.to_html());
            }

            r.push_str("</");
            r.push_str(self.name);
            r.push('>');
        }

        r
    }
}

#[test]
fn test_to_html() {
    let og = OpenGraph {
        title: "open graph".to_owned().into(),
        description: "this is open graph".to_owned().into(),
        ..Default::default()
    };

    let html = og.to_html();

    println!("{html}");

    assert_eq!(
        html,
        r#"<html prefix="og: https://ogp.me/ns#"><head><meta property="og:title" content="open graph"/><meta property="og:description" content="this is open graph"/></head></html>"#
    )
}
