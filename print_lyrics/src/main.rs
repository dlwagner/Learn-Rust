fn main() {
    let mut _index = 0;
    let mut done = false;
    let mut day = [""; 12];
    day[0] = "first";
    day[1] = "second";
    day[2] = "third";
    day[3] = "fourth";
    day[4] = "fifth";
    day[5] = "sixth";
    day[6] = "seventh";
    day[7] = "eighth";
    day[8] = "ninth";
    day[9] = "tenth";
    day[10] = "eleventh";
    day[11] = "twelfth";

    let mut numbers = [""; 12];
    numbers[0] = "";
    numbers[1] = "2 ";
    numbers[2] = "3 ";
    numbers[3] = "4 ";
    numbers[4] = "5 ";
    numbers[5] = "6 ";
    numbers[6] = "7 ";
    numbers[7] = "8 ";
    numbers[8] = "9 ";
    numbers[9] = "10 ";
    numbers[10] = "11 ";
    numbers[11] = "12 ";

    let mut gifts = [""; 12];
    gifts[0] = "A Partridge in a Pear Tree";
    gifts[1] = "Turtle Doves
    ";
    gifts[2] = "French Hens
    ";
    gifts[3] = "Calling Birds
    ";
    gifts[4] = "Golden Rings
    ";
    gifts[5] = "Geese a Laying
    ";
    gifts[6] = "Swans a Swimming
    ";
    gifts[7] = "Maids a Milking
    ";
    gifts[8] = "Ladies Dancing
    ";
    gifts[9] = "Lords a Leaping
    ";
    gifts[10] = "Pipers Piping
    ";
    gifts[11] = "Drummers Drumming
    ";

    let mut song = String::new();
    while !done {
        song.push_str("On the ");
        song.push_str(day[_index]);
        song.push_str(
            " day of Christmas my true love sent to me: 
        ",
        );

        for x in (0.._index + 1).rev() {
            if _index == 0 {
                song.push_str(gifts[x]);
            } else {
                if x == 0 {
                    song.push_str("and ");
                    song.push_str(numbers[x]);
                    song.push_str(gifts[x]);
                } else {
                    song.push_str(numbers[x]);
                    song.push_str(gifts[x]);
                }
            }
        }

        println!("{}", song);
        song = String::new();

        _index += 1;

        if _index == 12 {
            done = true;
        }
    }
}
