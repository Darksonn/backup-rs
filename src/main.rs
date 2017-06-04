extern crate backblaze_b2;
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate rand;
extern crate sha1;

fn main() {
    let client = Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
    let connector = HttpsConnector::new(NativeTlsClient::new().unwrap());

    let cred_file = File::open("credentials.txt").unwrap();
    let cred: B2Credentials = serde_json::from_reader(cred_file).unwrap();
    let auth: B2Authorization = cred.authorize(&client).unwrap();

}
