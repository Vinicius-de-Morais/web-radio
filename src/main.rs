use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, env, path::Path};

use bytes::Bytes;
use cytoplasm::cytoplasm::Cytoplasm;
use output_encoder::audio_encoder::OutputCodec;
use rocket::{
    http::ContentType,
    response::{content::RawHtml, stream::ByteStream},
};
use station::station::Station;
use track::track::{StationManifest, Track};

pub mod cytoplasm;
pub mod input_decoder;
pub mod output_encoder;
pub mod output_stream;
pub mod station;
pub mod track;

#[macro_use]
extern crate rocket;

fn schedule_next_track(station: Arc<Mutex<Station>>) {
    thread::spawn(move || loop {
        let ms = {
            let st = station.lock().unwrap();
            st.current_track.file_info.audio_milliseconds
        };
        thread::sleep(Duration::from_millis(ms));
        station.lock().unwrap().next();
    });
}

#[get("/")]
fn index() -> RawHtml<&'static [u8]> {
    return RawHtml(b"<!DOCTYPE html>\n<audio controls src='/station'>");
}

type StationMap = HashMap<String, Arc<Mutex<Station>>>;

#[get("/station")]
fn station_endpoint(state: &rocket::State<StationMap>) 
    -> (ContentType, ByteStream![Bytes]) 
{
    let station = state.get("RadioZero").unwrap();
    let station = station.lock().unwrap();
    let stream = station
        .cytoplasm
        .output_streams
        .get(&OutputCodec::Mp3_64kbps)
        .unwrap();

    stream.create_consumer_http_stream()
}

#[launch]
fn rocket() -> _ {
    let mut stations: StationMap = HashMap::new();

    for station_id in vec!["RadioZero"] {
        let (track_tx, track_rx) = channel::<Track>();
        let station_base_dir = Path::new(env::current_dir().unwrap().to_str().unwrap())
            .join("stations")
            .join(station_id);

        let manifest = StationManifest::from_base_dir(station_base_dir.clone())
            .expect("falha ao interpretar manifesto da estação");

        let cytoplasm = Cytoplasm::new(&[OutputCodec::Mp3_64kbps], track_rx);

        let station = Station::new(station_base_dir, manifest, cytoplasm, track_tx);

        let station = Arc::new(Mutex::new(station));


        // Preciso fazer a transição de DownState → PlayingState, se não fizer isso, quando chama o next() DownState::next não avança a faixa então vai ficar retornando a msm faixa sempre.
        {
            let mut st = station.lock().unwrap();
            st.play();
        }

        schedule_next_track(station.clone());

        stations.insert(station_id.to_owned(), station);
    }

    rocket::build()
        .manage(stations)
        .mount("/", routes![index, station_endpoint])
}
