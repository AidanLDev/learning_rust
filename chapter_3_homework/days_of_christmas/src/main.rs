fn main() {
    println!("Lets sing along now shall we?");
    /* Write a program (taking advantage of the songs repetition" that prints the lyrics to the 12
     * days of Christmas
     */
    let mut day = 1;
    let gift = [
        "partridge in a pear tree",
        "Two turtule doves",
        "Three French Hens",
        "Four calling birds",
        "Five gold rings...",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    while day < 13 {
        let num_postfix: &str;
        num_postfix = match day {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!("On the {day}{num_postfix} day of Christmas my ture love gave to me.");

        let mut day_index = day;

        while day_index > 0 {
            if day == 1 {
                println!("A {}", gift[day_index -1]);
            } else if day > 1 && day_index == 1 {
                println!("and a {}...", gift[day_index -1]);
            } else {
                println!("{}", gift[day_index -1]);
            }
            day_index -= 1;
        }
        day += 1;
    }
}
