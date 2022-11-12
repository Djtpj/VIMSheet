use crate::{get_first_word, parse_string_lines, is_mappable, mapline::MapLine, map_lines};

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
