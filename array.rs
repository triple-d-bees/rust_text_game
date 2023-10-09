struct Hero {
    hero_name: String,
    icon: char,
    health: u32,
    x: i32,
    y: i32,
}

struct Map {
    state: Vec<Vec<String>>,
    size: usize,
}

fn main(){
    let width = 100;
    let height = 100;
    // let mut array = vec![vec![0; width]; height];
    let mut map = Map {
        state: vec![vec!["0".to_string(); width]; height],
        size: 100
    };
    // let mut user_name = String::new();
    // let mut size = String::new();
    // println!("Enter your name :");
    // std::io::stdin()
    //     .read_line(&mut user_name)
    //     .unwrap();
    // println!("Hello, {}", hero.hero_name);

    let mut hero = Hero {
            hero_name: "Bob".to_string(),
            icon: '@',
            health: 100,
            x: 6,
            y: 2
        };
    map.state[3][4] = "&".to_string();
    map.state[1][10] = "1".to_string();
    map.state[9][10] = "9".to_string();
    init_map(&mut map, width, height);
    print_map(&map, &hero);
    println!("Please type a direction - N, E, S, W,");
    println!(" or x to escape:");
    let mut input_string = String::new();

    while input_string.trim() != "x" && input_string.trim() != "q"{
        input_string.clear();
        std::io::stdin().read_line(&mut input_string).unwrap();
        // ip.clear();
        let ip = input_string.trim().to_ascii_uppercase().to_string();
        remove_old(hero.x,hero.y, &mut map);
        move_character(&mut hero, &ip);
        update_place(&hero, &mut map);
        print_map(&map, &hero);
        // println!("You wrote {}", &ip);


    }
    println!("See you later!");
    
}
fn move_character(h: &mut Hero, s: &String){
    println!("Your hero was at {}, {}", h.x.to_string(),h.y.to_string());
    if s == "N"{
        move_character_north(h);
        println!("Moving {}!",s);
    }
    else if s == "S"{
        move_character_south(h);
        println!("Moving {}!",s);
    }
    else if s == "E"{
        move_character_east(h);
        println!("Moving {}!",s);
    }
    else if s == "W"{
        move_character_west(h);
        println!("Moving {}!",s);
    }
    else {
        println!("Where are you trying to go?\nDirections are N, E, S, or W");
        println!("x or q to quit");
    }

    
}
fn move_character_north(h: &mut Hero){
    if h.y > 2{
        h.y -=1;
    }
}
fn move_character_south(h: &mut Hero){
    if h.y < 999{
        h.y +=1;
    }
}
fn move_character_east(h: &mut Hero){
    if h.x < 999{
        h.x +=1;
    }
}
fn move_character_west(h: &mut Hero){
    if h.x >2{
        h.x -=1;
    }
}



fn init_map(m: &mut Map, width: usize, height: usize){
    for n in 0..width{
        for o in 0..height{
            if o == 0 {
                m.state[n][o] = "|".to_string();
            }
            else if o == 1000{
                m.state[n][o] = "|".to_string();
            }
            else if n == 0 {
                m.state[n][o] = "-".to_string();
            }
            else if n == 1000 {
                m.state[n][o] = "-".to_string();
            }
            else if o == 1  {
                m.state[n][o] = n.to_string();
            }
            else {
                m.state[n][o] = ".".to_string();
            }     
        }
    }
}

fn remove_old(x: i32, y: i32, m: &mut Map){
    m.state[y as usize][x as usize] = ",".to_string();
}
fn update_place(h :&Hero, m: &mut Map){
    m.state[h.y as usize][h.x as usize] = h.icon.to_string();
}

fn print_map(m: &Map, h :&Hero){
    let mut base_x = 0;
    let mut base_y = 0;
    let mut max_x = 50;
    let mut max_y = 10;
    if h.y > 10 {
        base_y = (h.y - 5) as usize;
        max_y = (h.y + 5) as usize;
    }
    else if h.y > 989 {
        base_y = 990 as usize;
        max_y = 1000 as usize;
    }
    if h.x > 30 {
        base_x = (h.x - 24) as usize;
        max_x = (h.x + 24) as usize;
    }
    else if h.x > 989 {
        base_x = 990 as usize;
        max_x = 1000 as usize;
    }
    for n in base_y..=max_y{
        for o in base_x..=max_x{
            print!("{}", m.state[n][o]);
        }
        println!("> Hero is at {}, {}", h.x.to_string(),h.y.to_string());
    }

}