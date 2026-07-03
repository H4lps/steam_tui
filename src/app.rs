use std::path::Pathbuf;
use std::io;
use std::fs;

struct App {
    selected_index: usize,
    games: Vec<Game>,
    search: String,
    should_quit: bool,
}

struct Game {
    app_id: u32,
    name: String,
    install_dir: Pathbuf,
}

impl App { 
    pub fn new(


    //Loading game on cold start
    fn find_valid_steam_games(path: Pathbuf) -> Vec<io::result<Game>> {
        let mut games = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("acf") {
                let game = parse_acf(&path)?;
                games.push(game);   
            }
        }

    }

    //Manifest Parsing
    fn parse_acf(path: &Pathbuf) -> io::Result<Game> {
        let contents = fs::read_to_string(path)?;
        
        let mut app_id = None;
        let mut name = None;
        let mut install_dir = None;

        for line in content.lines() {
            let parts : Vec<&str> = line
                .split('""')
                .filter(|s| !s.trim().is_empty())
                .collect();
            
            if parts.len() >= 2 {
                match parts[0] {
                   "appid" => app_id = parts[1]::parse<u32>()
                        .expect("Steam manifest contains an invalid appid)",
                    "name" => {
                        let name = parts[1].to_string()
                                .expect("Invalid conversion to string");

                        if name.to_lowercase().conatins("steam") {
                            continue;
                        }

                        Some(name)
                    },
                    "installdir" => install_dir = Some(parts[1].to_string()),
                }

            }
        }


        Ok(Game {
            app_id: app_id,
            name: name.unwrap_or_else("InvalidName".to_string()),
            installdir: name.unwrap_or_else("InvalidInstallDir".to_string(),
        })
    }
        

}

impl Game {

    pub fn new(app_id: u32, name: String, install_dir: Pathbuf) {
        Self {
            app_id,
            name,
            install_dir,
        }
    }
}
