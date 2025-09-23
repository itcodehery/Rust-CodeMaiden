struct BankAccount {
    name: String,
    password: String,
}

impl BankAccount {
    fn new(password: String, name: String) -> Self {
        Self { password, name }
    }

    fn display_password(&self) {
        println!("{}", self.password);
    }

    fn display_name(&self) {
        println!("{}", self.name);
    }
}

fn main() {
    let acc1: BankAccount = BankAccount {
        password: "asidjoasjd".to_string(),
        name: String::from("Arden"),
    };

    println!("{}", acc1.password);

    acc1.display_password();
    acc1.display_name();
}
