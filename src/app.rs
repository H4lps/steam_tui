use std::path::PathBuf;
use std::io;
use std::fs;
use std::error::Error;
pub struct App {
    steam_path : PathBuf,
    selected_index: usize,
    games: Vec<Result<Game,std::io::Error>>,
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
        fn parse_acf(path: &PathBuf) -> Result<Game,Box<dyn Error> > {
            let content = fs::read_to_string(path)?;
        
            let mut app_id : Option<u32>  = None;
            let mut name : Option<String> = None;
            let mut install_dir : Option<PathBuf> = None;

            for line in content.lines() {
                let parts : Vec<&str> = line
                    .split('"')
                    .filter(|s| !s.trim().is_empty())
                    .collect();
                
                if parts.len() >= 2 {
                    match parts[0] {
                       "appid" => app_id = parts[1].parse::<u32>().ok(),
                        "name" => {
                            let name = parts[1].to_string();
                            if name.to_lowercase().contains("steam") {
                                continue;
                            }

                            Some(name)
                        },
                        "installdir" => install_dir = Some(
                            PathBuf::from(parts[1].to_string())
                        ),
                    }
                }
            }

            let app_id = app_id.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Parse or Missing")
            })?;

            let name = name.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "missing name")
            })?;

            let install_dir = install_dir.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "missing installdir")
            })?;


            Ok(Game {
                app_id: app_id,
                name: name,
                install_dir: install_dir,
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
