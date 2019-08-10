use std::fs::read;
use std::path::Path;
use std::rc::Rc;
use ws::{listen, Handler, Request, Response};
use isomorphic_websockets_app::App;
use mime_guess::from_path;

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

struct Server<'a> {
    static_files: Rc<&'a Path>
}

impl<'a> Handler for Server<'a> {
    fn on_request(&mut self, req: &Request) -> ws::Result<(Response)> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            path if path.starts_with("/static") => self.handle_static(path),
            path @ _ => self.respond(path),
        }
    }
}
impl<'a> Server<'a> {
    fn handle_static(&self, path: &str) -> ws::Result<(Response)> {
        let rel_path = Path::new(&path).strip_prefix("/static").unwrap();
        let fspath = match self.static_files.join(&rel_path).canonicalize() {
            Ok(fspath) => fspath,
            _ => return Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        };
        let bytes = match read(fspath) {
            Ok(bytes) => bytes,
            _ => return Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        };
        let mut resp = Response::new(200, "OK", bytes);
        match from_path(path).first() {
            Some(mime) => {
                resp.headers_mut().push(
                    ("Content-Type".to_string(), mime.to_string().as_bytes().to_vec()),
                );
            },
            _ => {},
        };
        return Ok(resp)
    }

    fn respond(&self, path: &str) -> ws::Result<(Response)> {
        let app = App::new(1001, path.to_string());
        let state = app.store.borrow();

        let html = format!("{}", include_str!("./index.html"));
        let html = html.replacen(HTML_PLACEHOLDER, &app.render().to_string(), 1);
        let html = html.replacen(STATE_PLACEHOLDER, &state.to_json(), 1);
        Ok(Response::new(200, "OK", html.as_bytes().to_vec()))
    }
}

pub fn serve(static_files: String) {
    println!("{}", static_files);
    let static_files = Rc::new(Path::new(&static_files));
    println!("Ws server listening on port 7878");
    listen("127.0.0.1:7878", |_out| {
        Server { static_files: Rc::clone(&static_files) }
    }).unwrap();
}
