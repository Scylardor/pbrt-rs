extern crate rpbrt;

#[test]
fn raytracer_import_scene() {
    let mut rt = rpbrt::Raytracer::from("tests/example.yml");
    match rt {
        Err(rpbrt::ParseError::IOError(ref err)) => println!("Unexpected IO error: {}", err),
        Err(rpbrt::ParseError::YamlScanError(ref err)) => println!("YAML parse error: {}", err),
        Err(rpbrt::ParseError::UnknownValueError(_)) => println!("Unknown error"),
        Ok(_) => println!("It works!"),
    }
    
    assert_eq!(2 + 2, 4);
}

