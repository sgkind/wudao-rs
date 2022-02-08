mod youdao;

use clap::{Arg,App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("wd")
        .version("0.1.0")
        .author("sunguanke <sunguanke11@mails.ucas.edu.cn>")
        .about("wudao dict")
        .arg(Arg::new("word")
            .help("the word to translate")
            .required(true)
            .index(1))
        .get_matches();
    if let Some(word) = matches.value_of("word") {
        youdao::youdao_dict(word).await?;
    }
    Ok(())
}
