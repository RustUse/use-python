use use_python_keyword::{is_python_reserved_word, PythonKeyword, PythonSoftKeyword};

fn main() -> Result<(), use_python_keyword::PythonKeywordParseError> {
    let keyword: PythonKeyword = "async".parse()?;
    let soft_keyword: PythonSoftKeyword = "match".parse()?;

    assert_eq!(keyword.to_string(), "async");
    assert_eq!(soft_keyword.as_str(), "match");
    assert!(is_python_reserved_word("case"));
    Ok(())
}
