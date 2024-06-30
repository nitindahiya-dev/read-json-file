use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() -> Result<()> {
    let json = r#"
    {
        "article": "How to work with json in rust",
        "author": "Nitin",
        "paragraph": [
        {
        "name": "starting sentence"
        },
        {
        "name": "middle of the sentence"
        },
        {
        "name": "ending sentence"
        }
        ]
    }
    "#;

    let article: Article = read_content(json);

    println!("{:?}", article);

    Ok(())
}

fn read_content(raw_data: &str) -> Article{
    let article: Article = serde_json::from_str(raw_data).unwrap();
    return article;
}