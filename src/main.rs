use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
use toy_arms::external::{read, Process};
use walkdir::WalkDir;


const DISCORD_APP_ID: &str = "1513882325824307332";
const CACHE_FILE: &str = "song_database.json";

#[derive(Serialize, Deserialize, Clone)]
struct SongInfo {
    title: String,
    artist: String,
    we_tag: String,
    difficulties: HashMap<i32, String>,
}

fn main() {
    println!("Loading Song Database...");
    let song_db = load_or_build_database();
    println!("Successfully loaded {} songs!", song_db.len());

    println!("Connecting to Discord...");
    let mut client = DiscordIpcClient::new(DISCORD_APP_ID);
    let mut is_discord_connected = false;

    println!("Waiting for chusanApp.exe...");

    // 1. Attach to the game
    let process = loop {
        if let Ok(p) = Process::from_process_name("chusanApp.exe") {
            break p;
        }
        thread::sleep(Duration::from_secs(2));
    };
    println!("Attached to game!");

    // Get Memory Pointers
    let module_base = process.get_module_base("chusanApp.exe").expect("Could not find module base");
    let target_base = module_base + 0x0185F37C;

    let music_offsets: [usize; 4] = [0x3C, 0x10, 0x1B4, 0x2B4];
    let diff_offsets: [usize; 4] = [0x3C, 0x10, 0x1B4, 0x2B0];

    // Cache Variables
    let mut cached_music_id = -1;
    let mut cached_diff_id = 0;
    let mut last_state = String::new();
    let mut last_details = String::new();
    let mut start_time = 0;

    loop {
        // Reconnect to Discord if closed
        if !is_discord_connected {
            if client.connect().is_ok() {
                is_discord_connected = true;
                println!("Discord RPC Connected!");
            }
        }

        // Read Memory using our custom 32-bit helper function
        let music_id = read_pointer(&process, target_base, &music_offsets);
        let diff_id = read_pointer(&process, target_base, &diff_offsets);

        if let (Some(m_id), Some(d_id)) = (music_id, diff_id) {

            let mut detail_text = String::from("In Menus");
            let mut status_text = String::from("Navigating...");

            if m_id == -1 {
                // PLAYING (Music ID turned to -1)
                // Use cached variables
                if let Some(song) = song_db.get(&cached_music_id) {
                    detail_text = format!("Playing: {} - {}", song.artist, song.title);

                    let diff_str = song.difficulties.get(&cached_diff_id).unwrap_or(&String::from("?")).clone();
                    let diff_name = get_diff_name(cached_diff_id);

                    if cached_diff_id == 5 {
                        status_text = format!("WORLD'S END [{}]", song.we_tag);
                    } else {
                        status_text = format!("{} (Lv. {})", diff_name, diff_str);
                    }
                }
            } else {
                // SELECTING
                // Update cache with the live data
                cached_music_id = m_id;
                cached_diff_id = d_id;

                if let Some(song) = song_db.get(&cached_music_id) {
                    detail_text = String::from("Selecting Track...");
                    status_text = format!("{} - {}", song.artist, song.title);
                }
            }

            // Only send to Discord if the text changed!
            if detail_text != last_state || status_text != last_details {

                // Update our memory of what we just sent
                last_state = detail_text.clone();
                last_details = status_text.clone();
                // Get the exact current time for the Discord Timer
                start_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                if is_discord_connected {
                    let payload = activity::Activity::new()
                        // WHY DISCORD PUT STATUS ON THE BOTTOM??
                        .state(&detail_text)
                        .details(&status_text)
                        .timestamps(activity::Timestamps::new().start(start_time)) // Adds the timer
                        .assets(activity::Assets::new().large_image("logo_large_square"));
                    if let Err(e) = client.set_activity(payload) {
                        println!("Discord Rate Limit / Error: {}", e);
                        is_discord_connected = false;
                    } else {
                        println!("Discord Updated: {} | {}", detail_text, status_text);
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(1000));
    }
}

fn read_pointer(process: &Process, base_addr: usize, offsets: &[usize]) -> Option<i32> {
    let mut current_addr = read::<u32>(process.process_handle, base_addr).ok()? as usize;
    for &offset in offsets.iter().take(offsets.len() - 1) {
        current_addr = read::<u32>(process.process_handle, current_addr + offset).ok()? as usize;
    }
    read::<i32>(process.process_handle, current_addr + offsets.last().unwrap()).ok()
}

fn get_diff_name(id: i32) -> &'static str {
    match id {
        0 => "BASIC",
        1 => "ADVANCED",
        2 => "EXPERT",
        3 => "MASTER",
        4 => "ULTIMA",
        5 => "WORLD'S END",
        _ => "UNKNOWN",
    }
}

fn load_or_build_database() -> HashMap<i32, SongInfo> {
    if Path::new(CACHE_FILE).exists() {
        if let Ok(data) = fs::read_to_string(CACHE_FILE) {
            if let Ok(db) = serde_json::from_str(&data) {
                return db;
            }
        }
    }

    let mut db = HashMap::new();
    let dirs_to_scan = ["./bin/option", "../data"];

    for dir in dirs_to_scan {
        if !Path::new(dir).exists() { continue; }
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_name() == "Music.xml" {
                if let Ok(xml_content) = fs::read_to_string(entry.path()) {
                    if let Ok(doc) = Document::parse(&xml_content) {
                        let id: i32 = get_node_text(&doc, "name", "id").parse().unwrap_or(-1);
                        let title = get_node_text(&doc, "name", "str");
                        let artist = get_node_text(&doc, "artistName", "str");
                        let we_tag = get_node_text(&doc, "worldsEndTagName", "str");

                        if id == -1 { continue; }

                        let mut difficulties = HashMap::new();
                        for fumen in doc.descendants().filter(|n| n.has_tag_name("MusicFumenData")) {
                            let type_id: i32 = fumen.descendants().find(|n| n.has_tag_name("type"))
                                .and_then(|n| n.descendants().find(|n| n.has_tag_name("id")))
                                .and_then(|n| n.text()).unwrap_or("-1").parse().unwrap_or(-1);
                            let level = fumen.descendants().find(|n| n.has_tag_name("level")).and_then(|n| n.text()).unwrap_or("0");
                            let level_dec = fumen.descendants().find(|n| n.has_tag_name("levelDecimal")).and_then(|n| n.text()).unwrap_or("0");

                            if type_id != -1 {
                                difficulties.insert(type_id, format!("{}.{}", level, level_dec));
                            }
                        }
                        db.insert(id, SongInfo { title, artist, we_tag, difficulties });
                    }
                }
            }
        }
    }

    if let Ok(json) = serde_json::to_string_pretty(&db) {
        let _ = fs::write(CACHE_FILE, json);
    }
    db
}

fn get_node_text(doc: &Document, parent_tag: &str, child_tag: &str) -> String {
    doc.descendants()
        .find(|n| n.has_tag_name(parent_tag))
        .and_then(|n| n.descendants().find(|n| n.has_tag_name(child_tag)))
        .and_then(|n| n.text())
        .unwrap_or("")
        .to_string()
}