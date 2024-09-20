#[derive(Debug, Eq, Copy, Clone)]
pub enum Team {
    Party,
    Allies,
    Neutral,
    Enemy,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum DurationUnit {
    Unknown,
    Turns,
    Minutes,
    Hours,
}

impl DurationUnit {
    fn to_string(&self) -> String {
        match self {
            DurationUnit::Unknown => "".to_string(),
            DurationUnit::Turns => "Turns".to_string(),
            DurationUnit::Minutes => "Minutes".to_string(),
            DurationUnit::Hours => "Hours".to_string(),
        }
    }
}

impl From<String> for Team {
    fn from(team: String) -> Self {
        match team.as_str() {
            "p" => Team::Party,
            "a" => Team::Allies,
            "n" => Team::Neutral,
            "e" => Team::Enemy,
            "party" => Team::Party,
            "ally" => Team::Allies,
            "allies" => Team::Allies,
            "neutral" => Team::Neutral,
            "enemy" => Team::Enemy,
            _ => Team::Unknown,
        }
    }
}

impl Team {
    fn to_string(&self) -> String {
        match self {
            Team::Party => "Party".to_string(),
            Team::Allies => "Allies".to_string(),
            Team::Neutral => "Neutral".to_string(),
            Team::Enemy => "Enemy".to_string(),
            Team::Unknown => "Unknown Team".to_string(),
        }
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // ordering: 1st: party, 2nd : allies, 3rd: enemy, 4th: neutral
        match self {
            Team::Party => match other {
                Team::Allies => std::cmp::Ordering::Less,
                Team::Enemy => std::cmp::Ordering::Less,
                Team::Neutral => std::cmp::Ordering::Less,
                Team::Unknown => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            },
            Team::Allies => match other {
                Team::Neutral => std::cmp::Ordering::Less,
                Team::Enemy => std::cmp::Ordering::Less,
                Team::Party => std::cmp::Ordering::Greater,
                Team::Unknown => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            },
            Team::Enemy => match other {
                Team::Neutral => std::cmp::Ordering::Less,
                Team::Allies => std::cmp::Ordering::Greater,
                Team::Party => std::cmp::Ordering::Greater,
                Team::Unknown => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            },
            Team::Unknown => std::cmp::Ordering::Greater,
            Team::Neutral => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<String> for DurationUnit {
    fn from(unit: String) -> Self {
        match unit.as_str() {
            "turns" => DurationUnit::Turns,
            "minutes" => DurationUnit::Minutes,
            "hours" => DurationUnit::Hours,
            "t" => DurationUnit::Turns,
            "m" => DurationUnit::Minutes,
            "h" => DurationUnit::Hours,
            _ => DurationUnit::Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Duration {
    length: u16,
    unit: DurationUnit,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct StatusEffect {
    name: String,
    duration: Duration,
}

#[derive(Debug, Clone)]
pub struct Entity {
    name: String,
    damage_taken: u16,
    team: Team,
    status_effects: Vec<StatusEffect>,
}

#[derive(Default, Clone)]
pub struct Game {
    entities: Vec<Entity>,
}

fn argumment_abreviations(arg: &str) -> &str {
    match arg {
        "as" => "add_effect",
        "rs" => "remove_effect",
        "ae" => "add_entity",
        "re" => "remove_entity",
        "d" => "damage",
        "h" => "heal",
        _ => arg,
    }
}

fn save_party(entities: Game, filename : String) -> Result<String, String> {
    let mut string = String::new();
    for entity in entities.entities.iter() {
        if entity.team == Team::Party {
            string.push_str(&entity.name);
            string.push_str("\n");
        }
    }
    //write to a file
    match std::fs::write("saves/".to_string() + &filename + ".txt", string) {
        Ok(_) => Ok("Saved Party".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

fn load_party(filename : String) -> Result<Vec<Entity>, String> {
    //load from file
    match std::fs::read_to_string("saves/".to_string() + &filename + ".txt") {
        Ok(contents) => {
            let mut entities: Vec<Entity> = Vec::new();
            for line in contents.lines() {
                entities.push(Entity::new(line.to_string(), Team::Party));
            }
            Ok(entities)
        },
        Err(e) => Err(e.to_string()),
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            entities: Vec::new(),
        }
    }

    pub fn get_entities_list(&mut self) -> String {
        let mut list = String::new();
        self.entities.sort_by(|a, b| a.team.cmp(&b.team));
        let mut last_team = Team::Unknown;
        for entity in self.entities.iter() {
            if entity.team != last_team {
                list.push_str("\n");
                list.push_str(&entity.team.to_string());
                list.push_str("\n");
            }
            last_team = entity.team;
            list.push_str(&entity.name);
            list.push_str(", Damage Taken: ");
            list.push_str(&entity.damage_taken.to_string());
            if entity.status_effects.len() > 0 {
                list.push_str(", Status Effects: ");
            }
            for effect in entity.status_effects.iter() {
                list.push_str(&effect.name);
                list.push_str(", ");
                list.push_str(&effect.duration.length.to_string());
                list.push_str(" ");
                list.push_str(&effect.duration.unit.to_string());
                list.push_str("; ");
            }
            list.push_str("\n");
        }
        list
    }

    pub fn process_command(&mut self, command: String) -> Result<String, String> {
        let args: Vec<&str> = command.split(" ").collect();
        match argumment_abreviations(args[0]) {
            "add_effect" => {
                if args.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }
                for entity in self.entities.iter_mut() {
                    if entity.name == args[1] {
                        entity.status_effects.push(StatusEffect {
                            name : args[2].to_string(),
                            duration : Duration {
                                length : {if args.len() > 3 {args[3].parse().unwrap()} else {0}} as u16 ,
                                unit : {if args.len() > 4 {DurationUnit::from(args[4].to_string())} else {DurationUnit::Unknown}},
                            },
                        });
                        return Ok("Added effect".to_string());
                    }
                }
                Err("No Entity Found".to_string())
            },
            "remove_effect" => {
                if args.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }
                for entity in self.entities.iter_mut() {
                    if entity.name == args[1] {
                        entity.status_effects.retain(|x| x.name != args[2]);
                        break;
                    }
                }
                Ok("Removed effect".to_string())
            },
            "add_entity" => {
                if args.len() < 2 {
                    return Err("Not enough arguments".to_string());
                }
                for entity in self.entities.iter() {
                    if entity.name == args[1].to_string() {
                        return Err("This entity already exists".to_string());
                    }
                }
                self.entities.push(Entity::new(args[1].to_string(), if args.len() > 2 {Team::from(args[2].to_string())} else {Team::Unknown}));
                Ok("Added entity".to_string())
            },
            "remove_entity" => {
                if args.len() < 2 {
                    return Err("Not enough arguments".to_string());
                }
                self.entities.retain(|x| x.name != args[1]);
                Ok("Removed entity".to_string())
            },
            "damage" => {
                if args.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }
                for entity in self.entities.iter_mut() {
                    if entity.name == args[1] {
                        entity.damage_taken += args[2].parse::<u16>().unwrap() as u16;
                        break;
                    }
                }
                Ok("Damaged entity".to_string())
            },
            "heal" => {
                if args.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }
                for entity in self.entities.iter_mut() {
                    if entity.name == args[1] {
                        entity.damage_taken -= args[2].parse::<u16>().unwrap() as u16;
                        break;
                    }
                }
                Ok("Healed entity".to_string())
            },
            "save" => {
                if args.len() < 2 {
                    return Err("Not enough arguments".to_string());
                }
                save_party(self.clone(), args[1].to_string())
            },
            "load" => {
                if args.len() < 2 {
                    return Err("Not enough arguments".to_string());
                }
                match load_party(args[1].to_string()) {
                    Ok(entities) => {
                        self.entities = entities;
                        Ok("Loaded Party".to_string())
                    },
                    Err(e) => Err(e),
                }
            },
            "clear" => {
                self.entities.clear();
                Ok("Cleared entities".to_string())
            },
            "help" => {
                if args.len() == 2 {
                    match args[1] {
                        "add_entity" => {
                            return Ok("add_entity <name> <team>".to_string());
                        },
                        "remove_entity" => {
                            return Ok("remove_entity <name>".to_string());
                        },
                        "add_effect" => {
                            return Ok("add_effect <name> <effect> <length> <unit>".to_string());
                        },
                        "remove_effect" => {
                            return Ok("remove_effect <name> <effect>".to_string());
                        },
                        "damage" => {
                            return Ok("damage <name> <amount>".to_string());
                        },
                        "heal" => {
                            return Ok("heal <name> <amount>".to_string());
                        },
                        "clear" => {
                            return Ok("clear".to_string());
                        },
                        "save" => {
                            return Ok("save <filename>".to_string());
                        },
                        "load" => {
                            return Ok("load <filename>".to_string());
                        },
                        _ => {
                            return Ok("Valid Commands: add_entity, remove_entity, add_effect, remove_effect, damage, heal, clear, save, load".to_string());
                        }
                    }
                }
                Ok("Valid Commands: add_entity, remove_entity, add_effect, remove_effect, damage, heal, clear, save, load. Use help <command> for more info".to_string())
            }
            _ => {
                Err("Unrecognized command use help to list commands".to_string())
            }
        }
    }
}

impl Entity {
    pub fn new(name: String, team: Team) -> Entity {
        Entity {
            name: name.to_string(),
            damage_taken: 0,
            team: team,
            status_effects: Vec::new(),
        }
    }
}
