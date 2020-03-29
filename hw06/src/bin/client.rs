extern crate bbs;
extern crate clap;
extern crate hyper;
extern crate rustc_serialize;
extern crate tokio;

use clap::{App, Arg, SubCommand};

use bbs::utils::Result;
use bbs::{UserClient, HTTP_ADDR, JSON_ADDR};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("CIS 198 Web 3.0 SoLoMobile-Scale Telnet BBS.js! command line client")
        .version("0.1.0")
        .author("alvsanand@gmail.com")
        .subcommand(SubCommand::with_name("get").about("Get messages of BBS.js"))
        .subcommand(
            SubCommand::with_name("send")
                .about("Send a message in BBS.js")
                .arg(
                    Arg::with_name("user")
                        .required(true)
                        .help("The user of the post"),
                )
                .arg(
                    Arg::with_name("text")
                        .required(true)
                        .help("The text of the post"),
                ),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("get") {
        let client = UserClient::new(JSON_ADDR.to_string());

        let response = client.get_content().await?;

        println!(
            "Get messages[{code}]:\n{response}",
            code = response.0,
            response = response.1
        );
    } else if let Some(matches) = matches.subcommand_matches("send") {
        let client = UserClient::new(HTTP_ADDR.to_string());

        let user = matches.value_of("user").unwrap();
        let text = matches.value_of("text").unwrap();

        let response = client.send_msg(user.to_string(), text.to_string()).await?;
        println!("Send message[{code}]", code = response);
    }
    Ok(())
}
