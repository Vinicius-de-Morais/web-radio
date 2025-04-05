use rocket::{
    http::ContentType,
    response::{
        content::{RawCss, RawHtml},
        stream::ByteStream,
    },
    tokio::sync::broadcast,
};

#[macro_use]
extern crate rocket;

use rocket::response::stream::{EventStream, Event};
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;
use tokio::sync::broadcast::{Sender};
use serde::Serialize;

// Estrutura que gerencia o broadcast de áudio
struct AudioBroadcaster {
    sender: Sender<Vec<u8>>,
}

// Estrutura para metadados da estação (mock)
#[derive(Serialize)]
struct Metadata {
    current_track: String,
    subscribers: usize,
    likes: usize,
}

#[get("/stream")]
async fn stream(broadcaster: &State<Arc<AudioBroadcaster>>) -> EventStream![] {
    // O padrão Observer é aplicado aqui: cada cliente se inscreve no canal de broadcast.
    let mut rx = broadcaster.sender.subscribe();
    EventStream! {
        loop {
            // Em um cenário real, chunks de áudio seriam enviados.
            // Aqui usamos um mock para simular a transmissão.
            match rx.recv().await {
                Ok(chunk) => yield Event::data(base64::encode(&chunk)),
                Err(_) => break,
            }
        }
    }
}

// Endpoint para obter os metadados da estação
#[get("/metadata")]
fn metadata() -> Json<Metadata> {
    // Os valores aqui são mock e servem para teste do frontend.
    Json(Metadata {
        current_track: "Faixa Exemplo".to_string(),
        subscribers: 5,
        likes: 10,
    })
}

#[launch]
fn rocket() -> _ {
    // Criação do canal de broadcast para áudio.
    // O buffer de 8 mensagens é arbitrário e pode ser ajustado conforme a necessidade.
    let (tx, _) = broadcast::channel::<Vec<u8>>(8);

    // enviando dados mock periodicamente em uma thread separada.
    // Em uma implementação real, isso seria substituído pela lógica do iterator do AudioFile.
    let broadcaster = Arc::new(AudioBroadcaster { sender: tx.clone() });
    let broadcaster_clone = Arc::clone(&broadcaster);

    // Thread mock para envio de dados (simula o streaming de áudio)
    std::thread::spawn(move || loop {
        // Mock: dados de áudio simulados (em um cenário real, ler de um arquivo)
        let mock_data = vec![0u8; 1024];
        let _ = broadcaster_clone.sender.send(mock_data);
        std::thread::sleep(std::time::Duration::from_millis(500));
    });

    rocket::build()
        .manage(broadcaster)
        .mount("/", routes![stream, metadata])
}