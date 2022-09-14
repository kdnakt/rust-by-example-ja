trait UsernameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get the selected age out of this widget
    fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // If you uncomment this line, you'll get an error saying
    // "multiple `get` found". Because, after all, there are multiple methods
    // named `get`.
    // println!("{}", form.get());

    // Disambiguating by a turbofish.
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);

    // Disambiguating by rustc recommendation.
    let username = UsernameWidget::get(&form);
    assert_eq!("rustacean".to_owned(), username);

    // Disambiguating by casting the form.
    let username_widget_form: &dyn UsernameWidget = &form;
    let username: String = username_widget_form.get();
    assert_eq!("rustacean".to_owned(), username);

    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}

