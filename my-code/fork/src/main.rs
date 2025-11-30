use libc::fork;

fn main() {
    unsafe {
        let mut x = 100 ;


        let pid = fork();

        if pid < 0 {
            eprintln!("Fork failed!");
        }else if pid == 0 {
            println!("from child x is {}!", x);
        }else{
            x = 101;
            println!("from parent x is {}!", x);
        }
    }
}

