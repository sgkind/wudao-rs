mod youdao;

use clap::{Arg,Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("wd")
        .version("0.1.0")
        .author("sunguanke <sunguanke11@mails.ucas.edu.cn>")
        .about("wudao dict")
        .arg(Arg::new("word")
            .help("the word to translate")
            .required(true)
            .index(1))
        .get_matches();
    if let Some(word) = matches.get_one::<String>("word") {
        youdao::youdao_dict(word).await?;
    }
    Ok(())
}
