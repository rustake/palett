struct Foo<'lifetime> {
    content: &'lifetime str
}

#[test]
fn lifetime_test() {
    let r;
    {
        let s1 = "rust";
        let s2 = "ecmascript";
        r = longer(s1, s2);
    }
    println!("{} is longer", r);

    let content;
    {
        let detail: &str = "bar";
        let foo = Foo { content: detail };
        content = foo.content;
    }
    println!("{}", content);

    let foo = Foo {
        content: "string_slice"
    };
    println!("s.content = {}", foo.content);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'b>(x: &'b str, y: &'b str) -> &'b str { if x.len() > y.len() { x } else { y } }

// 指定 x & y 必须拥有相同的生命周期，且返回值也具有同样生命周期
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str { if y.len() > x.len() { y } else { x } }
