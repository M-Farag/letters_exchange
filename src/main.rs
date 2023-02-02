use std::io;
fn main() {
    println!("Define the letter you wanna replace, x -> y");
    let mut user_replace_input:String = String::new();
    let mut user_input_text:String = String::new();

    println!("Define the two letters you wanna replace ?! x -with-> y");
    io::stdin().read_line(&mut user_replace_input).expect("Err Reading input");
    let letters_to_replace = chars_to_replace(user_replace_input);
    println!("Change letter '{}' with a '{}' ",letters_to_replace.0,letters_to_replace.1);

    println!("Add the text you wanna change");
    io::stdin().read_line(&mut user_input_text).expect("Err reading your text");

    let user_input_text = replace_text(letters_to_replace, user_input_text);
    println!("Modified text: {}",user_input_text);


}

fn chars_to_replace(user_replace_input:String) -> (char,char)
{
    (  user_replace_input.chars().take(1).last().unwrap(),
       user_replace_input.chars().take(3).last().unwrap() 
    )
    
}

fn replace_text(letter_to_replace:(char, char), user_text:String) ->String
{
    let mut modified_text:String = String::with_capacity(user_text.len());
    let letters = user_text.chars();

    for mut letter in letters {
        if letter == letter_to_replace.0  {
            letter = letter_to_replace.1;
        }
        modified_text.push(letter);
    }
    modified_text

}