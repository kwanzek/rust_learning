fn main() {
    run_carol();
}

fn run_carol() {
    for i in 0..12 {
        if i == 0 {
            let prefix = get_day_string(i);
            println!("{}a {}\n", prefix, get_carol_line_for_day(i));
        }
        else {
            let mut output = get_day_string(i);
            for j in (0..i+1).rev() {
                if j == 0 {
                    output.push_str("and a ");
                    output.push_str(&get_carol_line_for_day(j));
                } else {
                output.push_str(&get_carol_line_for_day(j));
                output.push_str("\n");
                }
            }
            println!("{}\n", output);
        }
    }
}

fn get_day_string(day: i32) -> String {
    let ordinal = get_ordinal_for_day(day);
    format!("On the {} day of Christmas my true love gave to me\n", ordinal)
}

fn get_carol_line_for_day(day: i32) -> String {
    String::from([
        "Partridge in a pear tree",
        "Two Turle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese-a-Laying",
        "Seven Swans-a-Swimming",
        "Eight Maid-a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords-a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ][day as usize])
}

fn get_ordinal_for_day(day: i32) -> String {
    String::from([
        "First",
        "Second",
        "Third",
        "Fourth",
        "Fifth",
        "Sixth",
        "Seventh",
        "Eighth",
        "Nineth",
        "Tenth",
        "Eleventh",
        "Twelveth"
    ][day as usize]) 
}