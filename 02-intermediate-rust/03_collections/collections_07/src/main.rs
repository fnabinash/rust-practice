// Create a program that stores user preferences in a HashMap and allows updating and retrieving preferences.

use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Preferences {
    A,
    B,
    C
}

fn get_preferences(pre_list: &HashMap<String, Preferences>, user: &str) -> Preferences {
    pre_list.get(user).unwrap().clone()
}

fn update_preferences(pre_list: &mut HashMap<String, Preferences>, user: &str, new_pre: Preferences) {
    let old_pre: &mut Preferences = pre_list.get_mut(user).unwrap();
    *old_pre = new_pre;
}

fn main() {
    let mut pre_list: HashMap<String, Preferences> = HashMap::new();
    let user: String = "Alice".to_string();

    pre_list.insert(user.clone(), Preferences::A);
    println!("{:?}", pre_list);

    let pre: Preferences = get_preferences(&pre_list, &user);
    println!("Preference: {:?}", pre);
    
    update_preferences(&mut pre_list, &user, Preferences::B);
    let pre: Preferences = get_preferences(&pre_list, &user);
    println!("Preference: {:?}", pre);

    update_preferences(&mut pre_list, &user, Preferences::C);
    let pre: Preferences = get_preferences(&pre_list, &user);
    println!("Preference: {:?}", pre);

}
