pub fn verse(n: i32) -> String {
//    unimplemented!("emit verse {}", n)

    let zero: String = String::from("No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
");
    let one: String = String::from("1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.
");
    let two = format!("{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottle of beer on the wall.
", 2,2,1);
    let multiple = format!("{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.
", n,n,n-1);

    match n {
        0 => zero,
        1 => one,
        2 => two,
        _ => multiple
    }
}

pub fn sing(start: i32, end: i32) -> String {
//    unimplemented!("sing verses {} to {}, inclusive", start, end)
    let mut song: String = String::new();

    for x in (end..start+1).rev() {
        song.push_str(&format!("{}", &verse(x)));
        if x != end {
            song.push_str(&format!("\n"));
        }
    }
    song
}
