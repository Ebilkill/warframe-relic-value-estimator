use futures::Future;
use futures::Stream;

use hyper::client::HttpConnector;
use hyper::Client;
use hyper::Uri;

use hyper_tls::HttpsConnector;

use tokio_core::reactor::Core;

pub type ClientConnector = Client<HttpsConnector<HttpConnector>>;

pub fn get_server_string(client: &ClientConnector, core: &mut Core, url: Uri) -> String {
    println!("Sending a request to {}.", url);
    let mut res_string = String::new();
    {
        let request = client.get(url)
            .and_then(|res| {
                res.into_body().for_each(|chunk| {
                    let s = chunk.iter().map(|u| *u as char);
                    for c in s {
                        res_string.push(c);
                    }
                    Ok(())
                })
            })
            .map_err(|e| println!("Request got this error: {}", e));

        // request is a Future, futures are lazy, so must explicitly run
        core.run(request).unwrap();
    }

    res_string
}

