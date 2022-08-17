use std::io::Split;

/*
Runs an file of code
 */
pub fn run_code(code: &str) {
    let mut environment = "r";

    let mut iter = code.chars();

    let mut count = 0;
    let mut parameters: Vec<String> = Vec::new();
    let mut op_code = 0;
    while let Some(ch) = iter.next() {
        /*
        r = Runtime
        p = Parameters
        s = String builder
        d = Default
         */

        let dec = ch as i32;
        //Get the op code that is wanted to be run
        if environment == "r" {
            op_code = dec;
            environment = "p";

        } else if environment == "s" {
            //Make the string

            let str: String = code.to_string();
            let out = &str[count+2 as usize..count + 2 + dec as usize];
            parameters.push(out.parse().unwrap());

            //Keep track, for string building
            count = count as usize + dec as usize;

            environment = "d";

            //Skip in the loop
            iter.nth((dec - 1) as usize);
        } else if environment == "p" {
            if dec == 2 {
                environment = "s";
            }
        } else if environment == "d" {
            if dec == 31 {
                //Run
                run_block(op_code, &parameters)
            }
        }
        count = count + 1;
    }
}


/*
Runs an opcode
 */
pub fn run_block(code: i32, params: &Vec<String>) {
    match code {
        128 => println!("{}", params.join("")),

        _ => { println!("Code escaped unexpectedly >> {}", code)}
    }
}