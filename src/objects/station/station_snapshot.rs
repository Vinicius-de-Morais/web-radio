use std::ptr::null_mut;

use rocket::{http::private::Listener, time::Date};
use chrono::prelude::{NaiveDate, Local};

use crate::objects::{subscriber::Subscriber, track::track::Track, station::station::Station};

struct StationSnapshot {
    pub name: String,
    pub owner_station: Station,
    current_track: Option<Track>,
    track_history: Vec<Track>,
    subscribers: Vec<Subscriber>,
    listeners: u32,
    created_on: NaiveDate,
}

impl StationSnapshot {
    
    pub fn new(owner_station:Station) -> Self {
        Self {
            name: format!("{} - snapshot", owner_station.name),
            owner_station,
            current_track: None,
            track_history:Vec::new(),
            subscribers:Vec::new(),
            listeners: 0,
            created_on: Local::now().date_naive()
        }
    }

    fn historic_trail_range(&mut self, track:Track){
        self.track_history
            .push(
                Some(track.clone())
                .expect("Erro ao adicionar a track no historico!!!!")
            ); 
    }

    pub fn set_current_track(&mut self, track:Track) {
        self.current_track = Some(track.clone());        

        self.historic_trail_range(track);
    }

    pub fn get_track_history(&mut self) -> Vec<Track> {
        return self.track_history.clone();
    }

    pub fn listener_in(&mut self) {
        self.listeners += 1;
    }
    
    pub fn listener_out(&mut self){
        self.listeners -= 1;
    }

    pub fn get_listener_count(&mut self) -> u32 {
        return self.listeners;
    }

}