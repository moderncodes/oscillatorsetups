use std:: {
    error::Error,
    fs::{create_dir_all, File},
    io::{Error as io_error, Read, Write, },
    path::{Path, PathBuf},
    fmt,
};

use serde::de;

use reqwest::Url;

/// Constructs the path for storing JSON data.
///
/// This function parses the given URL, and if the URL can be parsed successfully, the domain is extracted.
/// The function then returns a folder path that includes the domain and the "klines" directory.
/// If the URL can't be parsed, or if the domain can't be extracted, the function returns "coinbase" as a default value.
///
/// # Arguments
/// * `base_url`        - URL host name
/// * `endpoint`        - The specific API path name
///
/// # Returns
/// A `String` that represents the path for storing JSON data.
/// ```markdown
/// ├── files
/// │   ├── `endpoint`
/// │   │   ├── `base_url`
/// │
/// ├── scr
/// ```
#[allow(dead_code)]
pub fn get_folder_path(base_url: &str, endpoint: &str) -> String {
    let parsed_url = Url::parse(base_url).unwrap_or_else(|_| panic!("Failed to parse URL: {}", base_url));
    parsed_url.domain().map(|domain| format!("{}/{}/", endpoint, domain)).unwrap_or("/temp".to_string())
}

/// Constructs the full path for a given file in a given folder.
/// The file is assumed to be in a directory "./files",
/// and the filename will be converted to lowercase and given a ".json" extension.
/// If the directories in the path don't exist, they will be created.
///
/// ## Arguments
/// * `folder_name` - A string representing the name of the folder.
/// * `file_name` - A string representing the name of the file.
///
/// ## Examples
/// ```
/// use oscillatorsetups::utils::build_path;
/// use std::path::PathBuf;
///
/// let path = build_path("folder_name", "testfile");
/// let expected_path = PathBuf::from("./files/folder_name/testfile.json");
/// assert_eq!(path, expected_path);
/// ```
pub fn build_path(folder_name: &str, file_name: &str) -> PathBuf {
    let file_name = file_name.to_lowercase();
    let file_name = format!("{}.json", file_name);
    let path = Path::new("./files").join(folder_name).join(file_name);

    if let Some(parent_path) = path.parent() {
        create_dir_all(parent_path).expect("Unable to create directory!");
    }

    path
}

/// Writes a string to a JSON file.
///
/// ## Arguments
/// * `folder_name` - A string representing the name of the folder.
/// * `file_name` - A string representing the name of the file.
/// * `data` - A string representing the data to write to the file.
///
/// # Examples
/// ```no_run
/// use oscillatorsetups::utils::data_to_json;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     data_to_json("folder_name", "testfile", "data")?;
///     Ok(())
/// }
///
/// ```
pub fn data_to_json(
    folder_name: &str,
    file_name: &str,
    data: &str,
) -> Result<(), Box<dyn Error>> {
    let path = build_path(folder_name, file_name);

    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}

/// Reads data from a JSON file.
///
/// # Arguments
/// * `folder_name` - A string representing the name of the folder.
/// * `file_name` - A string representing the name of the file.
///
/// # Examples
/// ```no_run
/// use oscillatorsetups::utils::data_from_json;
/// use oscillatorsetups::utils::data_to_json;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let folder_name = "folder_name";
///     let file_name = "testfile";
///     let data = r#"{"key": "value"}"#;
///
///     data_to_json(folder_name, file_name, data)?;
///     let read_data = data_from_json(folder_name, file_name)?;
///
///     assert_eq!(data, read_data);
///     Ok(())
/// }
/// ```
pub fn data_from_json(folder_name: &str, file_name: &str) -> Result<String, io_error> {
    let path = build_path(folder_name, file_name);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// see https://docs.rs/serde/latest/serde/de/trait.Visitor.html
struct F64Visitor;
impl<'de> de::Visitor<'de> for F64Visitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of f64")
    }

    fn visit_str<E>(self, value: &str) -> Result<f64, E>
        where
            E: de::Error,
    {
        value.parse::<f64>().map_err(E::custom)
    }
}
/// Deserializes a string into an `f64`.
///
/// # Arguments
/// * `deserializer` - The deserializer instance used for the conversion.
///
/// # Returns
/// A parsed `f64` value or an error.
pub fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: de::Deserializer<'de>,
{
    deserializer.deserialize_string(F64Visitor)
}



/// Custom error type for utility-related operations.
///
/// This type can be used for returning custom error messages.
#[derive(Debug)]
pub struct CustomError(String);
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}
impl Error for CustomError {}
impl CustomError {
    /// Creates a new `CustomError` with the provided message.
    ///
    /// # Arguments
    /// * `msg` - The error message.
    ///
    /// # Returns
    /// A new `CustomError` instance.
    pub fn new(msg: String) -> CustomError { CustomError(msg) }
}