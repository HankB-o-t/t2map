extern "C" {
//    fn kbhit() -> bool;
     fn getchar() -> u8;
}

fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}

// windows only
fn main() {
    unsafe {

    let mut c;
    let mut x = 1;
    let mut y = 1;
    let w = 30; 
    let h = 10;
    let mut arr =vec![vec!["_";w];h];

    let obs = "█";
    let key = "¬";
    let mut collected_key = false;

        loop {
            if /*kbhit()*/ true {
                c = getchar() as char;
                match c {
                    's' => { if y >= h-1 
                            || arr[y+1][x] == obs 
                            { continue; }
                            y+=1;
                    },
                    'w' => { if y == 0 
                            || arr[y-1][x] == obs 
                            { continue; }
                            y-=1;
                    },
                    'a' => { if x == 0 
                            || arr[y][x-1] == obs 
                            { continue; }
                            x-=1;
                    },
                    'd' => { if x >= w-1 
                            || arr[y][x+1] == obs 
                            { continue; }
                            x+=1;
                    },
                    'q' => { break; },
                    _ => {  }
                }
            }
            
            
            for i in 0..h {
                for j in 0..w {
                    arr[i][j] =" ";

                    // ---------- MAIN WALLS -------------
                    for up in 0..w { arr[0][up] = obs; } // wall up
                    for left in 0..h { arr[left][0] = obs; } // wall left
                    for down in 0..w { arr[h-1][down] = obs; } // wall down
                    for right in 0..h { arr[right][w-1] = obs; } // wall left
                    //------------------------------------

                    // ROOM KEY
                    for a in 0..3 { arr[a][2] = obs; } // wall
                    for a in 2..8 { arr[3][a] = obs; } // wall
                    for a in 2..9 { arr[1][a] = obs; } // wall
                    for a in 0..8 { arr[a][9] = obs; } // wall

                    // Walls
                    for a in 1..8 { arr[5][a] = obs; } // wall
                    for a in 2..9 { arr[7][a] = obs; } // wall
                    for a in 2..h-1 { arr[a][11] = obs; } // wall
                    for a in 0..h-2 { arr[a][13] = obs; } // wall
                    for a in 2..h-1 { arr[a][15] = obs; } // wall
                    for a in 17..w-1 { arr[2][a] = obs; } // wall
                    for a in 16..w-2 { arr[4][a] = obs; } // wall
                    for a in 17..w-1 { arr[6][a] = obs; } // wall
                    for a in 16..w-1 { arr[8][a] = obs; } // wall

                    // KEY
                    if collected_key == false {arr[2][3] = key;}
                    else { arr[2][3] = "_"; }
                    if arr[y][x] == arr[2][3] { collected_key = true; }
                    if collected_key == true {
                        arr[7][w-1] = "▒";
                    } else {
                        arr[7][w-1] = "█";
                    }
                    if arr[y][x] == arr[1][28] {
                        arr[1][27] = obs;
                        println!("SECRET ENDING!!!!1!!");
                        println!("Press Ctrl+C or Q to quit.");
                        cls();
                    }
                    if arr[y][x] == arr[7][w-1] && collected_key == true {
                        arr[7][w-2] = obs;
                        println!("Congrats. You have beaten da game");
                        println!("Press Ctrl+C or Q to quit.");
                        cls();
                    }
                    
                    
                    arr[1][28] = "▓";
                    arr[y][x] = "&";
                    print!("{}",arr[i][j]);
                }
                println!("");
            }
            
            std::thread::sleep(std::time::Duration::from_millis(50));
            cls(); // clr
        }

    }
}
