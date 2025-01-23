    #![deny(unsafe_code)]
    #![no_main]
    #![no_std]
    
    use cortex_m_rt::entry;
    use rtt_target::{rtt_init_print, rprintln};
    use panic_rtt_target as _;
    use microbit::{
        board::Board,
        display::blocking::Display,
        hal::{prelude::*, Timer},
    };
    
    const NODES: [[usize; 2]; 16] = [
        [0,0], [0,1], [0,2], [0,3], [0,4],
        [1,4], [2,4], [3,4], [4,4],
        [4,3], [4,2], [4,1], [4,0],
        [3,0], [2,0], [1,0],
    ];
    
    #[entry]
    fn main() -> ! {    
        rtt_init_print!();
        
        let board = Board::take().unwrap();
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);
        let mut light_it_all = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        loop {
            for node in NODES {
                let i = node[0];
                let j = node[1];
                light_it_all[i][j] = 1;
                display.show(&mut timer, light_it_all, 30);
                light_it_all[i][j] = 0;
            }
        }
    }