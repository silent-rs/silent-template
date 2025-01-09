mod test;

use silent::prelude::Route;

pub fn get_routes() -> Route {
    Route::new("").append(test::get_route())
}
