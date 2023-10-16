pub mod article;
pub mod profile;

use std::borrow::Cow;

use article::Article;

#[derive(Debug, Clone, Default)]
pub struct OpenGraph {
    /// The title of your object as it should appear within the graph, e.g., "The Rock".
    pub title: Option<String>,

    /// The type of your object, e.g., "video.movie". Depending on the type you specify, other properties may also be required.
    pub kind: Option<OpenGraphType>,

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

#[derive(Debug, Clone)]
pub enum OpenGraphType {
    Article(Article),
    Profile(Profile),
}

impl AsRef<str> for OpenGraphType {
    fn as_ref(&self) -> &str {
        match self {
            OpenGraphType::Article(_) => "article",
            OpenGraphType::Profile(_) => "profile",
        }
    }
}

macro_rules! open_graph_nodes_opt {
    [$(($og:expr, $x:ident)$(,)?)*] => {
        {
            let mut xs = Vec::new();
            $(
                if let Some($x) = $x {
                    let node = Node {
                        name: "meta",
                        attr: vec![("property", $og.into()), ("content", $x.into())],
                        children: Vec::new(),
                    };
                    xs.push(node);
                }
            )*
            xs
        }
    };
}

pub(crate) use open_graph_nodes_opt;

macro_rules! open_graph_nodes_vec {
    [$(($og:expr, $x:ident)$(,)?)*] => {
        {
            let mut xs = Vec::new();
            $(
                for $x in $x {
                    let node = Node {
                        name: "meta",
                        attr: vec![("property", $og.into()), ("content", $x.into())],
                        children: Vec::new(),
                    };
                    xs.push(node);
                }
            )*
            xs
        }
    };
}

pub(crate) use open_graph_nodes_vec;
use profile::Profile;

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

        let (ns, nodes) = match kind.as_ref() {
            Some(OpenGraphType::Article(article)) => {
                let ns = "og: https://ogp.me/ns# article: http://ogp.me/ns/article# profile: https://ogp.me/ns/profile#";
                let nodes = article.to_nodes();
                (ns, nodes)
            }
            Some(OpenGraphType::Profile(profile)) => {
                let ns = "og: https://ogp.me/ns# profile: https://ogp.me/ns/profile#";
                let nodes = profile.to_nodes();
                (ns, nodes)
            }
            None => ("og: https://ogp.me/ns#", Vec::new()),
        };

        let kind = as_ref(kind);

        let open_graph_nodes = merge(
            open_graph_nodes_opt![
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
            ],
            open_graph_nodes_vec![("og:locale:alternate", alternate_locale)],
        );

        Node {
            name: "html",
            attr: vec![("prefix", ns.into())],
            children: vec![Node {
                name: "head",
                attr: Vec::new(),
                children: append(
                    merge(open_graph_nodes, nodes),
                    Node {
                        name: "meta",
                        attr: vec![("charset", "utf-8".into())],
                        children: Vec::new(),
                    },
                ),
            }],
        }
    }
}

macro_rules! iso8601 {
    [$($x:ident$(,)?)*] => {
        $(
            let $x = $x.map(|x| x.to_rfc3339());
        )*
    };
}

pub(crate) use iso8601;

pub fn merge<T>(mut xs: Vec<T>, mut ys: Vec<T>) -> Vec<T> {
    xs.append(&mut ys);

    xs
}

pub fn append<T>(mut xs: Vec<T>, x: T) -> Vec<T> {
    xs.push(x);

    xs
}

pub fn as_ref<T, U>(x: &Option<T>) -> Option<&U>
where
    T: AsRef<U>,
    U: ?Sized,
{
    x.as_ref().map(|u| u.as_ref())
}

struct Node<'a> {
    name: &'static str,
    attr: Vec<(&'static str, Cow<'a, str>)>,
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
        r#"<html prefix="og: https://ogp.me/ns#"><head><meta property="og:title" content="open graph"/><meta property="og:description" content="this is open graph"/><meta charset="utf-8"/></head></html>"#
    )
}

#[test]
fn test_profile() {
    let og = OpenGraph {
        title: "Syrflover".to_owned().into(),
        description: "madome developer".to_owned().into(),
        kind: OpenGraphType::Profile(Profile {
            first_name: "Lee".to_owned().into(),
            last_name: "TaeWoo".to_owned().into(),
            username: "syrflover".to_owned().into(),
            gender: profile::Gender::Male.into(),
        })
        .into(),
        ..Default::default()
    };

    let html = og.to_html();

    println!("{html}");

    assert_eq!(
        html,
        r#"<html prefix="og: https://ogp.me/ns# profile: https://ogp.me/ns/profile#"><head><meta property="og:title" content="Syrflover"/><meta property="og:type" content="profile"/><meta property="og:description" content="madome developer"/><meta property="profile:first_name" content="Lee"/><meta property="profile:last_name" content="TaeWoo"/><meta property="profile:username" content="syrflover"/><meta property="profile:gender" content="male"/><meta charset="utf-8"/></head></html>"#
    );
}

#[test]
fn test_article() {
    let og = OpenGraph {
        title: "why can't fly".to_owned().into(),
        kind: OpenGraphType::Article(Article {
            published_time: Some("2022-12-19T16:39:57+09:00".parse().unwrap()),
            modified_time: Some("2023-03-12T11:25:33+09:00".parse().unwrap()),
            expiration_time: Some("2024-05-03T00:00:00+09:00".parse().unwrap()),
            author: vec!["https://og.example.com/@syrflover".to_owned()],
            section: "Nothing".to_owned().into(),
            tag: vec!["chicken".to_owned(), "food".to_owned(), "fry".to_owned()],
        })
        .into(),
        ..Default::default()
    };

    let html = og.to_html();

    println!("{html}");

    assert_eq!(
        html,
        r#"<html prefix="og: https://ogp.me/ns# article: http://ogp.me/ns/article# profile: https://ogp.me/ns/profile#"><head><meta property="og:title" content="why can't fly"/><meta property="og:type" content="article"/><meta property="article:published_time" content="2022-12-19T07:39:57+00:00"/><meta property="article:modified_time" content="2023-03-12T02:25:33+00:00"/><meta property="article:expiration_time" content="2024-05-02T15:00:00+00:00"/><meta property="article:section" content="Nothing"/><meta property="article:author" content="https://og.example.com/@syrflover"/><meta property="article:tag" content="chicken"/><meta property="article:tag" content="food"/><meta property="article:tag" content="fry"/><meta charset="utf-8"/></head></html>"#
    );
}
