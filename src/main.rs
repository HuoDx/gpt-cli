mod chat;
mod config;
mod api;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    match &argv[..] {
        [_,cmd, sub@..]=> {
            match &cmd[..] { // refactor later
                "chat" => chat::run(sub),
                "config" => config::run(sub),
                _ => println!("Unknown function name: {}", cmd)
            }
        },
        _=>println!("gpt-cli chat|config [params]")
    }
}
