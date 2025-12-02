struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

const PARSE_ERR: &str = "Game should be in format \"Game {id}: (({nuimber} {color})*,)*;";

impl Game {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game_id, draws) = value.split_once(':').expect(PARSE_ERR);
        let id = game_id
            .split_once(' ')
            .expect(PARSE_ERR)
            .1
            .parse()
            .expect(PARSE_ERR);
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for game in draws.split(';') {
            for color in game.split(',') {
                let (n, color) = color.trim().split_once(' ').expect(PARSE_ERR);
                let n: u32 = n.parse().expect(PARSE_ERR);
                match color.trim() {
                    "red" if n > red => red = n,
                    "green" if n > green => green = n,
                    "blue" if n > blue => blue = n,
                    _ => {}
                }
            }
        }
        Self {
            id,
            red,
            green,
            blue,
        }
    }
}

impl TryFrom<(&str, u32, u32, u32)> for Game {
    type Error = String;
    fn try_from(
        (value, red_limit, green_limit, blue_limit): (&str, u32, u32, u32),
    ) -> Result<Self, Self::Error> {
        let (game_id, draws) = value.split_once(": ").expect(PARSE_ERR);
        let id = game_id
            .split_once(' ')
            .expect(PARSE_ERR)
            .1
            .parse()
            .expect(PARSE_ERR);
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for game in draws.split("; ") {
            for color in game.split(", ") {
                let (n, color) = color.split_once(' ').expect(PARSE_ERR);
                let n = n.parse().expect(PARSE_ERR);
                match color {
                    "red" => red = new_value(color, red, n, red_limit)?,
                    "green" => green = new_value(color, green, n, green_limit)?,
                    "blue" => blue = new_value(color, blue, n, blue_limit)?,
                    _ => unreachable!("{PARSE_ERR}"),
                }
            }
        }
        Ok(Self {
            id,
            red,
            green,
            blue,
        })
    }
}

fn new_value(color: &str, old: u32, new: u32, limit: u32) -> Result<u32, String> {
    if new > limit {
        return Err(format!(
            "{color} value too large: expected at most {limit}, got {new}"
        ));
    }
    Ok(if old < new { new } else { old })
}

pub fn task_1(file: &str) -> String {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;
    file.lines()
        .flat_map(|l| Game::try_from((l, red_limit, green_limit, blue_limit)))
        .map(|g: Game| g.get_id())
        .sum::<u32>()
        .to_string()
}

pub fn task_2(file: &str) -> String {
    file.lines()
        .map(Game::from)
        .map(|g: Game| g.get_power())
        .sum::<u32>()
        .to_string()
}
