fn main() {
    let sample_string = "this1isa5\ntestof9purpose";
    // Spit string by end line character
    let input_info = sampleString.split("\n").collect::<Vec<&str>>();
    println!("splitString: {:?}", input_info);

    for element in input_info 
    {
        println!("{}", element);
    }
}
