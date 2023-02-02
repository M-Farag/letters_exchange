use std::io;
fn main() {
    println!("Define the letter you wanna replace, x -> y");
    let mut user_replace_input:String = String::new();
    let mut user_input_text:String = String::new();

    println!("Define the two letters you wanna replace ?! x -with-> y");
    io::stdin().read_line(&mut user_replace_input).expect("Err Reading input");
    let mut letters_to_replace:(char,char) = (' ',' ');
    let letters_to_replace = chars_to_replace(user_replace_input,letters_to_replace);
    println!("Letter to change are {} & {}",letters_to_replace.0,letters_to_replace.1);

    println!("Add the text you wanna change");
    io::stdin().read_line(&mut user_input_text).expect("Err reading your text");

    let user_input_text = replace_text(letters_to_replace, user_input_text);
    println!("Modified text: {}",user_input_text);


}

fn chars_to_replace(user_replace_input:String,mut letters_container:(char, char)) -> (char,char)
{
    
    letters_container.0 = user_replace_input.chars().take(1).last().unwrap();
    letters_container.1 = user_replace_input.chars().take(3).last().unwrap();
    letters_container
    
}

fn replace_text(letter_to_replace:(char, char), user_text:String) ->String
{
    let mut modified_text:String = String::new();
    let letters = user_text.chars();

    for letter in letters {
        if letter == letter_to_replace.0  {
            modified_text.push(letter_to_replace.1);
        }
        else {
            modified_text.push(letter);
        }
        
    }
    modified_text

}