use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    
    let mut collect = HashMap::new();
    collect.insert("n150", vec!["c", "rust", "java", "csharp", "lisp", "go", "fsharp", "fortran", "javascript", "racket", "typescript", "erlang", "php", "python", "perl", "lua", "jruby"]);
    collect.insert("i78700", vec!["c", "rust", "java", "lisp", "csharp", "go", "fsharp", "fortran", "javascript", "racket", "typescript", "erlang", "php", "lua", "python", "perl", "jruby"]);
    collect.insert("epyc #1", vec!["rust", "c", "java", "csharp", "fsharp", "lisp", "go", "fortran", "racket", "javascript", "erlang", "typescript", "php", "python", "jruby", "perl", "lua"]);
    collect.insert("epyc #2", vec!["rust", "c", "java", "csharp", "fsharp", "lisp", "go", "fortran", "racket", "javascript", "erlang", "php", "typescript", "python", "jruby", "perl", "lua"]);
    collect.insert("m2pro", vec!["c", "java", "fortran", "fsharp", "lisp", "rust", "csharp", "go", "javascript", "racket", "typescript", "jruby", "erlang", "php", "lua", "python", "perl"]);

    let keys: Vec<&str>= collect.keys().cloned().collect();

    for n in 0..keys.len() {
        for m in n+1..keys.len() {
            let a = keys[n];
            let b = keys[m];
            let tau = kendall(collect[a].clone(), collect[b].clone());
            println!("{:7} - {:7} | Tau: {:.3}", a,b,tau);
        }
    }

}


fn kendall<T: PartialEq>(a: Vec<T>, b: Vec<T> ) -> f32 {
    let mut ix = vec![];
    for elem in a {
        let i = b.iter().position(|x| *x == elem).unwrap();
        ix.push(i);
    }


    let mut con:f32 = 0.0;
    let mut dis:f32 = 0.0;
    for n in 0..=ix.len()-1 {
        let v = ix[n];
        for m in n+1..=ix.len()-1 {

            match v.cmp(&ix[m]){ 
                Ordering::Less =>  con +=1.0,
                Ordering::Greater => dis +=1.0,
                _ =>  break,
            };

        }
    }

    //println!("{} - {}", con, dis);

    let n = ix.len() as f32;
    (con-dis) / ((n*(n-1.0))/2.0)
}
