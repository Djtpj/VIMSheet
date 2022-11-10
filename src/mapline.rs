pub struct MapLine {
    mode: Option<Mode>,
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

}

enum Mode {
    INSERT,
    NORMAL,
    ALL,
    VISUAL,
}

fn identify_mode(line: &String) -> Option<Mode> {
    if line.is_empty() {
        return None;
    }

    match line.chars().next().unwrap() {
        'i' => Some(Mode::INSERT),
        'n' => Some(Mode::NORMAL),
        'v' => Some(Mode::VISUAL),
        'm' | _ => Some(Mode::ALL),
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
    line.split("\"").map(|s| s.to_string()).collect::<Vec<String>>().get(1).unwrap_or(&String::from("No Description.")).clone().trim().to_string()
}

#[test]
fn test_mode_identification() {
    let insert = "inoremap";
    matches!(identify_mode(&String::from(insert)).unwrap(), Mode::INSERT);

    let normal = "nnoremap";
    matches!(identify_mode(&String::from(normal)).unwrap(), Mode::NORMAL);

    let visual = "vnoremap";
    matches!(identify_mode(&String::from(visual)).unwrap(), Mode::VISUAL);

    let all = "map";
    matches!(identify_mode(&String::from(all)).unwrap(), Mode::ALL);
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
