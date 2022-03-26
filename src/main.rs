use std::{fs, io::Write};

fn create_root_and_manifest(project_name: &str) -> std::io::Result<()> {
    fs::create_dir(&project_name)?;
    let mut cargo_toml = fs::File::create(format!("{project_name}/Cargo.toml"))?;
    let cargo_toml_content = include_str!("../templates/Cargo.toml").replace("dayXX", project_name);
    cargo_toml.write_all(cargo_toml_content.as_bytes())?;

    Ok(())
}

fn create_empty_input_files(project_name: &str) -> std::io::Result<()> {
    fs::create_dir(format!("{project_name}/input"))?;
    fs::File::create(format!("{project_name}/input/input_.txt"))?;
    fs::File::create(format!("{project_name}/input/sample_.txt"))?;

    Ok(())
}

fn create_source_files(project_name: &str) -> std::io::Result<()> {
    fs::create_dir(format!("{project_name}/src"))?;
    let mut main = fs::File::create(format!("{project_name}/src/main.rs"))?;
    let main_content = include_str!("../templates/src/main.rs").replace("dayXX", project_name);
    main.write_all(main_content.as_bytes())?;
    let mut lib = fs::File::create(format!("{project_name}/src/lib.rs"))?;
    lib.write_all(include_bytes!("../templates/src/lib.rs"))?;

    Ok(())
}

fn create_test_files(project_name: &str) -> std::io::Result<()> {
    fs::create_dir(format!("{project_name}/tests"))?;
    let mut integration_test =
        fs::File::create(format!("{project_name}/tests/integration_test.rs"))?;
    let integration_test_content =
        include_str!("../templates/tests/integration_test.rs").replace("dayXX", project_name);
    integration_test.write_all(integration_test_content.as_bytes())?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let project_name = std::env::args().nth(1).unwrap();

    create_root_and_manifest(&project_name)?;

    create_empty_input_files(&project_name)?;

    create_source_files(&project_name)?;

    create_test_files(&project_name)?;

    Ok(())
}
