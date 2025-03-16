use std::{cmp::Ordering};

fn main() {


    //let n150 = vec![];
    //let i78700 = vec![];
    let epyc1 = vec!["rust", "c", "java", "csharp", "fsharp", "lisp", "go", "fortran", "racket", "javascript", "erlang", "typescript", "php", "python", "jruby", "perl", "lua"];
    let epyc2 = vec!["rust", "c", "java", "csharp", "fsharp", "lisp", "go", "fortran", "racket", "javascript", "erlang", "php", "typescript", "python", "jruby", "perl", "lua"];
    //let m2pro = vec![];

    let list_a = epyc1;
    let list_b = epyc2;

    // [3,0,1,2]
    let mut ix = vec![];
    for elem in list_a {
        let i = list_b.iter().position(|&x| x == elem).unwrap();
        ix.push(i);
    }


    let mut con:f32 = 0.0;
    let mut dis:f32 = 0.0;
    for n in 0..=ix.len()-2 {
        let v = ix[n];
        for m in n+1..=ix.len()-2 {

            match v.cmp(&ix[m]){ 
                Ordering::Less =>  con +=1.0,
                Ordering::Greater => dis +=1.0,
                _ =>  break,
            };

        }
    }

    let n = ix.len() as f32;
    let tau = (con-dis) / ((n*(n-1.0))/2.0);

    let z = (3.0*tau * (n*(n-1.0)).sqrt())/(2.0*(2.0*n + 5.0)).sqrt();

    println!("{}, {}", tau, z);

    println!("{} - {}", con, dis);

    println!("Hello, world! {:?}", ix);
}



