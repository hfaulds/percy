use isomorphic_websockets_server::server::serve;

fn main() {
    env_logger::init();

    let static_files = {
        // Development
        #[cfg(debug_assertions)]
        {
            format!("{}/../client/build", env!("CARGO_MANIFEST_DIR"))
        }

        // Production
        #[cfg(not(debug_assertions))]
        {
            format!("{}/../client/dist", env!("CARGO_MANIFEST_DIR"))
        }
    };

    serve(static_files)
}
