use crate::{api::OpenAI, config::Config};
use text_io::read;
pub fn run(_: &[String]) {
    let config = Config::load();
    let sys_prompt = config.system_prompt
    .expect("Run `gpt-cli config` to configure your settings.");
    let model_name = config.model_name.expect("Run `gpt-cli config` to configure your settings.");
    let api_key = config.api_key.expect("Run `gpt-cli config` to configure your settings.");
    let mut api = OpenAI::new(&api_key, &model_name, &sys_prompt);
    println!("SYSTEM: {}\n", sys_prompt);
    print!(">>> ");
    let mut user: String = read!("{}\n");
    loop {
        match &user[..] {
            "::exit" => break,
            _ => {
                let rslt = api.chat(&user);
                if !rslt.eq("/NR") {
                    println!("\n{}\n", rslt);
                }
            }
        }
        print!(">>> ");
        user = read!("{}\n");
    }
}
