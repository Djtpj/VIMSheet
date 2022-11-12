#[derive(Debug)]
pub struct MapLine {
    mode: Mode,
    trigger: String,
    description: String,
}

impl MapLine {
    pub fn new(unfiltered: String) -> Self {
        let line = filter_attrs(&unfiltered);
        
        MapLine {
            mode: identify_mode(&line),
            trigger: identify_trigger(&line),
            description: get_desc(&line),
        }
    }

    pub fn serialize(&self) -> [String; 3] {
        let mode = serialize_mode(&self.mode); 
        let trigger = &self.trigger;
        let description = &self.description;

        return [description.clone(), trigger.clone(), mode.clone()];
    }
}

impl std::fmt::Display for MapLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.mode, self.trigger, self.description)
    }
}

#[derive(Debug)]
pub enum Mode {
    INSERT,
    NORMAL,
    ALL,
    VISUAL,
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        match self {
            Mode::INSERT => Mode::INSERT,
            Mode::NORMAL => Mode::NORMAL,
            Mode::ALL => Mode::ALL,
            Mode::VISUAL => Mode::VISUAL 
        }  
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serialize_mode(self))
    }
}

fn serialize_mode(mode: &Mode) -> String {
    match mode {
        Mode::INSERT => String::from("Insert"),
        Mode::NORMAL => String::from("Normal"),
        Mode::ALL => String::from("All"),
        Mode::VISUAL => String::from("Visual"),
    }
}

pub fn identify_mode(line: &String) -> Mode {
    match line.chars().next().unwrap() {
        'i' => Mode::INSERT,
        'n' => Mode::NORMAL,
        'v' => Mode::VISUAL,
        'm' | _ => Mode::ALL,
    }
}

pub fn filter_attrs(line: &String) -> String{
    line
        .replace(" <expr>", "")
        .replace(" <buffer>", "")
        .replace(" <silent>", "")
        .replace(" <script>", "")
        .replace(" <unique>", "")
        .replace(" <special>", "")
}

pub fn identify_trigger(line: &String) -> String {
   let split = line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>(); 
   split.get(1).unwrap().clone()
}

pub fn get_desc(line: &String) -> String {
    let split = line.split("\"").map(|s| s.to_string()).collect::<Vec<String>>();

    // Get inline comment
    let desc = split.get(split.len() - 1).unwrap().clone().trim().to_string();

    if split.len() <= 1 || // If there isn't a inline comment
        desc.trim().is_empty() // Or if the line is empty
        { return String::from("No Description.") }

    return desc;
}
