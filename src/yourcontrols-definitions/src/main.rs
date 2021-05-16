use yourcontrols_definitions::DefinitionsParser;
use yourcontrols_types::Error;

fn main() -> Result<(), Error> {
    let mut parser = DefinitionsParser::new();

    parser.load_scripts("definitions/scripts");
    parser.load_file("definitions/templates/ToggleSwitch.yaml");

    println!(
        "{:#?} {:#?}",
        parser.load_file("definitions/aircraft/Asobo_C172.yaml"),
        parser
    );
    // parser.load_definition_file("definitions/aircraft/Asobo_330LT.yaml");
    // println!("{:?}", parser.get_datums());

    Ok(())
}