use std::fs;
use std::path::Path;

fn main() {
    let base_path = Path::new("mock_save");
    if base_path.exists() {
        fs::remove_dir_all(base_path).unwrap();
    }
    fs::create_dir_all(base_path).unwrap();
    
    // permissions.json
    fs::write(base_path.join("permissions.json"), r#"{
  "users": {
    "176197f8-eb00-40b9-b13f-75f1324644b4": { "groups": ["Adventure"] },
    "60ba750f-70b9-4143-8837-22551d315a4e": { "groups": ["Adventure"] }
  },
  "groups": {
    "Default": [],
    "OP": ["*"]
  }
}"#).unwrap();

    // config.json
    fs::write(base_path.join("config.json"), r#"{
  "Mods": {
    "EdgeTypE:Logica": { "Enabled": true },
    "JarHax:EyeSpy": { "Enabled": false }
  }
}"#).unwrap();

    // whitelist.json
    fs::write(base_path.join("whitelist.json"), r#"{"enabled":false,"list":["60ba750f-70b9-4143-8837-22551d315a4e"]}"#).unwrap();

    // bans.json
    fs::write(base_path.join("bans.json"), r#"[{"type":"infinite","target":"e9fde93b","by":"admin","timestamp":1770280761662,"reason":"Griefing."}]"#).unwrap();

    // client_metadata.json
    fs::write(base_path.join("client_metadata.json"), r#"{"CreatedWithPatchline": "release"}"#).unwrap();

    // universe/memories.json
    let universe_path = base_path.join("universe");
    fs::create_dir_all(&universe_path).unwrap();
    fs::write(universe_path.join("memories.json"), r#"{
  "Memories": [
    {
      "Id": "NPC",
      "NPCRole": "Snail_Magma",
      "TranslationKey": "server.npcRoles.Snail_Magma.name",
      "IsMemoriesNameOverridden": false,
      "CapturedTimestamp": 1768949234233,
      "FoundLocationNameKey": "server.map.region.Zone4_Tier5"
    }
  ]
}"#).unwrap();

    // universe/players/
    let players_path = universe_path.join("players");
    fs::create_dir_all(&players_path).unwrap();
    fs::write(players_path.join("60ba750f-70b9-4143-8837-22551d315a4e.json"), r#"{
  "Components": {
    "Nameplate": { "Text": "edgetype" },
    "Player": {
      "Version": 5,
      "GameMode": "Adventure",
      "Inventory": {
        "Storage": { "Id": "Simple", "Capacity": 36, "Items": {} },
        "HotBar": { "Id": "Simple", "Capacity": 9, "Items": {} }
      }
    }
  }
}"#).unwrap();

    // universe/worlds/default/config.json
    let world_path = universe_path.join("worlds").join("default");
    fs::create_dir_all(&world_path).unwrap();
    fs::write(world_path.join("config.json"), r#"{
  "DisplayName": "showcase",
  "Seed": 1770034372178,
  "GameMode": "Creative",
  "IsPvpEnabled": false,
  "ClientEffects": {
    "SunHeightPercent": 100.0,
    "SunAngleDegrees": 0.0,
    "SunIntensity": 0.25
  }
}"#).unwrap();

    println!("Mock data created at ./mock_save");
}
