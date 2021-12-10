use std::str;

#[test]
fn test() {
    let mut buffer: Vec<u8> = vec![];
    quilt_lang::run("examples/hello_world.png", 1, &mut buffer);
    let s = str::from_utf8(&buffer).unwrap();
    assert_eq!(s, "Hello world!");
}

#[test]
fn test1() {
    let mut buffer: Vec<u8> = vec![];
    quilt_lang::run("examples/hello_world_elaborate.png", 1, &mut buffer);
    let s = str::from_utf8(&buffer).unwrap();
    assert_eq!(s, "Hello world!\n");
}
