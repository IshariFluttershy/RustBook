
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(v: &Vec<String>) -> Config {
        if v.len() < 3 {
            panic!("Not enough arguments");
        }

        Config {
            query : v[1].to_string(),
            filename : v[2].to_string(),
        }
    }
}

pub fn run(conf : &Config) -> std::io::Result<()> {
    let contents = std::fs::read_to_string(&conf.filename)?;

    println!("");
    println!("Searching for {}", conf.query);
    println!("In file {}", conf.filename);
    println!("");

    grep_file(&contents[..], &conf.query[..]);

    Ok(())
}

fn print_if_contains(s: &str, pat: &str) {
    if s.contains(pat) {
        println!("{}", s);
    }
}

fn grep_file(f: &str, pat: &str) {
    let mut lines = f.lines();
    loop {
        if let Some(l) = lines.next() {
            print_if_contains(l, pat);
        } else {
            break;
        }
    }
}


