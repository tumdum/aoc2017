fn insert(b: &mut Vec<usize>, pos: usize, value: usize) -> usize {
    let pos = pos % b.len() + 1;
    b.insert(pos, value);
    pos
}

fn solve_a(iterations: usize, step: usize) -> usize {
    let mut buf = vec![0];
    let mut pos = 0;

    for _ in 0..iterations {
        let len = buf.len();
        pos = insert(&mut buf, pos + step, len);
    }

    buf[(pos+1)%buf.len()]
}

fn solve_b(iterations: usize, step: usize) -> usize {
    let mut pos = 0;
    let mut counter = 1;
    let mut last = 0;

    for i in 1..iterations {
        pos = (pos + step) % counter + 1;
        if pos == 1 {
            last = i;
        }
        counter += 1;
    }
    last
}

fn main() {
    println!("{}", solve_a(2017, 348));
    println!("{}", solve_b(50000000, 348));
}

#[test]
fn solve_a_test() {
    assert_eq!(638, solve_a(2017, 3));
    assert_eq!(417, solve_a(2017, 348));
}

#[test]
fn solve_b_test() {
    assert_eq!(34334221, solve_b(50000000, 348));
}

#[test]
fn insert_test() {
    let mut v = vec![0];
    let mut pos = 0;

    pos = insert(&mut v, pos + 3, 1);
    assert_eq!(vec![0, 1], v);
    assert_eq!(1, pos);

    pos = insert(&mut v, pos + 3, 2);
    assert_eq!(vec![0, 2, 1], v);
    assert_eq!(1, pos);

    pos = insert(&mut v, pos + 3, 3);
    assert_eq!(vec![0, 2, 3, 1], v);
    assert_eq!(2, pos);

    pos = insert(&mut v, pos + 3, 4);
    assert_eq!(vec![0, 2, 4, 3, 1], v);
    assert_eq!(2, pos);
}
