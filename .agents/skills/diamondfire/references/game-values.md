# DiamondFire game values

_Generated from actiondump (patch 8.13). In a template a game value is `{"id":"g_val","data":{"type":"<exact name below>","target":"Default"}}`. The `type` string is the display name with colour codes removed (verified on 6 names from sample templates)._

## Statistical Values (28)

- **Current Health** (`NUMBER`) — Gets a target's remaining health points. _Returns:_ 0.0 (dead) up to the target's maximum health (20.0 by default). _Info:_ ❤ = 2 Health _Aliases:_ Remaining Health.
- **Maximum Health** (`NUMBER`) — Gets a target's maximum health points. _Returns:_ Maximum health, 1.0 or above. _Info:_ ❤ = 2 Health
- **Absorption Health** (`NUMBER`) — Gets a target's absorption health (golden hearts). _Returns:_ Absorption health. _Info:_ ❤ = 2 Health
- **Food Level** (`NUMBER`) — Gets a target's remaining food points. _Returns:_ 0 (starving) to 20 (full bar).
- **Food Saturation** (`NUMBER`) — Gets a target's saturation level, which depends on the types of food consumed. _Returns:_ 0.0 (minimum), up to the player's food level. _Info:_ If saturation is > 0.0, the player's food level will not drop.
- **Food Exhaustion** (`NUMBER`) — Gets a target's exhaustion level, which is increased by the player's actions. _Returns:_ 0.0 (minimum) to 4.0 (reset point). _Info:_ When exhaustion resets from 4.0 to 0.0, a player's saturation decreases by 1.
- **Attack Damage** (`NUMBER`) — Gets a target's attack damage, which has a base value that can be altered by items. _Returns:_ 0.0 or higher (more damage). _Info:_ Default base value = 1.0
- **Attack Speed** (`NUMBER`) — Gets a target's attack speed, which has a base value that can be altered by items. _Returns:_ 0.0 or higher (faster). _Info:_ Default base value = 4.0
- **Attack Cooldown** (`NUMBER`) — Gets a target's current attack cooldown as a percentage of the way to fully charged _Returns:_ 0.0 (fully uncharged) to 100.0 (fully charged).
- **Attack Cooldown Ticks** (`NUMBER`) — Gets the number of ticks it will take to fully charge a target's attack cooldown after attacking with their held item. _Returns:_ 0.0 (instant) or above. _Info:_ This value is set to -1 if it will never recharge.
- **Armor Points** (`NUMBER`) — Gets a target's armor points, which has a base value that can be altered by items. _Returns:_ 0.0 (no armor) to 20.0 (full bar). _Info:_ Default base value = 0.0
- **Armor Toughness** (`NUMBER`) — Gets a target's armor toughness, which has a base value that can be altered by items. _Returns:_ 0.0 or above (full set of diamond armor = 8.0). _Info:_ Armor Toughness increases the amount of damage required to penetrate one armor point. Default base value = 0.0
- **Invulnerability Ticks** (`NUMBER`) — Gets a target's remaining ticks of invulnerability. _Returns:_ 0 (can be hurt) or above (invulnerable). _Info:_ This value is set to 10 upon taking damage.
- **Experience Level** (`NUMBER`) — Gets a target's experience level. _Returns:_ 0 (no levels) or above.
- **Experience Progress** (`NUMBER`) — Gets a target's experience progress percentage to the next level. _Returns:_ 0.0% (no progress) to 100.0% (next level).
- **Fire Ticks** (`NUMBER`) — Gets the remaining ticks a target is on fire for. _Returns:_ 0 (not on fire) or above (burning).
- **Freeze Ticks** (`NUMBER`) — Gets the remaining ticks a target is freezing for. _Returns:_ 0 (not frozen) or above (freezing).
- **Remaining Air** (`NUMBER`) — Gets a target's remaining air ticks. _Returns:_ 0 (drowning) to 300 (maximum air). _Info:_ One breath bubble is equal to 30 air ticks.
- **Fall Distance** (`NUMBER`) — Gets a target's distance fallen in blocks. _Returns:_ 0.0 (not falling) or higher (falling down). _Info:_ Resets to 0 upon landing. Works on the Damage Event.
- **Held Slot** (`NUMBER`) — Gets a target's selected hotbar slot index. _Returns:_ 1 (leftmost slot) to 9 (rightmost slot).
- **Ping** (`NUMBER`) — Gets the latency between a player and the server in milliseconds. _Returns:_ Player ping ,,╻╻┃ 0 to 149 ,,╻╻┃ 150 to 299 ,,╻╻┃ 300 to 599 ,,╻╻┃ 600 to 999 ,,╻╻┃ over 1000.
- **Steer Sideways Movement** (`NUMBER`) — While a player is steering an entity, gets the sideways movement of the steering. _Returns:_ -1 (right), 1 (left), or 0 (none).
- **Steer Forward Movement** (`NUMBER`) — While a player is steering an entity, gets the forward movement of the steering. _Returns:_ -1 (backward), 1 (forward), or 0 (none).
- **Item Usage Progress** (`NUMBER`) — Gets the progress percentage of a target using their held item, such as food. _Returns:_ 0 (not using an item), or 0.0% (start) to 100.0%. _Info:_ Bows, crossbows and tridents remain at 100.0% until they are released.
- **Flight Speed** (`NUMBER`) — Gets a target's flight speed as a percentage. _Returns:_ Flight speed percentage (0% to 1000%).
- **Walk Speed** (`NUMBER`) — Gets a target's walk speed as a percentage. _Returns:_ Walk speed percentage (0% to 500%).
- **Entity Width** (`NUMBER`) — Gets the width of an entity's bounding box. _Returns:_ Width in blocks.
- **Entity Height** (`NUMBER`) — Gets the height of an entity's bounding box. _Returns:_ Height in blocks.

## Locational Values (15)

- **Location** (`LOCATION`) — Gets a target's location. _Returns:_ Location and rotation, at feet height.
- **Target Block Location** (`LOCATION`) — Gets the location of the block a target is looking at. _Returns:_ Block center.
- **Target Block Side** (`VECTOR`) — Gets the side of the block a target is looking at as a direction. _Returns:_ Block side. _Aliases:_ Target Block Face.
- **Eye Location** (`LOCATION`) — Gets a target's location, but adjusted to its eye height. _Returns:_ Eye location and rotation.
- **X-Coordinate** (`NUMBER`) — Gets the X coordinate of a target's position. _Returns:_ Coordinate.
- **Y-Coordinate** (`NUMBER`) — Gets the Y coordinate of a target's position. _Returns:_ Coordinate.
- **Z-Coordinate** (`NUMBER`) — Gets the Z coordinate of a target's position. _Returns:_ Coordinate.
- **Midpoint Location** (`LOCATION`) — Gets the location of the center of the target's bounding box. _Returns:_ Location and rotation, at midpoint.
- **Pitch** (`NUMBER`) — Gets the pitch (up/down rotation) of a target's position. _Returns:_ -90.0 to 90.0. _Info:_ -90.0° = up 90.0° = down
- **Yaw** (`NUMBER`) — Gets the yaw (left/right rotation) of a target's position. _Returns:_ -180.0 to 180.0. _Info:_ -180.0° & 180.0° = north -90.0° = east 0.0° = south 90.0° = west
- **Body Yaw** (`NUMBER`) — Gets the yaw (left/right rotation) of a target's body. _Returns:_ -180.0 to 180.0. _Info:_ -180.0° & 180.0° = north -90.0° = east 0.0° = south 90.0° = west
- **Standing Block Location** (`LOCATION`) — Gets the location of the block that is supporting the player. _Returns:_ Block center. _Info:_ When not grounded, this will return the location of the block at the target's location.
- **Spawn Location** (`LOCATION`) — Gets a target's original spawn location. _Returns:_ Location this entity was created at.
- **Velocity** (`VECTOR`) — Gets the speed at which a target is moving (not walking) in each direction. _Returns:_ Movement velocity. _Info:_ When grounded, a target may still have a downward velocity due to how gravity is applied.
- **Direction** (`VECTOR`) — Gets the looking direction of a target's location as a vector. _Returns:_ Direction vector (length of 1.0).

## Item Values (10)

- **Main Hand Item** (`ITEM`) — Gets a target's currently held item. _Returns:_ Item in the selected hotbar slot.
- **Off Hand Item** (`ITEM`) — Gets a target's currently held off hand item. _Returns:_ Item in the offhand slot.
- **Armor Items** (`LIST`) — Gets the items in a target's armor slots. _Returns:_ Contains one Item entry for each armor slot (air if empty, 4 in total). _Info:_ Armor slots are ordered from helmet to boots.
- **Hotbar Items** (`LIST`) — Gets a target's current hotbar items. _Returns:_ Contains one Item entry for each hotbar slot (air if empty, 9 in total).
- **Full Inventory Items** (`LIST`) — Gets a target's inventory items (includes hotbar). _Returns:_ Contains one Item entry for each inventory slot (air if empty, 36 in total). _Aliases:_ Inventory Items.
- **Upper Inventory Items** (`LIST`) — Gets a target's inventory items. _Returns:_ Contains one Item entry for each inventory slot (air if empty, 27 in total).
- **Cursor Item** (`ITEM`) — Gets the item on a target's cursor (used when moving items in the inventory). _Returns:_ Cursor item.
- **Inventory Menu Items** (`LIST`) — Gets a target's current inventory menu items. _Returns:_ Contains one Item entry for each menu slot (air if empty). _Info:_ Works with container inventories.
- **Saddle Item** (`ITEM`) — Gets a target's currently worn saddle or carpet. _Returns:_ Item in the saddle/decor slot.
- **Entity Item** (`ITEM`) — The item form of the target. _Returns:_ The entity item.

## Informational Values (27)

- **Name ** (`COMPONENT`) — Gets a target's name. _Returns:_ Target name.
- **UUID** (`TEXT`) — Gets a target's universally unique identifier. _Returns:_ Target UUID.
- **Entity Type** (`TEXT`) — Gets a target's entity type. _Returns:_ Entity type, e.g. "tipped_arrow" or "cow".
- **Game Mode** (`TEXT`) — Gets a player's game mode. _Returns:_ Game mode: "survival", "creative", "adventure", "spectator".
- **Open Inventory Title ** (`COMPONENT`) — Gets the title of a target's opened inventory. _Returns:_ Inventory title, or "none" if either no menu or the player's regular inventory is open.
- **Potion Effects** (`LIST`) — Gets a target's active potion effects. _Returns:_ Contains one Potion Effect entry for each active effect on the target.
- **Vehicle** (`TEXT`) — Gets the UUID of the entity that the target is riding. _Returns:_ UUID of the ridden entity, or "none" if the target is not riding one. _Info:_ The ridden entity does not need to be of vehicular type. In a stack of entities, the vehicle is the bottom entity.
- **Passengers ** (`LIST`) — Gets the UUIDs of any entities riding a target. _Returns:_ Contains one String entry (UUID) for each passenger riding the target (Empty List if the target has no passengers).
- **Lead Holder** (`TEXT`) — Gets the entity that is holding a target on a lead. _Returns:_ Lead holder UUID, or "none" if the target is not on a lead.
- **Attached Leads** (`LIST`) — Gets all entities attached to to a target by a lead. _Returns:_ Contains one String entry (UUID) for each leashed entity (Empty List if the target holds no leads).
- **Targeted Entity UUID** (`TEXT`) — Gets the UUID of the entity that the target is targeting. _Returns:_ UUID of the targeted entity, or "none" if the target is not chasing one. _Info:_ The targeted entity is the entity the target is chasing.
- **Projectile Shooter UUID** (`TEXT`) — Gets the UUID of a projectile's shooter, or "none" if not set. _Returns:_ Shooter UUID.
- **Display Entity Left Rotation** (`VECTOR`) — Gets the left rotation of a display entity's transformation.  This is the default rotation type. _Returns:_ Pitch/Yaw/Roll Vector. _(rank: Mythic)_
- **Display Entity Right Rotation** (`VECTOR`) — Gets the right rotation of a display entity's transformation. _Returns:_ Pitch/Yaw/Roll Vector. _(rank: Mythic)_
- **Display Entity Translation** (`VECTOR`) — Gets the translation of a display entity's transformation. _Returns:_ Translation. _(rank: Mythic)_
- **Display Entity Scale** (`VECTOR`) — Gets the scale of a display entity's transformation. _Returns:_ Scale. _(rank: Mythic)_
- **Display Transformation Matrix** (`LIST`) — Gets the transformation matrix of a display entity. _Returns:_ List of 16 numbers representing a row-major matrix.. _Info:_ This is not guaranteed to be exactly equal to a set transformation matrix due to float-point conversion imprecision. _(rank: Mythic)_
- **Display Entity Item** (`ITEM`) — Gets the item of an item or block display entity. _Returns:_ Display Item/Block. _(rank: Mythic)_
- **Display Block Tags** (`BLOCK_TAG`) — Gets the block tags of a block display. _Returns:_ Block Tags. _(rank: Mythic)_
- **Text Display Text** (`COMPONENT`) — Gets the text of a text display. _Returns:_ Text. _(rank: Mythic)_
- **Pose** (`TEXT`) — Gets the target's pose. _Returns:_ Pose: "dying" (dead), "fall_flying" (gliding), "sleeping", "sneaking", "spin_attack" (riptiding with a trident), "standing" (standing normally), "swimming" (swimming/crawling) Camel Only: "sitting" Frog Only: "croaking", "long_jumping", "using_tongue" Warden Only: "digging", "emerging", "roaring", "sniffing" Breeze Only: "inhaling", "shooting", "sliding".
- **Villager Trades** (`LIST`) — Gets a list of dictionaries representing the trades of a villager. _Returns:_ A list of dictionaries with two entries "result": result item "ingredients": list of ingredients.
- **Weather Type** (`TEXT`) — Gets a player's weather type. _Returns:_ Weather: "clear", "downfall".
- **Pick Entity Result** (`SPAWN_EGG`) — Gets the item that would be created when a player middle clicks the target in Creative mode. _Returns:_ Item or air.
- **Particle Visibility Status** (`TEXT`) — Gets the target's Particles setting. _Returns:_ Particle Visibility Status: "all", "decreased", "minimal".
- **Pressed Movement Keys** (`LIST`) — Gets a list of all movement keys that are currently pressed by a player. _Returns:_ Contains one String value for each movement type: "forward", "backward", "left", "right", "jump", "sneak", "sprint".
- **Main Hand** (`TEXT`) — Gets the player's main hand. _Returns:_ Main Hand: "left", "right",.

## Event Values (48)

- **Event Block Location** (`LOCATION`) — Gets the location of the block in this event. _Returns:_ Block center.
- **Event Block Side** (`VECTOR`) — Gets the side of the block that was hit in this event as a direction. _Returns:_ Block side.
- **Event Destination Block Location** (`LOCATION`) — Gets the location of the destination block in this event. _Returns:_ Block center.
- **Event Damage** (`NUMBER`) — Gets the amount of damage dealt in this event. Includes damage reduction. _Returns:_ 0.0 or above. _Info:_ ❤ = 2 Health
- **Damage Event Cause** (`TEXT`) — Gets the type of damage taken or dealt in this event. _Returns:_ Damage Cause: "block_explosion", "campfire", "contact" (cactus), "cramming", "custom" (damage action), "dragon_breath", "drowning", "dryout", (fish on land), "entity_attack", "entity_explosion", "entity_sweep_attack", "fall", "falling_block", "fire" (in fire block), "fire_tick", "fly_into_wall", "freeze", "hot_floor" (magma block), "kill", "lava", "lightning", "magic", "melting" (snowman), "poison", "projectile", "sonic_boom", "starvation", "suffocation", "thorns", "void", "wither", "world_border".
- **Raw Event Damage** (`NUMBER`) — Gets the amount of damage dealt in this event before any damage reductions. _Returns:_ 0.0 or above. _Info:_ ❤ = 2 Health
- **Event Knockback** (`VECTOR`) — Gets the knockback vector in this event. _Returns:_ Knockback vector.
- **Knockback Event Cause** (`TEXT`) — Gets the cause of the knockback in this event. _Returns:_ Knockback Cause: "damage" (non-entity damage), "entity_attack", "explosion", "shield_block", "sweep_attack", "push" (generic push), "unknown".
- **Event Death Message ** (`COMPONENT`) — Gets the death message for this death event. _Returns:_ Death message.
- **Event Heal Amount** (`NUMBER`) — Gets the amount of health regained in this event. _Returns:_ Health regained. _Info:_ ❤ = 2 Health
- **Heal Event Cause** (`TEXT`) — Gets the reason the target regained health in this event. _Returns:_ Healing Cause: "natural" (natural regeneration), "magic" (instant health effect), "magic_regen" (regen effect), "custom" (code).
- **Event Affected Blocks** (`LIST`) — Gets the locations of blocks affected in this event. _Returns:_ Contains one Location entry for each block. _Aliases:_ Event Explosion Affected Blocks.
- **Event Power** (`NUMBER`) — Gets the percentage of power this event was executed with. _Returns:_ 0.0% to 100.0%. _Aliases:_ Event Bow Power.
- **Event Command** (`TEXT`) — Gets the entire command line entered in this event. _Returns:_ Command, with the first "@" excluded. _(rank: Emperor)_
- **Event Command Arguments** (`LIST`) — Gets the separated parts of the event command. _Returns:_ Contains one String entry for each word in the command (split by " "). _(rank: Emperor)_
- **Event Item** (`ITEM`) — Gets the item in an item related event. _Returns:_ Main item in event.
- **Event Hotbar Slot** (`NUMBER`) — Gets the hotbar slot being changed to in this event. _Returns:_ 1 (leftmost slot) to 9 (rightmost slot). _Aliases:_ Event New Held Slot.
- **Event Clicked Slot Index** (`NUMBER`) — Gets the index of the clicked inventory slot in this event. _Returns:_ From 1 (first slot) up to the inventory's size.
- **Event Clicked Slot Item** (`ITEM`) — Gets the inventory item clicked on in this event. _Returns:_ Item in slot (before the click event).
- **Event Clicked Slot New Item** (`ITEM`) — Gets the inventory item clicked with in this event. _Returns:_ Item in slot (after the click event).
- **Close Inventory Event Cause** (`TEXT`) — Gets the reason the target's inventory was closed in this event. _Returns:_ Close Cause: "player", "code", "open_new", "teleport", "unloaded", "cant_use", "disconnect", "death", "unknown". _(rank: Emperor)_
- **Inventory Event Click Type** (`TEXT`) — Gets the click type in this inventory click event. _Returns:_ Click Type: "left", "shift_left", "double_left", "right", "shift_right", "drop" (Q), "drop_stack" (Ctrl+Q), "copy" (middle click), "creative", "swap_offhand" (F), "swap_hotbar" (1 ... 9). _(requires tokens)_
- **Fish Event Cause** (`TEXT`) — Gets the cause of this fish event. _Returns:_ Event Type: "cast", "bite", "catch", "cancel", "cancel_block", "miss", "pull_entity". _Aliases:_ Event Fish Type.
- **Teleport Event Cause** (`TEXT`) — Gets the reason the player was teleported in this event. _Returns:_ Teleport Cause: "code", "ender_pearl", "consumable_effect", "unknown".
- **Teleport Location** (`LOCATION`) — Gets the location that will be teleported to in this event. _Returns:_ Location.
- **Exhaustion Event Cause** (`TEXT`) — Gets the reason the target became exhausted in this event. _Returns:_ Exhaustion Cause: "attack", "block_mined", "custom", "crouch", "damaged", "hunger_effect", "jump", "jump_sprint", "none", "regen", "sprint", "swim", "walk", "walk_on_water", "walk_underwater".
- **Event Exhaustion** (`NUMBER`) — Gets the amount of exhaustion gained in this event. _Returns:_ Number.
- **Transform Event Cause** (`TEXT`) — Gets the reason the target transformed in this event. _Returns:_ Transform Cause: "cured" (Zombie Villager → Villager), "drowned" (Zombie → Drowned), "frozen" (Skeleton → Stray), "infection" (Villager → Zombie Villager), "metamorphosis" (Tadpole → Frog), "sheared" (Mooshroom → Cow), "split" (Slime → Small Slimes), "lightning", "piglin_zombified", "unknown".
- **Event Transform Entities** (`LIST`) — Gets the entities an entity transforms into. _Returns:_ UUIDs of the new entity/entities.
- **Event Hit Type** (`TEXT`) — Gets the type of object that the projectile collided with _Returns:_ Hit Type: "block", "entity".
- **Product ID** (`TEXT`) — Gets the ID of the product purchased. _Returns:_ Product ID.
- **Event Message** (`TEXT`) — The message sent in this event _Returns:_ Chat message. _(rank: Mythic)_
- **Event Sign Text** (`LIST`) — Gets the sign text in this event. _Returns:_ Contains one String entry for each sign line..
- **Event Sign Side** (`TEXT`) — Gets the sign side modified in this event. _Returns:_ "front" or "back".
- **Trade Ingredients** (`LIST`) — Gets the items given to a villager in a trade. _Returns:_ List of items.
- **Trade Result** (`ITEM`) — Gets the result item of a villager trade. _Returns:_ Result item.
- **Combust Event Duration** (`NUMBER`) — Gets the duration of fire inflicted in this event. _Returns:_ Fire duration in ticks.
- **Combust Event Cause** (`TEXT`) — Gets the reason the target caught on fire in this event. _Returns:_ Combust Cause: "player", "entity", "block", "code", "unknown".
- **Event Air Change** (`NUMBER`) — Gets the change in air level in this event. _Returns:_ Change in air in ticks. _Info:_ Each bubble is equal to 30 air ticks.
- **Start Spectate Event Cause** (`TEXT`) — Gets the cause for starting to spectate in this event. _Returns:_ Start Spectate Event Cause: "code" (Player Action: Spectate Target), "manual" (Clicking an entity).
- **Event Vault State** (`TEXT`) — Gets the state of the vault before this event. _Returns:_ State: "inactive", "active", "unlocking", "ejecting".
- **Event New Vault State** (`TEXT`) — Gets the state of the vault after this event. _Returns:_ State: "inactive", "active", "unlocking", "ejecting".
- **Event Redstone Current Strength** (`NUMBER`) — Gets the strength of the redstone current before this event. _Returns:_ Redstone power (0-16).
- **Event New Redstone Current Strength** (`NUMBER`) — Gets the strength of the redstone current after this event. _Returns:_ Redstone power (0-16).
- **Edit Book Event Old Book** (`ITEM`) — Gets the original book of an Edit Book Event. _Returns:_ Old Book. _Info:_ This item will always have a stack size of one.
- **Edit Book Event New Book** (`ITEM`) — Gets the resulting book of an edit book event. _Returns:_ New Book. _Info:_ This item will always have a stack size of one.
- **Click Dialog Event Dialog ID** (`TEXT`) — Gets the ID of the dialog that was clicked. _Returns:_ ID of the clicked Dialog.
- **Click Dialog Event Button ID** (`TEXT`) — Gets the ID of the button that was clicked. _Returns:_ ID of the clicked Button.

## Plot Values (15)

- **Player Count** (`NUMBER`) — Gets the amount of players playing on the plot. _Returns:_ Player count.
- **CPU Usage** (`NUMBER`) — Gets the percentage of the plot's CPU being used this instant. _Returns:_ Usage as a percentage. Can go above 100%..
- **Server TPS** (`NUMBER`) — Gets the amount of game Ticks Per Second the server is currently able to handle. _Returns:_ 20.0 (no server lag) or below (decreases with more lag).
- **Timestamp** (`NUMBER`) — Gets the current time as one number in seconds. E.g.: 1418840496.5 means Dec 17, 2014, 18:21:36. _Returns:_ Current time. _Info:_ The number represents the total seconds passed since 1970 (Unix Time).
- **Selection Size** (`NUMBER`) — Gets the amount of targets in the selection. _Returns:_ 0 (no targets) or above.
- **Selection Target Names** (`LIST`) — Gets the name of each target in the selection. _Returns:_ Contains one String entry (name) for each target.
- **Selection Target UUIDs** (`LIST`) — Gets the UUID of each target in the selection. _Returns:_ Contains one String entry (UUID) for each target.
- **Plot ID** (`TEXT`) — Gets the id of the plot as a string. _Returns:_ Plot ID.
- **Plot Name** (`COMPONENT`) — Gets the name of the plot as a styled text. _Returns:_ Plot Name.
- **Plot Size** (`NUMBER`) — Gets the size of the plot. _Returns:_ Plot size in blocks. _Info:_ A basic plot will return 51. A large plot will return 101. A massive plot will return 301. A mega plot will return 1001.
- **Microseconds Since Startup** (`NUMBER`) — Gets the number of microseconds elapsed since the first player joined the plot. The decimal part represents nanoseconds. _Returns:_ Microseconds.
- **Ticks Since Startup** (`NUMBER`) — Gets the number of ticks that elapsed since the first player joined the plot. _Returns:_ Ticks.
- **Active Block Transactions** (`NUMBER`) — Gets the number of block transactions a plot is executing. _Returns:_ Active Block Transactions. _(rank: Overlord, advanced)_
- **Plot Player Names** (`LIST`) — Gets the name of each player on the plot. _Returns:_ Contains one String entry (name) for each player.
- **Plot Player UUIDs** (`LIST`) — Gets the UUID of each player on the plot. _Returns:_ Contains one String entry (UUID) for each player.
