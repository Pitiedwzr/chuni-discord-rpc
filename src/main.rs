use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

fn main() {
    let discord_app_id = "1513882325824307332";
    let target_process = "chusanApp";
    let interval = 5;

    println!("Waiting for {} to start...", target_process);

    let mut sys = System::new_all();
    let mut client = DiscordIpcClient::new(discord_app_id);

    let mut is_connected = false;

    // Loop every 5 seconds
    loop {
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing(),
        );

        let mut game_is_running = false;
        for _process in sys.processes_by_name(target_process.as_ref()) {
            game_is_running = true;
            break;
        }

        if game_is_running {
            if !is_connected {
                println!("{} detected! Connecting to Discord...", target_process);

                if client.connect().is_ok() {
                    is_connected = true;

                    let payload = activity::Activity::new()
                        .state("Playing") // Bottom text
                        .details("Playing CHUNITHM") // Top text
                        .assets(
                            activity::Assets::new()
                                .large_image("logo_large_square")
                                .large_text("CHUNITHM X-VERSE-X"),
                        );

                    // Send the presence to Discord
                    if let Err(e) = client.set_activity(payload) {
                        println!("Failed to set activity: {}", e);
                    } else {
                        println!("Discord Rich Presence is now active!");
                    }
                } else {
                    println!("Make sure Discord is running!");
                }
            }
        } else {
            if is_connected {
                println!("{} closed. Removing Discord presence.", target_process);
                let _ = client.close();
                is_connected = false;
                break;
            }
        }

        thread::sleep(Duration::from_secs(interval));
    }
}