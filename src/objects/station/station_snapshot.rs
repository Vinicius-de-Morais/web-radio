use std::ptr::null_mut;

use rocket::{futures::stream::Filter, http::private::Listener, time::Date};
use chrono::prelude::{NaiveDate, Local};

use crate::objects::{station::station::Station, subscriber::Subscriber, track::track::Track};

use super::station;

pub struct StationSnapshot {
    name: String,
    station: Station,
    current_track: Option<Track>,
    track_history: Vec<Track>,
    subscribers: Vec<Subscriber>,
    listeners: u32,
    created_on: NaiveDate,
}

impl StationSnapshot {
    
    pub fn new(station:Station) -> Self {
        Self {
            name: format!("{} - Snapshot", station.name),
            station,
            current_track: None,
            track_history:Vec::new(),
            subscribers:Vec::new(),
            listeners: 0,
            created_on: Local::now().date_naive()
        }
    }

    pub fn station(self) -> Station {
        return self.station;
    }

    pub fn name(&mut self) -> String{
        return self.name.clone();
    }
    
    pub fn creation(&mut self) -> NaiveDate {
        return self.created_on;
    }

    fn historic_trail_range(&mut self, track:Track){
        self.track_history.push(track.clone()); 
    }

    pub fn track_history_list(&mut self) -> Vec<Track> {
        return self.track_history.clone();
    }

    pub fn get_current_track(&mut self) -> Track {
        return self.current_track.clone().expect("Track ainda nao setada!");
    }
    
    pub fn set_current_track(&mut self, track:Track) {
        self.current_track = Some(track.clone());        
        self.historic_trail_range(track);
    }

    pub fn listener_in(&mut self) {
        self.listeners += 1;
    }
    
    pub fn listener_out(&mut self){
        self.listeners -= 1;
    }

    pub fn listeners_count(&mut self) -> u32 {
        return self.listeners;
    }

    pub fn subscribed_subscribers(&mut self) -> Vec<Subscriber> {
        return self.subscribers.clone();
    } 

    pub fn subscriber_unassign(&mut self, subout:Subscriber ) {
        self.subscribers.retain(|subscriber| subscriber != &subout );
    }

    pub fn subscriber_assign(&mut self, subs:Subscriber) {
        self.subscribers.push(subs);
    }

}