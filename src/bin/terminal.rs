use game_life::{config::*, game::Context};
use std::{env, io::stdin, process::exit, time::Instant};

fn main() {
    let filename = env::args().nth(1);
    if filename.is_none() {
        return eprintln!("Error: missing arg filename");
    }

    let config: Config = match Config::from_file(filename.unwrap()) {
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
        Ok(config) => config,
    };

    let mut context = Context::new(config);

    for n in -100..100 {
        for m in -100..100 {
            context.map.set(n, m, rand::random::<CellID>() % 2);
            // context.map.set(n, m, 1);
        }
    }

    let _custom_struct: Vec<(i16, i16)> = vec![
        // // Left Square (Block)
        // (1, 5), (1, 6), (2, 5), (2, 6),

        // // Left Engine (The main shooting structure)
        // (11, 5), (11, 6), (11, 7),
        // (12, 4), (12, 8),
        // (13, 3), (13, 9),
        // (14, 3), (14, 9),
        // (15, 6),
        // (16, 4), (16, 8),
        // (17, 5), (17, 6), (17, 7),
        // (18, 6),

        // // Right Engine
        // (21, 3), (21, 4), (21, 5),
        // (22, 3), (22, 4), (22, 5),
        // (23, 2), (23, 6),
        // (25, 1), (25, 2), (25, 6), (25, 7),

        // // Right Square (Block)
        // (35, 3), (35, 4), (36, 3), (36, 4),

        // // Acorn
        // (1, 0),
        // (3, 1),
        // (0, 2), (1, 2), (4, 2), (5, 2), (6, 2),

        // // R-pentomino
        (1, 0),
        (2, 0),
        (0, 1),
        (1, 1),
        (1, 2),
    ];
    for (x, y) in _custom_struct {
        context.map.set(x, y, 1);
    }

    // context.map.set(0,-1, 1);
    // context.map.set(0,0, 1);
    // context.map.set(0,1, 1);

    // context.map.display_term(0, 0, 10);
    // print!("\x1b[H"); // Move cursor to top left corner of terminal
    loop {
        print!("\x1b[s");
        context.map.display_term(50, 0, 200, 100);
        // sleep(time::Duration::from_millis(20));
        let now = Instant::now();
        context.update();
        println!("ms elapsed -- {:}", now.elapsed().as_millis());
        let mut _s = String::new();
        _ = stdin().read_line(&mut _s);
        print!("\x1b[u");
    }
    // println!("===================");

    // context.update();
    // context.map.display_term(0, 0, 10);
    // println!("===================");
    // context.update();
    // context.map.display_term(0, 0, 10);
    // println!("===================");
    // context.update();
    // context.map.display_term(0, 0, 10);
    // println!("===================");
    // context.update();
    // context.map.display_term(0, 0, 10);
}
