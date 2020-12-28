#[cfg(test)]
mod cards_test {
    use veho::hashmap::Mappers;

    use palett::cards::{AMBER, LIME};
    use palett::dye::DyeFactory;
    use palett::enums::Effect;

    #[test]
    fn test() {
        let dye_factory = DyeFactory::hex(&[Effect::Bold, Effect::Underline]);
        AMBER.to_hashmap().iterate(|k, v| {
            println!("{}: {}", k, dye_factory.render(v, v));
        });
        LIME.to_hashmap().iterate(|k, v| {
            println!("{}: {}", k, dye_factory.render(v, v));
        });
    }
}