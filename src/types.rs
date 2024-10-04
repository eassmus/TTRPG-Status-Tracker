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
        match team.to_lowercase().as_str() {
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

impl From<Team> for String {
    fn from(team: Team) -> Self {
        match team {
            Team::Party => "party".to_string(),
            Team::Allies => "ally".to_string(),
            Team::Neutral => "neutral".to_string(),
            Team::Enemy => "enemy".to_string(),
            Team::Unknown => "unknown".to_string(),
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

fn save(entities: Game, filename: String, team: Team) -> Result<String, String> {
    let mut string = String::new();
    for entity in entities.entities.iter() {
        if entity.team == team || team == Team::Unknown {
            string.push_str(&entity.name);
            string.push_str("|");
            string.push_str(&entity.team.to_string());
            string.push_str("\n");
        }
    }
    //write to a file
    match std::fs::write("saves/".to_string() + &filename + ".txt", string) {
        Ok(_) => Ok("Saved ".to_string() + team.to_string().as_str() + " to " + filename.as_str()),
        Err(e) => Err(e.to_string()),
    }
}

fn load(filename: String) -> Result<Vec<Entity>, String> {
    //load from file
    match std::fs::read_to_string("saves/".to_string() + &filename + ".txt") {
        Ok(contents) => {
            let mut entities: Vec<Entity> = Vec::new();
            for line in contents.lines() {
                let line = line.split("|").collect::<Vec<&str>>();
                if line.len() != 2 {
                    return Err("Invalid save file".to_string());
                }
                entities.push(Entity::new(
                    line[0].to_string(),
                    Team::from(line[1].to_string()),
                ));
            }
            Ok(entities)
        }
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

    pub fn get_matchable_names(&mut self) -> Vec<String> {
        let mut matchables = Vec::new();
        for entity in self.entities.iter() {
            matchables.push(entity.name.clone());
        }
        matchables
    }

    pub fn process_command(&mut self, command: String) -> Result<String, String> {
        let args: Vec<&str> = command.split(" ").collect();
        match argumment_abreviations(args[0]) {
            "add_effect" => {
                if args.len() < 4 {
                    return Err("Not enough arguments".to_string());
                }
                let entity_names =
                    Vec::from_iter(args[4..].iter().map(|x| x.to_string().to_lowercase()));
                let effect = args[1].to_string();
                let duration = args[2].parse().unwrap_or_else(|_| 0);
                let duration_unit = DurationUnit::from(args[3].to_string());
                for entity in self.entities.iter_mut() {
                    if entity_names.contains(&entity.name.to_lowercase()) {
                        entity.status_effects.push(StatusEffect {
                            name: effect.clone(),
                            duration: Duration {
                                length: duration,
                                unit: duration_unit,
                            },
                        });
                    }
                }
                Ok("Added effects".to_string())
            }
            "remove_effect" => {
                if args.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }
                let effect = args[1].to_string();
                let entity_names =
                    Vec::from_iter(args[2..].iter().map(|x| x.to_string().to_lowercase()));
                for entity in self.entities.iter_mut() {
                    if entity_names.contains(&entity.name.to_lowercase()) {
                        entity.status_effects.retain(|x| x.name != effect);
                    }
                }
                Ok("Removed effects".to_string())
            }
            "add_entity" => {
                if args.len() < 2 {
                    return Err("Not enough arguments".to_string());
                }
                for entity in self.entities.iter() {
                    if entity.name.to_lowercase() == args[1].to_string().to_lowercase() {
                        return Err("This entity already exists".to_string());
                    }
                }
                self.entities.push(Entity::new(
                    args[1].to_string(),
                    if args.len() > 2 {
                        Team::from(args[2].to_string())
                    } else {
                        Team::Unknown
                    },
                ));
                Ok("Added entity".to_string())
            }
            "remove_entity" => {
                if args.len() < 2 {
                    return Err("Not enough arguments".to_string());
                }
                self.entities
                    .retain(|x| x.name.to_lowercase() != args[1].to_lowercase());
                Ok("Removed entity".to_string())
            }
            "damage" => {
                if args.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }
                let damage = args[1].parse::<u16>();
                match damage {
                    Ok(damage_amount) => {
                        let entity_names =
                            Vec::from_iter(args[2..].iter().map(|x| x.to_string().to_lowercase()));
                        for entity in self.entities.iter_mut() {
                            if entity_names.contains(&entity.name.to_lowercase()) {
                                entity.damage_taken += damage_amount;
                            }
                        }
                        Ok("Damaged entities".to_string())
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            "heal" => {
                if args.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }
                let healing = args[1].parse::<u16>();
                match healing {
                    Ok(healing_amount) => {
                        let entity_names =
                            Vec::from_iter(args[2..].iter().map(|x| x.to_string().to_lowercase()));
                        for entity in self.entities.iter_mut() {
                            if entity_names.contains(&entity.name.to_lowercase()) {
                                entity.damage_taken -= healing_amount;
                            }
                        }
                        Ok("Healed entities".to_string())
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            "save" => {
                if args.len() < 3 {
                    return Err("Not enough arguments".to_string());
                }
                save(
                    self.clone(),
                    args[2].to_string(),
                    Team::from(args[1].to_string()),
                )
            }
            "load" => {
                if args.len() < 2 {
                    return Err("Not enough arguments".to_string());
                }
                match load(args[1].to_string()) {
                    Ok(entities) => {
                        for entity in entities {
                            self.entities.push(entity);
                        }
                        Ok("Loaded".to_string())
                    }
                    Err(e) => Err(e),
                }
            }
            "clear" => {
                self.entities.clear();
                Ok("Cleared entities".to_string())
            }
            "help" => {
                if args.len() == 2 {
                    match args[1] {
                        "add_entity" => {
                            return Ok("add_entity <name> <team>".to_string());
                        }
                        "remove_entity" => {
                            return Ok("remove_entity <name>".to_string());
                        }
                        "add_effect" => {
                            return Ok("add_effect <effect> <length> <unit> <names[]>".to_string());
                        }
                        "remove_effect" => {
                            return Ok("remove_effect <effect> <names[]>".to_string());
                        }
                        "damage" => {
                            return Ok("damage <amount> <names[]>".to_string());
                        }
                        "heal" => {
                            return Ok("heal <amount> <names[]>".to_string());
                        }
                        "clear" => {
                            return Ok("clear".to_string());
                        }
                        "save" => {
                            return Ok("save <party | enemy | all> <filename>".to_string());
                        }
                        "load" => {
                            return Ok("load <filename>".to_string());
                        }
                        _ => {
                            return Ok("Valid Commands: add_entity, remove_entity, add_effect, remove_effect, damage, heal, clear, save, load".to_string());
                        }
                    }
                }
                Ok("Valid Commands: add_entity, remove_entity, add_effect, remove_effect, damage, heal, clear, save, load. Use help <command> for more info".to_string())
            }
            _ => Err("Unrecognized command use help to list commands".to_string()),
        }
    }
}
