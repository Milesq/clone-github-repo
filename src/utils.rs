use std::process::Output;
mod match_repo_adress;
pub use match_repo_adress::*;

fn execute_switch(args: Vec<String>, actions: Vec<(&[&str], impl Fn())>) -> bool {
    let is_switch_set = |switches: &[&str]| {
        args.iter()
            .any(|el| switches.iter().any(|switch| switch == el))
    };

    for (args, handler) in actions {
        if is_switch_set(args) {
            handler();
            return true;
        }
    }

    false
}

pub fn get_message(obj: Output) -> String {
    String::from_utf8(if !obj.stdout.is_empty() {
        obj.stdout
    } else {
        obj.stderr
    })
    .unwrap()
}

type ArgsHandler<'a, 'b> = (&'a [&'b str], fn());

pub fn preparse_args(args: Vec<String>) -> bool {
    let actions: Vec<ArgsHandler> = vec![
        (&["-h", "--help"], || println!("{}", crate::HELP_MSG)),
        (&["-c", "--clean"], || {
            std::fs::remove_file(dirs::home_dir().unwrap().join("./clone-cfg.bin")).unwrap();
            println!("Clean");
        }),
    ];

    execute_switch(args, actions)
}
