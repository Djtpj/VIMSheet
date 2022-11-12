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

    pub fn get_info(&self) -> (Mode, String, String) {
        (self.mode.clone(), self.trigger.clone(), self.description.clone())
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

fn identify_mode(line: &String) -> Mode {
    match line.chars().next().unwrap() {
        'i' => Mode::INSERT,
        'n' => Mode::NORMAL,
        'v' => Mode::VISUAL,
        'm' | _ => Mode::ALL,
    }
}

fn filter_attrs(line: &String) -> String{
    line
        .replace(" <expr>", "")
        .replace(" <buffer>", "")
        .replace(" <silent>", "")
        .replace(" <script>", "")
        .replace(" <unique>", "")
        .replace(" <special>", "")
}

fn identify_trigger(line: &String) -> String {
   let split = line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>(); 
   split.get(1).unwrap().clone()
}

fn get_desc(line: &String) -> String {
    let split = line.split("\"").map(|s| s.to_string()).collect::<Vec<String>>();

    let desc = split.get(split.len() - 1).unwrap().clone().trim().to_string();

    if split.len() <= 1 || desc.trim().is_empty() { return String::from("No Description.") }

    return desc;
}

#[test]
fn test_mode_identification() {
    let insert = "inoremap";
    matches!(identify_mode(&String::from(insert)), Mode::INSERT);

    let normal = "nnoremap";
    matches!(identify_mode(&String::from(normal)), Mode::NORMAL);

    let visual = "vnoremap";
    matches!(identify_mode(&String::from(visual)), Mode::VISUAL);

    let all = "map";
    matches!(identify_mode(&String::from(all)), Mode::ALL);
}

#[test]
fn test_attr_filter() {
    let line = String::from("noremap <silent> <C-h> :tabprevious<CR>");
    let filtered = filter_attrs(&line);

    let base = "noremap <C-h> :tabprevious<CR>";

    assert_eq!(base, filtered);
}

#[test]
fn test_trigger_identification() {
    let line = String::from("noremap <silent> <C-h> :tabprevious<CR>");
    let trigger = String::from("<C-h>");

    let result = identify_trigger(&filter_attrs(&line));

    assert_eq!(trigger, result);
}

#[test]
fn test_desc_retreival() {
    let line = String::from("noremap <silent> <C-h> :tabprevious<CR> \" Flips to the previous tab");
    let desc = String::from("Flips to the previous tab");

    let result = get_desc(&line);

    assert_eq!(desc, result);
}
