use clap::{Command, Arg};
use regex::Regex;
use reqwest::blocking::get;
use std::{fs, process};
use chrono::Datelike;

// バリデーション関数（半角英数字チェック）
fn validate_alphanumeric(input: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    re.is_match(input)
}

// PyPIでライブラリ名が使えるか確認する
fn is_name_available_on_pypi(name: &str) -> bool {
    let url = format!("https://pypi.org/pypi/{}/json", name);
    match get(&url) {
        Ok(response) => response.status() == 404,
        Err(_) => false,
    }
}

// テンプレートファイルを読み込む関数
fn read_template(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| {
        eprintln!("Error: Could not read the template file '{}'.", file_path);
        process::exit(1);
    })
}

// `templates/licenses`からライセンスファイルを取得し、候補を作成する関数
fn get_license_options() -> Vec<String> {
    let license_dir = "templates/licenses";
    let mut licenses = Vec::new();

    // ディレクトリ内のファイルを取得
    for entry in fs::read_dir(license_dir).unwrap_or_else(|_| {
        eprintln!("Error: Could not read licenses directory '{}'.", license_dir);
        process::exit(1);
    }) {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();

        // ドットで始まる隠しファイルを除外
        if !file_name.starts_with('.') {
            licenses.push(file_name);
        }
    }

    // カスタムオプションを追加
    licenses.push("custom".to_string());
    licenses
}

// 入力とバリデーションを一括で行う関数（必須入力を確認）
fn prompt_required_input(message: &str, default_value: Option<String>) -> String {
    if let Some(value) = default_value {
        return value;
    }
    let input = dialoguer::Input::<String>::new().with_prompt(message).interact_text().unwrap_or_else(|_| {
        eprintln!("Input was cancelled. Exiting.");
        process::exit(1);
    });
    if input.trim().is_empty() {
        eprintln!("Error: This field is required.");
        process::exit(1);
    }
    input.trim().to_string()
}

// オプション入力（空白許可）
fn prompt_optional_input(message: &str, default_value: Option<String>) -> String {
    if let Some(value) = default_value {
        return value;
    }
    // 空入力を許可するために、trimでトリミングした後にそのまま返す
    dialoguer::Input::<String>::new()
        .with_prompt(message)
        .allow_empty(true)  // 空入力を許可
        .interact_text()
        .unwrap_or_else(|_| {
            eprintln!("Input was cancelled. Exiting.");
            process::exit(1);
        })
        .trim()
        .to_string()
}

// ライセンス選択
fn select_license(default_value: Option<String>) -> String {
    if let Some(license) = default_value {
        let valid_licenses = get_license_options();
        if valid_licenses.contains(&license) {
            return license;
        } else {
            eprintln!("Error: Invalid license '{}'. Available options are: {:?}", license, valid_licenses);
            process::exit(1);
        }
    }

    let licenses = get_license_options(); // ファイルから取得したライセンスリストを使用
    let selection = dialoguer::Select::new()
        .with_prompt("Choose a license")
        .items(&licenses)
        .default(0)
        .interact()
        .unwrap_or_else(|_| {
            eprintln!("License selection was cancelled. Exiting.");
            process::exit(1);
        });
    licenses[selection].to_string()
}

// 現在の西暦を取得
fn get_current_year() -> i32 {
    let now = chrono::Utc::now();
    now.year()
}

// ファイルとディレクトリ作成
fn create_project_files(name: &str, desc: &str, author: &str, license: &str, custom_license: Option<&str>) {
    // ディレクトリ構造を作成
    let project_dir = format!("./{}", name);
    let module_dir = format!("{}/{}", project_dir, name);
    let tests_dir = format!("{}/tests", project_dir);
    fs::create_dir_all(&module_dir).expect("Failed to create directories.");
    fs::create_dir_all(&tests_dir).expect("Failed to create directories.");

    // テンプレートファイルのパス
    let template_path = "templates/";

    // ファイルの作成
    let license_file = format!("{}/LICENSE", project_dir);
    let readme_file = format!("{}/README.md", project_dir);
    let init_file = format!("{}/__init__.py", module_dir);
    let requirements_file = format!("{}/requirements.txt", project_dir);
    let setup_file = format!("{}/setup.py", project_dir);

    // ライセンステンプレートの読み込みと置換
    let current_year = get_current_year().to_string();
    let license_content = if license == "custom" {
        custom_license.unwrap().to_string()
    } else {
        let license_template = format!("{}licenses/{}", template_path, license);
        let mut license_text = read_template(&license_template);
        license_text = license_text.replace("{year}", &current_year);
        license_text = license_text.replace("{author}", author);
        license_text
    };
    fs::write(&license_file, license_content).expect("Failed to create LICENSE file.");

    // READMEテンプレートの読み込みと置換
    let readme_template = read_template(&format!("{}README.md", template_path));
    let readme_content = readme_template.replace("{name}", name).replace("{desc}", desc);
    fs::write(&readme_file, readme_content).expect("Failed to create README.md file.");

    // __init__.pyテンプレートの読み込みと置換
    let init_template = read_template(&format!("{}__init__.py", template_path));
    let init_content = init_template
        .replace("{name}", name)
        .replace("{desc}", desc)
        .replace("{author}", author)
        .replace("{license}", license)
        .replace("{year}", &current_year);
    fs::write(&init_file, init_content).expect("Failed to create __init__.py file.");

    // requirements.txtの作成
    let requirements_template = read_template(&format!("{}requirements.txt", template_path));
    fs::write(&requirements_file, requirements_template).expect("Failed to create requirements.txt file.");

    // setup.pyのテンプレートを読み込み、変数置換
    let setup_template = read_template(&format!("{}setup.py", template_path));
    let setup_content = setup_template
        .replace("{name}", name)
        .replace("{desc}", desc)
        .replace("{author}", author)
        .replace("{license}", license);
    fs::write(&setup_file, setup_content).expect("Failed to create setup.py file.");

    println!("Project files created successfully.");
}

fn main() {
    let matches = Command::new("pyinit")
    .version(env!("CARGO_PKG_VERSION"))
        .about("CLI tool to create Python library scaffolding")
        .arg(Arg::new("name").short('n').long("name").required(false).help("Library name"))
        .arg(Arg::new("description").short('d').long("description").required(false).help("Library description"))
        .arg(Arg::new("author").short('a').long("author").required(false).help("Author's name"))
        .arg(Arg::new("license").short('l').long("license").required(false).help("License type"))
        .get_matches();

    // 1. ライブラリ名を入力させる
    let library_name = prompt_required_input(
        "Enter the library name", 
        matches.get_one::<String>("name").cloned()
    );

    // 2. ライブラリ名のバリデーション
    if !validate_alphanumeric(&library_name) {
        eprintln!("Error: Library name must consist of alphanumeric characters only.");
        process::exit(1);
    }

    // 3. PyPIでライブラリ名が使えるか確認
    if !is_name_available_on_pypi(&library_name) {
        eprintln!("Error: The library name '{}' is already taken on PyPI.", library_name);
        process::exit(1);
    }

    // 4. ライブラリの説明を入力（空白も許可）
    let description = prompt_optional_input(
        "Enter a description for the library (optional)", 
        matches.get_one::<String>("description").cloned()
    );

    // 5. 作者名を入力
    let author = prompt_required_input(
        "Enter the author's name", 
        matches.get_one::<String>("author").cloned()
    );
    if !validate_alphanumeric(&author) {
        eprintln!("Error: Author name must consist of alphanumeric characters only.");
        process::exit(1);
    }

    // 6. ライセンスの選択
    let license = select_license(
        matches.get_one::<String>("license").cloned()
    );
    let custom_license = if license == "custom" {
        Some(prompt_optional_input("Enter your custom license (optional)", None))
    } else {
        None
    };

    // 7. ファイルとディレクトリの作成
    create_project_files(&library_name, &description, &author, &license, custom_license.as_deref());
}
