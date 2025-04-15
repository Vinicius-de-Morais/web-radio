#[cfg(test)]
mod test_station_snapshot {
    use std::clone;
    use std::ops::Deref;

    use chrono::Local;
    use web_radio::objects::station::station::Station;
    use web_radio::objects::station::station_snapshot::StationSnapshot;
    use web_radio::objects::station::station_state::MockStationState;
    use web_radio::objects::track::track::Track;
    // use web_radio::objects::subscriber::Subscriber;


    fn get_station_test() -> Station {
        let station_state = MockStationState::new();
        Station::new(
            "Diamond City Radio".to_owned(),
            "./diamond_city_radio/".to_owned(),
            98.9,
            Box::new(station_state),
        )
    }

    fn mock_track() -> Track {
        Track::new(
            "Mocked Title".to_string(),
            "Mocked Artist".to_string(),
            "Mocked Album".to_string(),
            300,
            "mp3".to_string(),
            "mocked_source.mp3".to_string(),
            vec![], // narrations after
            vec![], // narrations before
        )
    }

    #[test]
    fn test_create_snapshot() {
        let station = get_station_test();
        let station_name = station.name.clone();
        let mut snapshot = StationSnapshot::new(station);

        assert_eq!(snapshot.name(), format!("{} - Snapshot", station_name));
        assert_eq!(snapshot.listeners_count(), 0);
        assert!(snapshot.track_history_list().is_empty());
        assert!(snapshot.subscribed_subscribers().is_empty());
        assert_eq!(snapshot.creation(), Local::now().date_naive());
    }

    #[test]
    fn test_set_current_track_and_history() {
        let station = get_station_test();
        let mut snapshot = StationSnapshot::new(station);
        let track = mock_track();

        snapshot.set_current_track(track.clone());

        assert_eq!(snapshot.get_current_track().title, "Mocked Title");
        assert_eq!(snapshot.track_history_list().len(), 1);
    }

    #[test]
    fn test_get_track_history() {
        let station = get_station_test();
        let mut snapshot = StationSnapshot::new(station);

        let t1 = mock_track();
        let t2 = Track { title: "Second Track".into(), ..mock_track() };

        snapshot.set_current_track(t1.clone());
        snapshot.set_current_track(t2.clone());

        let history = snapshot.track_history_list();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].title, t1.title);
        assert_eq!(history[1].title, t2.title);
    }

    #[test]
    fn test_listener_in_out() {
        let station = get_station_test();
        let mut snapshot = StationSnapshot::new(station);

        snapshot.listener_in();
        snapshot.listener_in();
        assert_eq!(snapshot.listeners_count(), 2);

        snapshot.listener_out();
        assert_eq!(snapshot.listeners_count(), 1);
    }
}
