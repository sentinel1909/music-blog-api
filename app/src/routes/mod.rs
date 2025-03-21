pub mod ping;

use pavex::blueprint::{Blueprint, router::GET};
use pavex::f;

fn api_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.route(GET, "/ping", f!(self::ping::get));
    bp
}

pub fn register(bp: &mut Blueprint) {
    bp.domain("api.crusty-metallion.music")
        .prefix("/v1")
        .nest(api_bp());
}
