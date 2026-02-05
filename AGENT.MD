Hytale Save Editor yapacağız. Rust ile. GUI'de olacak.


Bir klasör seçilecek. Seçilen klasör x olsun.

x/preview.png gui image.

x/permissions.json:
{
  "users": {
    "176197f8-eb00-40b9-b13f-75f1324644b4": {
      "groups": [
        "Adventure"
      ]
    },
    "60ba750f-70b9-4143-8837-22551d315a4e": {
      "groups": [
        "Adventure"
      ]
    },
    "576239f2-03d1-4051-b7a6-c2affd7e6a7b": {
      "groups": [
        "Adventure"
      ]
    },
    "e9fde93b-02b0-4b75-9e51-a1d9b4155aa2": {
      "groups": [
        "Adventure"
      ]
    }
  },
  "groups": {
    "Default": [],
    "OP": [
      "*"
    ]
  }
}
user'ların gamemode'unu değiştirme, OP oyuncu ekleme, kaldırma. 


x/config.json:
{
  "Mods": {
    "AnimalGrowth:AnimalGrowthPlugin": {
      "Enabled": false
    },
    "Example:ExamplePlugin": {
      "Enabled": false
    },
    "JarHax:EyeSpy": {
      "Enabled": false
    },
    "Goygoy Engine:HytaleTurkceYama": {
      "Enabled": false
    },
    "EdgeTypE:Logica": {
      "Enabled": true
    },
    "soulforge:soulforgePlugin": {
      "Enabled": false
    }
  }
}
`mod aktifleştirme, devre dışı bırakma özelliği eklenecek.`

x/whitelist.json:
{"enabled":false,"list":["60ba750f-70b9-4143-8837-22551d315a4e"]}
whitelist'e user ekleme, user çıkarma, whitelist'i aktifleştirme ve devredışı bırakma eklenecek.

x/bans.json:
[{"type":"infinite","target":"e9fde93b-02b0-4b75-9e51-a1d9b4155aa2","by":"00000000-0000-0000-0000-000000000000","timestamp":1770280761662,"reason":"No reason."}]
ban atma, ban kaldırma.



client_metadata.json:
{
  "CreatedWithPatchline": "release"
}



x/universe/memories.json:
{
  "Memories": [
    {
      "Id": "NPC",
      "NPCRole": "Snail_Magma",
      "TranslationKey": "server.npcRoles.Snail_Magma.name",
      "IsMemoriesNameOverridden": false,
      "CapturedTimestamp": 1768949234233,
      "FoundLocationNameKey": "server.map.region.Zone4_Tier5"
    },
    {
      "Id": "NPC",
      "NPCRole": "Skeleton_Incandescent_Fighter",
      "TranslationKey": "server.npcRoles.Skeleton_Incandescent_Fighter.name",
      "IsMemoriesNameOverridden": true,
      "CapturedTimestamp": 1768939267706,
      "FoundLocationNameKey": "server.map.region.Zone2_Tier2"
    },
    {
      "Id": "NPC",
      "NPCRole": "Hawk",
      "TranslationKey": "server.npcRoles.Hawk.name",
      "IsMemoriesNameOverridden": false,
      "CapturedTimestamp": 1768917722898,
      "FoundLocationNameKey": "server.map.region.Zone3_Tier2"
    },
    {
      "Id": "NPC",
      "NPCRole": "Mouflon_Lamb",
      "TranslationKey": "server.npcRoles.Mouflon_Lamb.name",
      "IsMemoriesNameOverridden": false,
      "CapturedTimestamp": 1769551693852,
      "FoundLocationNameKey": "server.map.region.Zone3_Tier2"
    },
    {
      "Id": "NPC",
      "NPCRole": "Scarak_Fighter",
      "TranslationKey": "server.npcRoles.Scarak_Fighter.name",
      "IsMemoriesNameOverridden": true,
      "CapturedTimestamp": 1768908007657,
      "FoundLocationNameKey": "server.map.region.Zone2_Tier3"
    },
    {
      "Id": "NPC",
      "NPCRole": "Snapdragon",
      "TranslationKey": "server.npcRoles.Snapdragon.name",
      "IsMemoriesNameOverridden": false,
      "CapturedTimestamp": 1769090025926,
      "FoundLocationNameKey": "server.map.region.Zone4_Tier5"
    },
    {
      "Id": "NPC",
      "NPCRole": "Goblin_Miner",
      "TranslationKey": "server.npcRoles.Goblin_Miner.name",
      "IsMemoriesNameOverridden": true,
      "CapturedTimestamp": 1768941677748,
      "FoundLocationNameKey": "server.map.region.Zone1_Tier2"
    }
  ]
}
memories editleme, captured timestamp vesayre değiştirme. yeni memories ekleme, var olanı kaldırma, var olanın IsMemoriesNameOverridden değerini değiştirme.




x/universe/players/uuid.json:
60ba750f-70b9-4143-8837-22551d315a4e.json:
{
  "Components": {
    "Nameplate": {
      "Text": "edgetype"
    },
    "EffectController": {},
    "ObjectiveHistory": {
      "ObjectiveHistory": {},
      "ObjectiveLineHistory": {}
    },
    "DisplayName": {
      "DisplayName": {
        "RawText": "edgetype",
        "Bold": null,
        "Italic": null,
        "Monospace": null,
        "Underline": null
      }
    },
    "UIComponentList": {},
    "Transform": {
      "Position": {
        "X": -366.1812744140625,
        "Y": 122.44999694824219,
        "Z": -422.3251037597656
      },
      "Rotation": {
        "Pitch": 0.0,
        "Yaw": 7.085043907165527,
        "Roll": 0.0
      }
    },
    "PlayerMemories": {
      "Capacity": 48,
      "Memories": [
        {
          "Id": "NPC",
          "NPCRole": "Shark_Hammerhead",
          "TranslationKey": "server.npcRoles.Shark_Hammerhead.name",
          "IsMemoriesNameOverridden": false,
          "CapturedTimestamp": 1769336999964,
          "FoundLocationNameKey": "server.map.region.Zone1_Tier2"
        },
        {
          "Id": "NPC",
          "NPCRole": "Rex_Cave",
          "TranslationKey": "server.npcRoles.Rex_Cave.name",
          "IsMemoriesNameOverridden": false,
          "CapturedTimestamp": 1769519430532,
          "FoundLocationNameKey": "server.map.region.Zone1_Shallow_Ocean"
        },
        {
          "Id": "NPC",
          "NPCRole": "Whale_Humpback",
          "TranslationKey": "server.npcRoles.Whale_Humpback.name",
          "IsMemoriesNameOverridden": false,
          "CapturedTimestamp": 1769519460779,
          "FoundLocationNameKey": "server.map.region.Zone1_Shore"
        },
        {
          "Id": "NPC",
          "NPCRole": "Skeleton_Burnt_Alchemist",
          "TranslationKey": "server.npcRoles.Skeleton_Burnt_Alchemist.name",
          "IsMemoriesNameOverridden": true,
          "CapturedTimestamp": 1770112572893,
          "FoundLocationNameKey": "server.map.region.Zone1_Tier2"
        },
        {
          "Id": "NPC",
          "NPCRole": "Goblin_Lobber",
          "TranslationKey": "server.npcRoles.Goblin_Lobber.name",
          "IsMemoriesNameOverridden": true,
          "CapturedTimestamp": 1770203393621,
          "FoundLocationNameKey": "server.map.region.Zone1_Tier2"
        },
        {
          "Id": "NPC",
          "NPCRole": "Goblin_Scavenger",
          "TranslationKey": "server.npcRoles.Goblin_Scavenger.name",
          "IsMemoriesNameOverridden": true,
          "CapturedTimestamp": 1770203628483,
          "FoundLocationNameKey": "server.map.region.Zone1_Tier2"
        }
      ]
    },
    "BuilderTools": {
      "SelectionHistory": true
    },
    "Velocity": {
      "Velocity": {
        "X": 0.0,
        "Y": 0.0,
        "Z": 0.0
      }
    },
    "ExplorationData": {
      "ExploredChunks": []
    },
    "Player": {
      "Version": 5,
      "UUID": {
        "$binary": "YLp1D3C5QUOINyJVHTFaTg==",
        "$type": "04"
      },
      "Inventory": {
        "Version": 4,
        "Storage": {
          "Id": "Simple",
          "Capacity": 36,
          "Items": {
            "0": {
              "Id": "Tool_Pickaxe_Crude",
              "Quantity": 1,
              "Durability": 47.75,
              "MaxDurability": 150.0,
              "OverrideDroppedItemAnimation": false
            },
            "33": {
              "Id": "Tool_Capture_Crate",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "29": {
              "Id": "Ingredient_Life_Essence",
              "Quantity": 21,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "25": {
              "Id": "BlameJared_MinersHelmet_Armor_Miners_Helmet",
              "Quantity": 1,
              "Durability": 38.60000000000001,
              "MaxDurability": 108.0,
              "OverrideDroppedItemAnimation": false
            },
            "4": {
              "Id": "*Container_Bucket_State_Filled_Water",
              "Quantity": 1,
              "Durability": 0.7000000000000001,
              "MaxDurability": 1.0,
              "OverrideDroppedItemAnimation": false
            },
            "12": {
              "Id": "Ingredient_Tree_Sap",
              "Quantity": 7,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "13": {
              "Id": "Ingredient_Leather_Light",
              "Quantity": 4,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "8": {
              "Id": "Ingredient_Fabric_Scrap_Linen",
              "Quantity": 4,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "9": {
              "Id": "Tool_Pickaxe_Copper",
              "Quantity": 1,
              "Durability": 200.0,
              "MaxDurability": 200.0,
              "OverrideDroppedItemAnimation": false
            },
            "24": {
              "Id": "Ingredient_Fire_Essence",
              "Quantity": 16,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "23": {
              "Id": "Ingredient_Fabric_Scrap_Cindercloth",
              "Quantity": 6,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "26": {
              "Id": "IronBoat",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "16": {
              "Id": "Ingredient_Fibre",
              "Quantity": 21,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "17": {
              "Id": "Tool_Pickaxe_Copper",
              "Quantity": 1,
              "Durability": 200.0,
              "MaxDurability": 200.0,
              "OverrideDroppedItemAnimation": false
            },
            "19": {
              "Id": "Plant_Seeds_Pumpkin_Eternal",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "18": {
              "Id": "Ingredient_Fabric_Scrap_Wool",
              "Quantity": 5,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "20": {
              "Id": "Ingredient_Life_Essence",
              "Quantity": 80,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "22": {
              "Id": "Ingredient_Bone_Fragment",
              "Quantity": 6,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "21": {
              "Id": "Tool_Shovel_Iron",
              "Quantity": 1,
              "Durability": 74.24999999999983,
              "MaxDurability": 250.0,
              "OverrideDroppedItemAnimation": false
            },
            "35": {
              "Id": "Tool_Pickaxe_Copper",
              "Quantity": 1,
              "Durability": 70.5833329608043,
              "MaxDurability": 200.0,
              "OverrideDroppedItemAnimation": false
            },
            "2": {
              "Id": "Furniture_Village_Window",
              "Quantity": 20,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "6": {
              "Id": "Wood_Dry_Trunk",
              "Quantity": 41,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "27": {
              "Id": "Rock_Stone_Cobble",
              "Quantity": 3,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "15": {
              "Id": "WoodBoat",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "14": {
              "Id": "Tool_Pickaxe_Iron",
              "Quantity": 1,
              "Durability": 250.0,
              "MaxDurability": 250.0,
              "OverrideDroppedItemAnimation": false
            },
            "10": {
              "Id": "Plant_Coral_Block_Red",
              "Quantity": 34,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "11": {
              "Id": "Ingredient_Stick",
              "Quantity": 19,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "31": {
              "Id": "Ingredient_Leather_Heavy",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "32": {
              "Id": "Tool_Capture_Crate",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "1": {
              "Id": "Rubble_Volcanic",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "3": {
              "Id": "NoCube_Neon_Block_Red",
              "Quantity": 8,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "7": {
              "Id": "Weapon_Arrow_Crude",
              "Quantity": 83,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "5": {
              "Id": "Rock_Volcanic_Cobble",
              "Quantity": 100,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            }
          }
        },
        "Armor": {
          "Id": "Simple",
          "Capacity": 4,
          "Items": {
            "2": {
              "Id": "Armor_Iron_Hands",
              "Quantity": 1,
              "Durability": 43.5,
              "MaxDurability": 90.0,
              "OverrideDroppedItemAnimation": false
            },
            "1": {
              "Id": "Armor_Iron_Chest",
              "Quantity": 1,
              "Durability": 42.0,
              "MaxDurability": 80.0,
              "OverrideDroppedItemAnimation": false
            },
            "3": {
              "Id": "Armor_Steel_Ancient_Legs",
              "Quantity": 1,
              "Durability": 34.89999999999999,
              "MaxDurability": 67.0,
              "OverrideDroppedItemAnimation": false
            }
          }
        },
        "HotBar": {
          "Id": "Simple",
          "Capacity": 9,
          "Items": {
            "0": {
              "Id": "Weapon_Sword_Copper",
              "Quantity": 1,
              "Durability": 54.22000000000103,
              "MaxDurability": 100.0,
              "OverrideDroppedItemAnimation": false
            },
            "2": {
              "Id": "Weapon_Shortbow_Copper",
              "Quantity": 1,
              "Durability": 93.62000000000002,
              "MaxDurability": 100.0,
              "OverrideDroppedItemAnimation": false
            },
            "6": {
              "Id": "Tool_Hoe_Iron",
              "Quantity": 1,
              "Durability": 154.0,
              "MaxDurability": 300.0,
              "OverrideDroppedItemAnimation": false
            },
            "4": {
              "Id": "Food_Vegetable_Cooked",
              "Quantity": 5,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "8": {
              "Id": "Glass_Panel",
              "Quantity": 62,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "1": {
              "Id": "Tool_Pickaxe_Iron",
              "Quantity": 1,
              "Durability": 192.0,
              "MaxDurability": 225.0,
              "OverrideDroppedItemAnimation": false
            },
            "3": {
              "Id": "Food_Kebab_Vegetable",
              "Quantity": 8,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "7": {
              "Id": "Rock_Volcanic_Cobble",
              "Quantity": 96,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "5": {
              "Id": "Tool_Hatchet_Copper",
              "Quantity": 1,
              "Durability": 163.2500005401671,
              "MaxDurability": 200.0,
              "OverrideDroppedItemAnimation": false
            }
          }
        },
        "Utility": {
          "Id": "Simple",
          "Capacity": 4,
          "Items": {
            "0": {
              "Id": "Furniture_Crude_Torch",
              "Quantity": 9,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "2": {
              "Id": "Furniture_Crude_Torch",
              "Quantity": 2,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "1": {
              "Id": "Weapon_Shield_Copper",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            }
          }
        },
        "Backpack": {
          "Id": "Simple",
          "Capacity": 9,
          "Items": {
            "0": {
              "Id": "Tool_Trap_Bait",
              "Quantity": 2,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "2": {
              "Id": "Wood_Ash_Trunk",
              "Quantity": 2,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "6": {
              "Id": "NoCube_Neon_Block_White",
              "Quantity": 8,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "4": {
              "Id": "Rubble_Stone",
              "Quantity": 2,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "8": {
              "Id": "Rock_Volcanic_Cobble",
              "Quantity": 100,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "1": {
              "Id": "Deco_Iron_Bars",
              "Quantity": 2,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "3": {
              "Id": "Plant_Moss_Block_Blue",
              "Quantity": 48,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "7": {
              "Id": "Ingredient_Crystal_White",
              "Quantity": 2,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "5": {
              "Id": "Furniture_Tavern_Bed",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            }
          }
        },
        "ActiveHotbarSlot": 0,
        "Tool": {
          "Id": "Simple",
          "Capacity": 23,
          "Items": {
            "0": {
              "Id": "EditorTool_Paint",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "2": {
              "Id": "EditorTool_Selection",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "4": {
              "Id": "EditorTool_Line",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "1": {
              "Id": "EditorTool_Sculpt",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "3": {
              "Id": "EditorTool_Paste",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            },
            "5": {
              "Id": "EditorTool_Layers",
              "Quantity": 1,
              "Durability": 0.0,
              "MaxDurability": 0.0,
              "OverrideDroppedItemAnimation": false
            }
          }
        },
        "ActiveToolsSlot": -1,
        "ActiveUtilitySlot": 1,
        "SortType": "Name"
      },
      "PlayerData": {
        "BlockIdVersion": 11,
        "World": "edigetigin",
        "KnownRecipes": [
          "Food_Salad_Caesar",
          "Food_Pie_Pumpkin"
        ],
        "PerWorldData": {
          "edigetigin": {
            "LastPosition": {
              "X": -365.8999938964844,
              "Y": 121.9000015258789,
              "Z": -423.6499938964844,
              "Pitch": -0.2738308906555176,
              "Yaw": 8.377580642700195,
              "Roll": 0.0
            },
            "LastMovementStates": {
              "Flying": false
            },
            "FirstSpawn": false,
            "RespawnPoints": [
              {
                "BlockPosition": {
                  "X": -366,
                  "Y": 121,
                  "Z": -423
                },
                "RespawnPosition": {
                  "X": -365.92500001192093,
                  "Y": 122.45000004768372,
                  "Z": -423.47500002384186
                },
                "Name": "edgetype's bed"
              }
            ],
            "DeathPositions": [
              {
                "MarkerId": "death-marker-97aa6507-6a2a-4585-a88f-2ce32f0366ac",
                "Transform": {
                  "X": -377.7067565917969,
                  "Y": 120.14759826660156,
                  "Z": -353.9605407714844,
                  "Pitch": 0.0,
                  "Yaw": 0.0,
                  "Roll": 0.0
                },
                "Day": 107
              },
              {
                "MarkerId": "death-marker-4e4fca80-5051-41fb-9daa-e797403e3371",
                "Transform": {
                  "X": -358.9189147949219,
                  "Y": 117.0,
                  "Z": -430.656494140625,
                  "Pitch": 0.0,
                  "Yaw": 0.0,
                  "Roll": 0.0
                },
                "Day": 109
              },
              {
                "MarkerId": "death-marker-7a3a651c-1454-48cb-a70e-ccd57f153d6d",
                "Transform": {
                  "X": -477.61846923828125,
                  "Y": 84.35880279541016,
                  "Z": -607.6117553710938,
                  "Pitch": 0.0,
                  "Yaw": 0.0,
                  "Roll": 0.0
                },
                "Day": 109
              },
              {
                "MarkerId": "death-marker-da08eebb-320d-4a11-8783-adca815a1752",
                "Transform": {
                  "X": -475.4424133300781,
                  "Y": 72.96575927734375,
                  "Z": -603.3251342773438,
                  "Pitch": 0.0,
                  "Yaw": 0.0,
                  "Roll": 0.0
                },
                "Day": 109
              },
              {
                "MarkerId": "death-marker-824a39fa-71f1-46f4-96c7-48503a51f48b",
                "Transform": {
                  "X": -475.3251037597656,
                  "Y": 63.14990234375,
                  "Z": -599.6748657226562,
                  "Pitch": 0.0,
                  "Yaw": 0.0,
                  "Roll": 0.0
                },
                "Day": 110
              }
            ]
          }
        },
        "DiscoveredZones": [
          "ForgottenTemple",
          "Zone1_Shore",
          "Zone3_Shallow_Ocean_Tier1",
          "Zone4_Tier5",
          "Zone1_Spawn",
          "Zone1_Tier1",
          "Zone1_Tier2",
          "Zone1_Tier3",
          "Zone4_Shallow_Ocean_Tier5",
          "Zone4_Shallow_Ocean_Tier4",
          "Zone4_Shore_Tier5",
          "Zone2_Shore",
          "Zone1_Shallow_Ocean",
          "Oceans",
          "Zone2_Tier1",
          "Zone2_Tier2",
          "Zone2_Tier3",
          "Zone3_Shore_Tier1",
          "Zone3_Tier1",
          "Zone3_Tier2",
          "Zone3_Tier3",
          "Zone2_Shallow_Ocean",
          "Zone1_Temple"
        ],
        "DiscoveredInstances": [
          {
            "$binary": "k5XnwvQHT6WO3Wb8MsdxBQ==",
            "$type": "04"
          },
          {
            "$binary": "AZYC8rlmQS+muFy0ZsfQXw==",
            "$type": "04"
          },
          {
            "$binary": "K6XFF1QgQQ2mnPSbuVxNqQ==",
            "$type": "04"
          },
          {
            "$binary": "B1DkFZSGS8mzsDzvDahqlQ==",
            "$type": "04"
          },
          {
            "$binary": "ROz8Na9aS4unuJEFNSFBrw==",
            "$type": "04"
          }
        ],
        "ReputationData": {},
        "ActiveObjectiveUUIDs": []
      },
      "BlockPlacementOverride": false,
      "HotbarManager": {
        "SavedHotbars": [
          null,
          null,
          null,
          null,
          null,
          null,
          null,
          null,
          null,
          null
        ],
        "CurrentHotbar": 0
      },
      "GameMode": "Adventure"
    },
    "HitboxCollision": {
      "HitboxCollisionConfigIndex": 1
    },
    "UniqueItemUsages": {
      "UniqueItemUsed": [
        "Upgrade_Backpack_1"
      ]
    },
    "Instance": {},
    "UUID": {
      "UUID": {
        "$binary": "YLp1D3C5QUOINyJVHTFaTg==",
        "$type": "04"
      }
    },
    "EntityStats": {
      "Version": 5,
      "Stats": {
        "Oxygen": {
          "Id": "Oxygen",
          "Value": 100.0
        },
        "Mana": {
          "Id": "Mana",
          "Value": 0.0
        },
        "Health": {
          "Id": "Health",
          "Value": 137.0,
          "Modifiers": {
            "Armor_ADDITIVE": {
              "Id": "Static",
              "CalculationType": "Additive",
              "Amount": 37.0
            }
          }
        },
        "GlidingActive": {
          "Id": "GlidingActive",
          "Value": 0.0
        },
        "Immunity": {
          "Id": "Immunity",
          "Value": 0.0
        },
        "SignatureCharges": {
          "Id": "SignatureCharges",
          "Value": 0.0
        },
        "Stamina": {
          "Id": "Stamina",
          "Value": 10.0
        },
        "Ammo": {
          "Id": "Ammo",
          "Value": 0.0
        },
        "Unknown": {
          "Id": "Unknown",
          "Value": 0.0
        },
        "SignatureEnergy": {
          "Id": "SignatureEnergy",
          "Value": 0.0,
          "Modifiers": {
            "*Weapon_0": {
              "Id": "Static",
              "CalculationType": "Additive",
              "Amount": 20.0
            }
          }
        },
        "DeployablePreview": {
          "Id": "DeployablePreview",
          "Value": 0.0
        },
        "StaminaRegenDelay": {
          "Id": "StaminaRegenDelay",
          "Value": 0.0
        },
        "MagicCharges": {
          "Id": "MagicCharges",
          "Value": 0.0
        }
      }
    },
    "HeadRotation": {
      "Rotation": {
        "Pitch": -0.3228307366371155,
        "Yaw": 0.016460344195365906,
        "Roll": 0.0
      }
    }
  }
}

envanterine item ekleme, kullanıcı istatistiklerini güncelleme, stamina etc. EntityStats değiştirme, envanter görüntüleme ve editleme. Konumunu değiştirme vesayre.



x/universe\worlds\default/config.json
`default` yerine başka isimler de olabilir, o dizindeki klasörler listelenecek ve kontrol yapılacak.:
{
  "Version": 4,
  "UUID": {
    "$binary": "uXcyga/dRsSuMcATInQnqA==",
    "$type": "04"
  },
  "DisplayName": "showcase",
  "Seed": 1770034372178,
  "SpawnProvider": {
    "Id": "Global",
    "SpawnPoint": {
      "X": 0.5,
      "Y": 80.0,
      "Z": 0.5,
      "Pitch": 0.0,
      "Yaw": 180.0,
      "Roll": 0.0
    }
  },
  "WorldGen": {
    "Type": "HytaleGenerator",
    "WorldStructure": "Default_Flat"
  },
  "WorldMap": {
    "Type": "WorldGen"
  },
  "ChunkStorage": {
    "Type": "Hytale"
  },
  "ChunkConfig": {},
  "IsTicking": true,
  "IsBlockTicking": true,
  "IsPvpEnabled": false,
  "IsFallDamageEnabled": false,
  "IsGameTimePaused": false,
  "GameTime": "0001-02-04T12:53:23.888882717Z",
  "ClientEffects": {
    "SunHeightPercent": 100.0,
    "SunAngleDegrees": 0.0,
    "BloomIntensity": 0.30000001192092896,
    "BloomPower": 8.0,
    "SunIntensity": 0.25,
    "SunshaftIntensity": 0.30000001192092896,
    "SunshaftScaleFactor": 4.0
  },
  "RequiredPlugins": {},
  "GameMode": "Creative",
  "IsSpawningNPC": false,
  "IsSpawnMarkersEnabled": true,
  "IsAllNPCFrozen": false,
  "GameplayConfig": "Default",
  "IsCompassUpdating": true,
  "IsSavingPlayers": true,
  "IsSavingChunks": true,
  "SaveNewChunks": true,
  "IsUnloadingChunks": true,
  "IsObjectiveMarkersEnabled": true,
  "DeleteOnUniverseStart": false,
  "DeleteOnRemove": false,
  "ResourceStorage": {
    "Type": "Hytale"
  },
  "Plugin": {}
}

World Detayları: her bir değer güncellenebilir olacak.



Örnek Save File Yolu: 
C:\Users\Çağrı\AppData\Roaming\Hytale\UserData\Saves\showcase\