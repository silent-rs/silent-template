use anyhow::Result;
use dto::test::Test;

pub async fn test() -> Result<Test> {
    Ok(Test {
        str: "Hello, world!".to_string(),
    })
}
