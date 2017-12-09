use std::io::{Cursor,Read,BufRead,BufReader};

#[derive(PartialEq,Debug)]
struct Result{groups: i32, score: i32, garbage: i32}

fn count_groups<R: Read>(r: R) -> Result {
    let mut score = 0;
    let mut depth = 0;
    let mut i = 0;
    let mut in_garbage = false;
    let mut garbage = 0;
    let mut last = None;
    for c in r.bytes().map(|b| b.unwrap() as char) {
        if last == Some('!') {
            last = None;
            if in_garbage {
                garbage -= 1;
            }
            continue;
        } else if c != '>' && in_garbage {
            last = Some(c);
            garbage += 1;
            continue;
        }
        match c {
            '{' => {
                    i+=1;
                    depth+=1;
            }
            '<' => { in_garbage = true; },
            '>' => { in_garbage = false; },
            '}' => {
                score += depth;
                depth -= 1;
            },
            _ => {},
        }
        last = Some(c);
    }
    Result{groups: i, score: score, garbage: garbage}
}

fn main() {
    println!("{:?}", count_groups(std::io::stdin()));
}

#[test]
fn test_count_groups() {
    assert_eq!(Result{groups: 1, score:1,  garbage: 0}, count_groups(Cursor::new("{}")));
    assert_eq!(Result{groups: 3, score:6,  garbage: 0}, count_groups(Cursor::new("{{{}}}")));
    assert_eq!(Result{groups: 3, score:5,  garbage: 0}, count_groups(Cursor::new("{{},{}}")));
    assert_eq!(Result{groups: 6, score:16, garbage: 0}, count_groups(Cursor::new("{{{},{},{{}}}}")));
    assert_eq!(Result{groups: 1, score:1,  garbage: 10}, count_groups(Cursor::new("{<{},{},{{}}>}")));
    assert_eq!(Result{groups: 1, score:1,  garbage: 4}, count_groups(Cursor::new("{<a>,<a>,<a>,<a>}")));
    assert_eq!(Result{groups: 5, score:9,  garbage: 4}, count_groups(Cursor::new("{{<a>},{<a>},{<a>},{<a>}}")));
    assert_eq!(Result{groups: 2, score:3,  garbage: 13}, count_groups(Cursor::new("{{<!>},{<!>},{<!>},{<a>}}")));

    assert_eq!(Result{groups: 1, score:1,  garbage: 0}, count_groups(Cursor::new("{<>}")));
    assert_eq!(Result{groups: 1, score:1,  garbage: 17}, count_groups(Cursor::new("{<random characters>}")));
    assert_eq!(Result{groups: 1, score:1,  garbage: 3}, count_groups(Cursor::new("{<<<<>}")));
    assert_eq!(Result{groups: 1, score:1,  garbage: 2}, count_groups(Cursor::new("{<{!>}>}")));
    assert_eq!(Result{groups: 1, score:1,  garbage: 0}, count_groups(Cursor::new("{<!!>}")));
    assert_eq!(Result{groups: 1, score:1,  garbage: 0}, count_groups(Cursor::new("{<!!!>>}")));
    assert_eq!(Result{groups: 1, score:1,  garbage: 10}, count_groups(Cursor::new(r#"{<{o"i!a,<{i<a>}"#)));
    assert_eq!(Result{groups: 2, score:3,  garbage: 0}, count_groups(Cursor::new("{!!{}}")));
}
