use crate::sheet::{
    get_first_word, 
    parse_string_lines, 
    is_mappable, 
    map_lines,
};

use crate::mapline::{
    MapLine, 
    Mode, 
    get_desc,
    identify_mode,
    filter_attrs,
    identify_trigger,
};

#[test]
fn test_string_line_parser() {
    let base = String::from("nnoremap <silent> <A-h> :wincmd h<CR>\nnoremap <silent> <A-l> :wincmd l<CR>");

    let expected = vec!["nnoremap <silent> <A-h> :wincmd h<CR>", "noremap <silent> <A-l> :wincmd l<CR>"].iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let result = parse_string_lines(&base);

    assert_eq!(expected, result);
}

#[test]
fn test_get_first_word() {
    // Test with an Empty String
    let mut result = get_first_word(&String::from(""));
    assert_eq!(result, None);

    // Test with a complete line
    result = get_first_word(&String::from("noremap <silent> <A-h> :wincmd h<CR>"));
    assert_eq!(result, Some(String::from("noremap")));
}

#[test]
fn test_map_lines() {
    let base = String::from("nnoremap <silent> <A-h> :wincmd h<CR>\nnoremap <silent> <A-l> :wincmd l<CR>");

    let expected = vec![MapLine::new(String::from("nnoremap <silent> <A-h> :wincmd h<CR>")), MapLine::new(String::from("noremap <silent> <A-l> :wincmd l<CR>"))];

    // TODO: Figure out why this is considered unused
    let _result = map_lines(parse_string_lines(&base));

    assert!(matches!(expected, _result));
}

#[test]
fn test_is_mappable() {
    // Test a mappable line
    let mut base = String::from("noremap <silent> <A-h> :wincmd h<CR>");

    let mut expected = true;

    let mut result = is_mappable(&base);

    assert_eq!(expected, result);

    // Test a non-mappable line
    base = String::from("\" This is a non-mappable description!");

    expected = false;

    result = is_mappable(&base);

    assert_eq!(expected, result);
}

#[test]
fn test_mode_identification() {
    // Test all of the different modes

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
