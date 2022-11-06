use tiny_id::ShortCodeGenerator;

pub type Id = String;

pub fn gen_id() -> Id {
    ShortCodeGenerator::new_lowercase_alphanumeric(4).next_string()
}
