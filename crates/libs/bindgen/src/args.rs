use crate::Result;

pub fn expand<I, S>(args: I) -> Result<Vec<String>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut result = vec![];
    from_iter(&mut result, args)?;
    Ok(result)
}

// This function is needed to avoid a recursion limit in the Rust compiler.
fn from_string(result: &mut Vec<String>, value: &str) -> Result<()> {
    from_iter(result, value.split_whitespace().map(|arg| arg.to_string()))
}

fn from_iter<I, S>(result: &mut Vec<String>, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut expand = false;

    for arg in args.into_iter().map(|arg| arg.as_ref().to_string()) {
        if arg.starts_with('-') {
            expand = false;
        }
        if expand {
            for args in crate::read_file_lines(&arg)? {
                if !args.starts_with("//") {
                    from_string(result, &args)?;
                }
            }
        } else if arg == "--etc" {
            expand = true;
        } else {
            result.push(arg);
        }
    }

    Ok(())
}
