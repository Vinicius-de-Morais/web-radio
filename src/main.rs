use std::{
    collections::HashMap,
    sync::{mpsc, Arc, RwLock},
};

use bytes::Bytes;
use cytoplasm::cytoplasm::Cytoplasm;
use output_encoder::{ConsumerMap, OutputCodec};
use rocket::{
    http::ContentType,
    response::{content::RawHtml, stream::ByteStream},
};
use uuid::Uuid;

pub mod cytoplasm;
pub mod input_decoder;
pub mod output_encoder;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> RawHtml<&'static [u8]> {
    return RawHtml(b"<!DOCTYPE html>\n<audio controls src='/station'>");
}

/// Um frame MP3 silencioso para adicionar ao início de um stream de áudio MP3.
///
/// **Raciocínio:**
/// Ao transmitir dados MP3 raw, no início da stream, o decoder MP3 do cliente pode começar a
/// receber dados do meio de um frame, levando a erros de decoding e falhas no playback.
/// Esse é um frame MP3 válido e silencioso. Ao adicionar este frame ao início do stream,
/// garantimos que o decoder sempre comece com um ponto de SYNC conhecido, mesmo que o stream real
/// comece no meio de um frame MP3. Isso melhora significativamente a robustez da stream.
///
/// **Situação:**
/// Usada em aplicações que transmitem dados MP3 brutos via HTTP sem manipular explicitamente os
/// quadros frames MP3 no lado do servidor. É particularmente útil quando o servidor está gerando
/// o stream dinamicamente (por exemplo, usando ffmpeg) e não tem controle preciso
/// sobre o início da stream.
///
/// **Tradeoff**: O cliente recebe um pacote de 0.02s de áudio silencioso no começo da stream.
/// 879 bytes a mais no executável da aplicação.
///
/// `ffmpeg -f lavfi -i anullsrc=channel_layout=stereo:sample_rate=44100 -acodec libmp3lame -ab 128k -ac 2 -ar 44100 -t 0.02 -vn -sn -f mp3 - | head -c 1472 > mp3_null_frame.bin`
const NULL_MP3_FRAME: &[u8; 879] = include_bytes!("./mp3_null_frame.bin");

#[get("/station")]
fn station(state: &rocket::State<ConsumerMap>) -> (ContentType, ByteStream![Vec<u8>]) {
    let client_id = Uuid::new_v4();
    let (tx, rx) = mpsc::channel::<Bytes>();

    let mut consumer_map = state.write().unwrap();
    consumer_map.insert(client_id, tx);
    drop(consumer_map);

    (
        ContentType::new("audio", "mpeg"),
        ByteStream! {
            yield NULL_MP3_FRAME.to_vec();
            eprintln!("server: Frame MP3 null enviado");

            'receive: loop {
                match rx.recv() {
                    Ok(chunk) => {
                        yield Vec::from_iter(chunk.into_iter());
                    }
                    Err(e) => {
                        eprintln!("server: Broadcast channel closed: {:?}", e);
                        break 'receive;
                    }
                }
            }

            eprintln!("server: Closing stream")
        },
    )
}

#[launch]
fn rocket() -> _ {
    let consumer_map = Arc::new(RwLock::new(HashMap::new()));
    let output = output_encoder::AudioEncoder::new(OutputCodec::Mp3_64kbps, consumer_map.clone());

    Cytoplasm::new(output);

    rocket::build()
        .manage(consumer_map)
        .mount("/", routes![index, station])
}

//fn main() {
//
//}
