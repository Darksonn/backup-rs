extern crate backblaze_b2;
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha1;
extern crate walkdir;

use std::fs::File;
use std::io;
use std::io::{Write,BufReader};

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use backblaze_b2::raw::authorize::*;
use backblaze_b2::raw::buckets::*;
use backblaze_b2::raw::files::*;
use backblaze_b2::raw::upload::*;

use walkdir::WalkDir;
use walkdir::DirEntry;

#[derive(Deserialize)]
struct Options {
    file_listing: String,
    bucket_name: String,
    backup_root: String,
    ignored_prefixes_file: String,
}

fn get_bucket<'a>(buckets: &'a Vec<Bucket>, bucket_name: &str) -> Option<&'a Bucket> {
    for bucket in buckets {
        if &bucket.bucket_name == bucket_name {
            return Some(bucket);
        }
    }
    None
}

fn main() {
    let options: Options = serde_json::from_reader(File::open("options.txt").unwrap()).unwrap();

    let client = Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
    let connector = HttpsConnector::new(NativeTlsClient::new().unwrap());

    let cred_file = File::open("credentials.txt").unwrap();
    let cred: B2Credentials = serde_json::from_reader(cred_file).unwrap();
    let auth: B2Authorization = cred.authorize(&client).unwrap();

    let buckets = auth.list_buckets(&client).unwrap();
    let bucket = get_bucket(&buckets, &options.bucket_name).expect("no such bucket");

    let file_listing: Vec<FileInfo> = if let Ok(file) = File::open(&options.file_listing) {
        serde_json::from_reader(file).unwrap()
    } else {
        println!("fetching file listing");
        let fl = auth.list_all_file_names(&bucket.bucket_id, 10000, None, None, &client).unwrap().files;
        let mut out = File::create(&options.file_listing).unwrap();
        write!(out, "{}", serde_json::to_string(&fl).unwrap()).unwrap();
        fl
    };

    let upload = auth.get_upload_url(&bucket.bucket_id, &client).unwrap();
    let prefixes = options.create_prefixes().unwrap();

    let backup_manager = BackupManager {
        client: client, connector: connector,
        options: options, bucket: bucket,
        file_listing: file_listing, auth: auth,
        upload: upload, prefixes: prefixes
    }

    backup_manager.walk();

}

impl Options {
    fn create_prefixes(&self) -> io::Result<Prefixes> {
        let mut file = BufReader::new(&try!(File::open(&self.options.ignored_prefixes_file)));
        let mut res = Vec::new();
        let mut 
        for line in file.lines() {
            
        }
    }
}
struct Prefixes {
    prefixes: Vec<Vec<String>>
}

struct BackupManager<'bucket> {
    client: Client,
    connector: HttpsConnector,
    options: Options,
    prefixes: Prefixes,
    bucket: &'bucket Bucket,
    file_listing: Vec<FileInfo>,
    auth: B2Authorization,
    upload: UploadAuthorization
}

impl<'bucket> BackupManager<'bucket> {
    fn walk(self) {
        let walker = WalkDir::new(&options.backup_root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| self.prefixes.allowed(e));
        for entry in walker {
        }
    }
}

