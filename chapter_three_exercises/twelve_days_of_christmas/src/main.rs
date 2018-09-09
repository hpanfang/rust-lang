fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four colly birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
                          "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut count = 0;
    while count < 12 {
        println!("On the {} day of christmas my true love gave to me", days[count]);
        for gift in (0..count+1).rev() {
            println!("{}", gifts[gift]);
        }
        println!();
        count += 1;
    }
}
