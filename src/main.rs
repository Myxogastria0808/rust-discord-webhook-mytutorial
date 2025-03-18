use serde::Serialize;
use std::vec;

#[derive(Debug, Serialize)]
struct Webhook {
    pub embeds: Vec<Embed>,
}

#[derive(Debug, Serialize)]
struct Embed {
    pub title: String,
    pub description: String,
    pub color: i32,
    pub image: Image,
    pub fields: Vec<Field>,
    pub footer: Footer,
}

#[derive(Debug, Serialize)]
struct Field {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
struct Image {
    pub url: String,
}

#[derive(Debug, Serialize)]
struct Footer {
    pub text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fields = vec![
        Field {
            name: "Field 1".to_string(),
            value: "Value 1".to_string(),
        },
        Field {
            name: "Field 2".to_string(),
            value: "Value 2".to_string(),
        },
    ];
    let image = Image {
        url: "https://pub-de6d92ba2fc744608843c67a8fac2807.r2.dev/nix-japanese-community-square-white-background.png".to_string(),
    };
    let footer = Footer {
        text: "Footer".to_string(),
    };
    let embed = Embed {
        title: "Title".to_string(),
        description: "Description".to_string(),
        color: 0x00ff00,
        image,
        fields,
        footer,
    };
    let request = Webhook {
        embeds: vec![embed],
    };
    let client = reqwest::Client::new();
    let response = client
        .post("{Discord Webhook URL}")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request)?)
        .send()
        .await?;
    println!("{:#?}", response);
    Ok(())
}
