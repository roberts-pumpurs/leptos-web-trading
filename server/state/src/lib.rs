pub mod webapp_state;

use actix::System;
use webapp_state::WebAppState;

pub fn spawn_actix_rt() -> (WebAppState, std::thread::JoinHandle<Result<(), std::io::Error>>) {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    let handle = std::thread::spawn(move || {
        let sys = System::new();

        tx.send(System::current()).unwrap();

        sys.run()
    });

    let sys = rx.recv().unwrap();
    (WebAppState::new(sys.arbiter().clone()), handle)
}
