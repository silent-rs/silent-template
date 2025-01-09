use dto::test::Test;
use silent::prelude::{HandlerAppend, Request, Result, Route};

pub fn get_route() -> Route {
    Route::new("").get(test)
}

pub async fn test(_req: Request) -> Result<Test> {
    Ok(services::test::test().await?)
}
