fn main() {

    let mut s1 = String::from("hello_world");

    let mut s2 = change(&mut s1);

    println!("---------the value of s1={s1}----------");

    //println!("---------the value of s1={s2}----------");

}

fn change(input_data:&mut String){
    input_data.push_str("_this_is_devil")
}
