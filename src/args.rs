pub struct Args {
    help: bool,
    word_list: Option<String>
}

impl Args {
    pub const HELP_MESSAGE: &str = "
NAME
    word_game - Wordle in terminal
SYNOPSIS
    word_game [-h] [WORD_LIST]
DESCRIPTION
    Play wordle in terminal

    -h, --help
        display this help and exit.
    WORD_LIST
        A text file containing the words each written on new line.
        Should contain at least 1 word.
        If no specify, the program will use default word list.
";

    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut args = args.into_iter();
        match args.len() {
            1 => Self {
                help: false,
                word_list: None
            },
            2 => {
                let arg = args.nth(1).unwrap();
                if arg == "-h" || arg == "--help" {
                    Self {
                        help: true,
                        word_list: None
                    }
                }
                else {
                    Self {
                        help: false,
                        word_list: Some(arg)
                    }
                }
            },
            _ => Self {
                help: true,
                word_list: None
            }
        }
    }

    pub fn help(&self) -> bool {
        self.help
    }

    pub fn word_list(&self) -> Option<&String> {
        self.word_list.as_ref()
    }
}
