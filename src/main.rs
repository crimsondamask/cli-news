use ureq;
use serde::Deserialize;
use std::error::Error;
use colour::{dark_green_ln, cyan_ln, e_red_ln};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// The source of the headlines: bbc-news, bloomberg...
    #[clap(short, long, default_value = "bbc-news")]
    source: String,

    /// The API key which you can get from apinews.org.
    #[clap(short, long)]
    key: String,
}

#[derive(Deserialize, Debug)]
struct Articles {
    articles : Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title : String,
    url : String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?
                            .into_string()?;
    
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)


}

fn render_articles(articles: &Articles) {
    for article in &articles.articles {
        dark_green_ln!("    {}", article.title);
        cyan_ln!("===> {}\n", article.url);
    }

}

fn articles_error(error: String) {
    e_red_ln!("Sorry, we couldn't fetch articles from newsapi\n{}", error);
}
fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();
    let url = format!("https://newsapi.org/v2/top-headlines?sources={}&apiKey={}", &args.source, &args.key);
    //let url = "https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=ddd78c9380054cd2b5b548afcf7c1011";
    let articles = get_articles(&url);

    match articles {
        Ok(articles) => render_articles(&articles),
        Err(error) => articles_error(error.to_string()),
    }
    
    Ok(())
}
