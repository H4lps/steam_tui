use std::path::PathBuf;
use std::io;
use std::fs;

pub struct App {
    steam_path : PathBuf,
    selected_index: usize,
    games: Vec<Game>,
    search: String,
    should_quit: bool,
}

struct Game {
    app_id: u32,
    name: String,
    install_dir: PathBuf,
}

impl App { 
    pub fn new(
        steam_path: PathBuf,
        selected_index: usize,
        search: String,
        should_quit: bool
        ) -> Result<Self,io::Error> {
        let games = Self::find_valid_steam_games(steam_path.clone())?;
        
        Ok(Self {
            steam_path,
            selected_index,
            search,
            should_quit,
            games,
        })
        
    }


    //Loading game on cold start
        fn find_valid_steam_games(path: PathBuf)
            -> io::Result<Vec<io::Result<Game>>> {
            let mut games = Vec::new();
     
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("acf") {
                    let game = Self::parse_acf(&path)?;
                    games.push(Ok(game));   
                }
            }
            Ok(games)
        }

        //Manifest Parsing
        fn parse_acf(path: &PathBuf) -> io::Result<Game> {
        let content = fs::read_to_string(path)?;
        
        let mut app_id = None;
        let mut name = None;
        let mut install_dir = None;

        for line in content.lines() {
            let parts : Vec<&str> = line
                .split('"')
                .filter(|s| !s.trim().is_empty())
                .collect();
            
            if parts.len() >= 2 {
                match parts[0] {
                   "appid" => app_id = parts[1].parse::<u32>()
                        .expect("Steam manifest contains an invalid appid"),
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
            app_id: app_id.unwrap_or(6767),
            name: name.unwrap_or_else(|| "InvalidName".to_string()),
            install_dir: name
                .unwrap_or_else(|| "InvalidInstallDir".to_string())
                .into(),
        })
    }
        

}

impl Game {

    pub fn new(app_id: u32, name: String, install_dir: PathBuf) -> Game{
        Self {
            app_id,
            name,
            install_dir,
        }
    }
}
