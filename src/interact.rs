use std::str::FromStr;

pub fn text_required<Text>(prompt: &str) -> crate::Result<Text>
where
    Text: Clone + ToString + FromStr,
    <Text as FromStr>::Err: ToString + std::fmt::Debug,
{
    let text = dialoguer::Input::new()
        .with_prompt(prompt)
        .interact_text()?;
    Ok(text)
}

pub fn text_optional<Text>(prompt: &str) -> crate::Result<Option<Text>>
where
    Text: Clone + ToString + FromStr + std::ops::Deref<Target = str>,
    <Text as FromStr>::Err: ToString + std::fmt::Debug,
{
    let text: Text = dialoguer::Input::new()
        .with_prompt(prompt)
        .allow_empty(true)
        .interact_text()?;
    Ok(if text.is_empty() {None} else {Some(text)})
}

pub fn select_one<'op, Option: ToString>(prompt: &str, options: &'op [Option]) -> crate::Result<&'op Option> {
    let index = dialoguer::Select::new()
        .with_prompt(prompt)
        .items(options)
        .interact()?;
    Ok(&options[index])
}
