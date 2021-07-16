// Primitive str is immutable, fixed-length string somewhere in memory.
// String is a growable, heap-allocated data structure. Use when you need to modify or own string data.

pub fn run() {
    // str
    let hello = "Hello";

    // String
    let mut bye = String::from("Bye bye!");

    println!("{} and {}", hello, bye);
    println!("Length of bye: {}", bye.len());
    bye.push('\u{1F980}');
    bye.push_str("Crab");
    bye = bye + "!!!";
    println!("And here's bye with crab: {}", bye);

    // Contain aka. strstr
    println!("Does crab contain 'rab'? {}", bye.contains("rab"));

    // Loop through string by whitespace
    for word in "asb asn thn ksm tgh".split_whitespace() {
        println!("{} ", word);
    }

    // Assertion testing
    assert_eq!(19, bye.len());

    // String literals are always Unicode.
    // String literals type are &'static str:
    // & meaning that it's referring to a place in memory, and it lacks a &mut meaning that the compiler will not allow modification
    // 'static meaning the string data will be available till the end of our program (it never drops)
    // str means that it points to a sequence of bytes that are always valid utf-8

    let a: &'static str = "hi 🦀";
    println!("{} {}", a, a.len());

    println!(
        "helloooo
    world"
    ); // notice that the spacing before w is ignored

    let html: &'static str = r#"
    <div class="advice">
        Raw strings are useful for some situations.
    </div>
    "#;
    println!("{}", html);

    // If you have some very large text, consider using the macro include_str! to include text from local files in your program:
    let gitignore = include_str!("../../.gitignore");
    println!("{}", gitignore);

    // Concat and join
    let helloworld = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}", abc);

    // Strings are different from C. Because of unicode, you can't just jump around
    // with indexes. Use something along these lines:
    let k = "kylpynalle";
    let tempchar = k.chars().nth(3).unwrap();
    println!("Character in index 3 is {}", tempchar);

    // You can iterate characters with char_indices
    fn string_find_a(s: &str) -> &str {
        for (n, x) in s.char_indices() {
            if x == 'a' {
                return &s[n..];
            }
        }
        s
    }
    println!("{:?}", string_find_a("kylpynalle"));
}
