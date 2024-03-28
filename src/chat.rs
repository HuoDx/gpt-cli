use crate::api::OpenAI;
use text_io::read;
pub fn run(args: &[String]) {
    let sys_prompt = "Behave like a plain UNIX bash; 
    be as concise as possible! You can choose not to respond by saying '/NR'";
    let mut api = OpenAI::new(&args[0], "gpt-4", sys_prompt);
    println!("SYSTEM: {}\n", sys_prompt);
    print!(">>> ");
    let mut user: String = read!("{}\n");
    loop {
        match &user[..] {
            "::exit" => break,
            _ => {
                let rslt = api.chat(&user);
                if !rslt.eq("/NR") {
                    println!("<<< {}", rslt);
                }
            }
        }
        print!(">>> ");
        user = read!("{}\n");
    }
}
