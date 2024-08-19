mod content;
use content::profession::Profession;

fn main() {
    // lenghty use reaching all the way down into the nested module:
    let person = content::person::Person::new("Said".to_string(), 35);
    println!("{:#?}", person);

    // shorthand use after the more specific import:
    let profession = Profession::new("SDE".to_string());
    println!("{:#?}", profession);    
}
