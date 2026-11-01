# DiamondFire actions — Player Action (`player_action`)

_Generated from actiondump (patch 8.13). Action names are exact wire strings; names in double quotes include **leading/trailing spaces** that must be kept._

## Block `player_action` — Player Action (212 actions in this file)

Used to do something related to a player or multiple players.

### `SetHotbar`
**Set Hotbar Items**
Sets items in a player's hotbar.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to set — _Slots 1-9_

### `SetReducedDebug`
**Set Reduced Debug Info Enabled**
When enabled, a player won't be able to see their coordinates, block info, or other info.
- **Tags:**
   - `Reduced Debug Info Enabled` (slot 26): `Enable` (default), `Disable`

### `CloseInv`
**Close Inventory**
Closes a player's inventory.
- **Tags:**
   - `Close Player Inventory` (slot 26): `Enable` (default), `Disable`
       - `Enable`: Allows closing a player's own inventory or chat.
       - `Disable`: Allows closing a player's own inventory or chat.
- _Flags:_ rank: Emperor

### `GiveItems`
**Give Items**
Gives a player all of the items in the chest.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to give
   - `[1]` `NUMBER` _(optional)_ — Amount to give

### `NoKeepInv`
**Lose Items on Death**
> ⚠ **LEGACY** → use `SetInventoryKept` (Set Inventory Kept tag to Disable.)
When this code block is executed, the player will no longer keep the contents of their inventory when they die.

### `SetHandCrafting`
**Set Allow Hand Crafting**
Sets if a player is allowed to interact with their hand-crafting menu.
- **Tags:**
   - `Allow Hand Crafting` (slot 26): `Enable`, `Disable` (default)

### `BossBar`
**Set Boss Bar** · aliases: `SetBossBar`
> ⚠ **LEGACY** → use `" SetBossBar "`
Creates or modifies a custom boss health bar at the top of a player's screen.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(optional)_ — Title
   - `[1]` `NUMBER` _(optional)_ — Current health — _Default = 100_
   - `[2]` `NUMBER` _(optional)_ — Maximum health — _Default = 100_
- **Tags:**
   - `Bar Slot` (slot 23): `Slot 1` (default), `Slot 2`, `Slot 3`, `Slot 4`, `Slot 5`, `Slot 6`, `Slot 7`, `Slot 8`, `Slot 9`
       - `Slot 1`: Creates or modifies a boss bar in slot 1.
       - `Slot 2`: Creates or modifies a boss bar in slot 2.
       - `Slot 3`: Creates or modifies a boss bar in slot 3.
       - `Slot 4`: Creates or modifies a boss bar in slot 4.
       - `Slot 5`: Creates or modifies a boss bar in slot 5.
       - `Slot 6`: Creates or modifies a boss bar in slot 6.
       - `Slot 7`: Creates or modifies a boss bar in slot 7.
       - `Slot 8`: Creates or modifies a boss bar in slot 8.
       - `Slot 9`: Creates or modifies a boss bar in slot 9.
   - `Bar Style` (slot 25): `Solid` (default), `6 segments`, `10 segments`, `12 segments`, `20 segments`
       - `Solid`: The boss bar has no segments.
       - `6 segments`: Each segment covers 16.66% of the boss bar.
       - `10 segments`: Each segment covers 10% of the boss bar.
       - `12 segments`: Each segment covers 8.33% of the boss bar.
       - `20 segments`: Each segment covers 5% of the boss bar.
   - `Sky Effect` (slot 24): `None` (default), `Create fog`, `Darken sky`, `Both`
   - `Bar Color` (slot 26): `Red`, `Purple` (default), `Pink`, `Blue`, `Green`, `Yellow`, `White`
- _Flags:_ rank: Mythic

### `ParticleSphere`
**Display Particle Sphere**
Displays a sphere of particles at a location to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Center location
   - `[2]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
- _Flags:_ rank: Noble

### `SetVelocity`
**Set Velocity**
Sets a player's movement velocity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VECTOR` — New velocity
- **Tags:**
   - `Add to Current Velocity` (slot 26): `True`, `False` (default)

### `Particle`
**Display Particle Effect**
Displays a particle effect to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` _(plural)_ — Effect
   - `[1]` `LOCATION` — Effect location

### `AddInvRow`
**Add Inventory Menu Row**
Adds a row to the bottom of a player's current inventory menu.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Items to display
- **Tags:**
   - `New Row Position` (slot 26): `Top row`, `Bottom row` (default)
- _Flags:_ rank: Emperor

### `NoNatRegen`
**Disable Natural Regeneration**
> ⚠ **LEGACY** → use `PlayerHeal` (Use the Player Heal Event, use Game Action: Cancel Event if the Heal Event Cause game value returns "natural".)
Prevents the player's health from regenerating naturally.

### `DisplayLightning`
**Display Lightning Bolt**
Displays a lightning strike effect to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Strike location

### `Damage`
**Damage**
Damages a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Damage to inflict — _❤ = 2 Health_
   - **OR**
   - `[1]` `TEXT` _(optional)_ — UUID of damager entity
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `SendAnimation`
**Send Player Animation**
> ⚠ **LEGACY** → use `` (Use the separate actions: Send Player Attack Animation, Send Hurt Animation, or Send Wake Up Animation.)
Makes a player perform an animation.
- **Tags:**
   - `Animation Type` (slot 26): `Hurt animation` (default), `Wake up (fade effect)`
- _Flags:_ rank: Noble

### `SetXPProg`
**Set XP Progress**
> ⚠ **LEGACY** → use `SetExp` (Set the Set Experience tag to Level Percentage.)
Sets the XP progress bar to a certain percentage.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Progress % (0-100)

### `SetInventory`
**Set Inventory Items**
Sets items in a player's upper inventory.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to set

### `TpSequence`
**Teleport Sequence**
> ⚠ **LEGACY** → use `Teleport` (Use Repeat: Repeat For Each in List and teleport while using a Control: Wait to set a delay.)
Teleports a player to multiple locations, with a delay between each teleport.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Locations to teleport to
   - `[1]` `NUMBER` _(optional)_ — Teleport delay (ticks, default = 60)
- _Flags:_ rank: Noble

### `Heal`
**Heal**
Restores a player's health.
- _Info:_ Triggers the Heal Event.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Amount to heal — _❤ = 2 Health_
   - **OR**
   - `[1]` `NONE` — Heals to full health)

### `SetSpawnPoint`
**Set Spawn Point**
Sets the location a player will spawn when they die and respawn.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — The new spawn location
   - **OR**
   - `[1]` `NONE` — Plot spawn)

### `SetInventoryKept`
**Set Inventory Kept**
Sets whether a player's inventory is kept after death.
- **Tags:**
   - `Inventory Kept` (slot 26): `Enable` (default), `Disable`

### `LaunchUp`
**Launch Up**
Launches a player up or down.
- _Info:_ A negative launch power will launch the player down.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Launch power
- **Tags:**
   - `Add to Current Velocity` (slot 26): `True` (default), `False`

### `GetTargetEntity`
- **Tags:**
   - `Ignore Blocks` (slot 26): `True`, `False` (default)

### `ForceFlight`
**Force Flight Mode**
> ⚠ **LEGACY** → use `SetFlying`
Forces a player to start or stop flying.
- **Tags:**
   - `Flight Mode` (slot 26): `Start Flight` (default), `Stop Flight`

### `LoadInv`
**Load Saved Inventory**
Loads a player's inventory.
- _Info:_ If no saved inventory exists, the player's inventory will be cleared.
- _Info:_ Inventories loaded with 'Load Saved Inventory' take several ticks to load. Use 'Control: Wait' if you need to check the inventory's contents after it is loaded.
- **Tags:**
   - `Code Flow` (slot 26): `Synchronous` (default), `Asynchronous`
       - `Synchronous`: Waits for the inventory to be loaded before continuing.
       - `Asynchronous`: Continues running code immediately.
- _Flags:_ rank: Noble

### `ChatStyle`
**Set Chat Style** · aliases: `ChatColor`
Sets a player's chat color or decoration.
- _Examples:_ "<#D4D4FF>"
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — New chat style
   - **OR**
   - `[1]` `NONE` — Resets chat style)
- _Flags:_ rank: Mythic

### `Kick`
**Kick**
Kicks a player from the plot.

### `ProjColl`
**Enable Projectile Collisions**
> ⚠ **LEGACY** → use `SetCollidable` (Set Collision tag to Enable.)
Allows projectiles to hit the player.

### `SetViewDistance`
**Set View Distance** · aliases: `SetFogDistance`
Sets how far in chunks the player can see the world.
- _Info:_ View distance is limited by the client's view distance setting.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Distance in chunks (2-7)
   - **OR**
   - `[1]` `NONE` — Reset to default)
- _Flags:_ rank: Noble

### `MiscAttribute`
**Set Miscellaneous Attribute**
Sets one of the player's miscellaneous attributes such as scale and burning time.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Scale` (default), `Luck`, `Oxygen bonus`, `Burning time`, `Camera distance`, `Name tag distance`
       - `Scale`: Base: 1.0 Range: 0.0625-16.0
       - `Luck`: Base: 0.0 Range: -1024.0-1024.0
       - `Oxygen bonus`: Base: 0.0 Range: 0.0-1024.0
       - `Burning time`: Base: 1.0 Range: 0.0-1024.0
       - `Camera distance`: Base: 4.0 Range: 0.0-32.0
       - `Name tag distance`: Base: 64.0 Range: 0.0-512.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `SpectateTarget`
**Spectate Target**
Makes a player spectate another player or entity.
- _Info:_ The player must be in spectator mode.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Target UUID
   - **OR**
   - `[1]` `NONE` — Stops spectating)
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.
- _Flags:_ rank: Mythic

### `HurtAnimation`
**Send Hurt Animation**
Makes a player perform a hurt animation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(optional)_ — Damage source — _Affects the direction of the screen tilt effect._
- _Flags:_ rank: Noble

### `SurvivalMode`
**Set to Survival Mode** · aliases: `GmSurvival`
Sets a player's game mode to Survival.

### `DisplayBellRing`
**Display Bell Ring**
Displays a bell ring animation at a location to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Block location
- **Tags:**
   - `Ring Direction` (slot 26): `North` (default), `South`, `West`, `East`
- _Flags:_ rank: Noble

### `SetStatus`
**Set Status**
Sets the player's game status, which is used to display information about what the player is doing in the game.
- _Info:_ A player's game status can be seen when using /locate.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Game Status — _Limited at 50 characters_
   - **OR**
   - `[1]` `NONE` — Clears status)
- _Flags:_ rank: Emperor

### `SetCursorItem`
**Set Cursor Item**
Sets the item on a player's cursor.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item to set
   - **OR**
   - `[1]` `NONE` — Clears cursor item)

### `SetAbsorption`
**Set Absorption Health**
> ⚠ **LEGACY** → use `" SetAbsorption "`
Sets a player's absorption health (golden hearts).
- _Info:_ The target does not need to have an absorption effect.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Absorption health — _❤ = 2 Health_

### `SetFireTicks`
**Set Fire Ticks**
Sets the remaining time a player is on fire for.
- _Info:_ 0 ticks mean the target is not on fire.
- _Info:_ This triggers the Player Combust event.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks

### `CombatAttribute`
**Set Combat Attribute**
Sets one of the player's combat-related attributes such as attack damage and attack speed.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Attack damage` (default), `Attack speed`, `Sweeping damage ratio`
       - `Attack damage`: Base: 2.0 Range: 0.0-2048.0
       - `Attack speed`: Base: 4.0 Range: 0.0-1024.0
       - `Sweeping damage ratio`: Base: 0.0 Range: 0.0-1.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `SetGamemode`
**Set Gamemode**
> ⚠ **LEGACY** → use `` (Use the separate game mode actions.)
Sets a player's gamemode.
- **Tags:**
   - `Flight Mode` (slot 25): `Respect Gamemode` (default), `Keep Original`
       - `Respect Gamemode`: Follows the flight access given to the gamemode.
       - `Keep Original`: Does not change the target's ability to fly before changing the gamemode.
   - `Gamemode` (slot 26): `Survival` (default), `Creative`, `Adventure`
       - `Creative`: Warning: a player can spawn any item they wish through a toolbar.  Click related events or game values may not work like they would in other gamemodes.

### `RemoveInvRow`
**Remove Inventory Menu Row**
Removes the given number of rows from the bottom of a player's current inventory menu.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Rows to remove
- **Tags:**
   - `Row to Remove` (slot 26): `Top row`, `Bottom row` (default)
- _Flags:_ rank: Emperor

### `WakeUpAnimation`
**Send Wake Up Animation**
Displays the wake up (fade in) animation to a player.
- _Flags:_ rank: Noble

### `DisableBlocks`
**Disallow Placing/Breaking Blocks**
Prevents a player from placing and breaking certain blocks.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` _(optional, plural)_ — Blocks to disallow — _If no blocks are given, disallows all blocks_

### `SetScoreObj`
**Set Scoreboard Objective Name**
Sets the objective name of the scoreboard sidebar.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Objective name

### `L SetHealth`
**Set Health**
> ⚠ **LEGACY** → use `SetHealth` (Use 'Set Current Health' for regular health. Use 'Set Absorption Health' for absorption.)
Sets a player's regular health, absorption health, or both.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — New health — _❤ = 2 Health_
- **Tags:**
   - `Heal Type` (slot 26): `Regular Health` (default), `Absorption Health`, `Combined Health`

### `ParticleEffect`
**Send Particle Effect**
> ⚠ **LEGACY** → use `Particle`
Sends one or more of a particle to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Particle to send
   - `[1]` `LOCATION` — Particle location
   - `[2]` `NUMBER` _(optional)_ — Particle count

### `ClearInv`
**Clear Inventory**
Empties a player's inventory.
- **Tags:**
   - `Clear Mode` (slot 26): `Entire inventory` (default), `Main inventory`, `Upper inventory`, `Hotbar`, `Armor`
   - `Clear Crafting and Cursor` (slot 25): `True` (default), `False`

### `SetFreezeTicks`
**Set Freeze Ticks**
Sets how long a player is frozen for.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks (0-140)
- **Tags:**
   - `Ticking Locked` (slot 26): `Enable`, `Disable` (default)
       - `Enable`: Player will stay frozen.
       - `Disable`: Player will slowly unfreeze over time.

### `SetGliding`
**Set Gliding**
Sets whether a player is gliding with elytra.
- _Info:_ The player must be wearing an elytra.
- **Tags:**
   - `Gliding` (slot 26): `Enable` (default), `Disable`

### `SetRotation`
**Set Rotation**
Changes a player's pitch and yaw.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Pitch (-90 to 90)
   - `[1]` `NUMBER` — Yaw (-180 to 180)

### `ClearItems`
**Clear Items**
Removes all of an item from a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to clear

### `OpenSign`
**Open Sign**
Opens a sign for a player. Also works with client-side signs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Sign location
- **Tags:**
   - `Sign Side` (slot 26): `Front` (default), `Back`
- _Flags:_ rank: Emperor

### `SetFlying`
**Set Flying**
Sets whether a player is flying.
- **Tags:**
   - `Flying` (slot 26): `Enable` (default), `Disable`

### `DisplayBlockOpen`
**Display Block Opened State**
Displays a container block at a location as being open or closed to a player.
- _Works with:_ Chest; Trapped Chest; Ender Chest; Shulker Box; Barrel
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Block location
- **Tags:**
   - `Container State` (slot 26): `Open` (default), `Closed`
- _Flags:_ rank: Noble

### `SetHandItem`
**Set Hand Item**
> ⚠ **LEGACY** → use `SetEquipment` (Set Equipment Slot tag to Main hand or Off hand.)
Sets the item in a player's main hand or off hand.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional)_ — Item to set
- **Tags:**
   - `Hand Slot` (slot 26): `Main Hand` (default), `Off Hand`

### `SendAdvancement`
**Send Advancement**
Displays a custom advancement popup to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Advancement name
   - `[1]` `ITEM` _(optional)_ — Advancement icon
- **Tags:**
   - `Toast Type` (slot 26): `Advancement` (default), `Goal`, `Challenge`
- _Flags:_ rank: Mythic

### `ClearChat`
**Clear Chat**
> ⚠ **LEGACY** → use `SendMessage` (Use <newline:20> on a Styled Text.)
Clears all messages on a player's chat window.

### `SetMenuItem`
**Set Inventory Menu Item**
Sets the item in a slot of a player's current inventory menu.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Slot
   - `[1]` `ITEM` _(optional)_ — Item to set
- _Flags:_ rank: Emperor

### `LaunchToward`
**Launch Toward Location**
Launches a player toward or away from a location.
- _Info:_ A negative launch power will launch the player away from the location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Launch destination
   - `[1]` `NUMBER` _(optional)_ — Launch power
- **Tags:**
   - `Add to Current Velocity` (slot 25): `True` (default), `False`
   - `Ignore Distance` (slot 26): `True`, `False` (default)

### `SetArmor`
**Set Armor Items**
Sets a player's armor items. Place the armor in slots 1-4 of the chest, with 1 being the helmet and 4 being the boots.
- _Info:_ Any block or item can render on the target's head if placed in the 'helmet' slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Armor to set

### `DisplayGateway`
**Display Gateway Beam**
Displays a vertical beam on an end gateway to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Gateway location
- **Tags:**
   - `Animation Type` (slot 26): `Initial beam` (default), `Periodic beam`
       - `Initial beam`: A magenta beam. Plays for 200 ticks.
       - `Periodic beam`: A purple beam. Plays for 40 ticks.

### `GiveSaturation`
**Give Saturation**
Adds saturation to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Saturation to give

### `NumEnvOption`
**Set Numeric Environment Options**
Sets a numeric environmental option on a player's view.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Environment Option` (slot 26): `Cloud fog end distance` (default), `Cloud height`, `Fog end distance`, `Fog start distance`, `Moon angle`, `Sky fog end distance`, `Sky light factor`, `Star angle`, `Star brightness`, `Sun angle`, `Water fog end distance`, `Water fog start distance`
       - `Cloud fog end distance`: Distance in blocks at which the cloud fog becomes fully opaque.  Range: DoubleRange[0.0,null]
       - `Cloud height`: Y coordinate of the clouds in the world.  Range: DoubleRange[null,null]
       - `Fog end distance`: Distance in blocks at which the fog becomes fully opaque.  Range: DoubleRange[0.0,null]
       - `Fog start distance`: Distance in blocks at which the fog starts to appear.  Range: DoubleRange[null,null]
       - `Moon angle`: Position of the moon in degrees from east to west, where 0 is directly up.  Range: DoubleRange[0.0,360.0]
       - `Sky fog end distance`: Distance in blocks at which the fog affecting the sky becomes fully opaque.  Range: DoubleRange[0.0,null]
       - `Sky light factor`: Brightness of the sky light.  Range: DoubleRange[0.0,1.0]
       - `Star angle`: Position of the stars in degrees from east to west, where 0 is directly up.  Range: DoubleRange[0.0,360.0]
       - `Star brightness`: Brightness of the stars, where 0.5 is the default brightness of the night stars.  Range: DoubleRange[null,null]
       - `Sun angle`: Position of the sun in degrees from east to west, where 0 is directly up.  Range: DoubleRange[0.0,360.0]
       - `Water fog end distance`: Distance in blocks at which the water fog becomes fully opaque.  Range: DoubleRange[0.0,null]
       - `Water fog start distance`: Distance in blocks at which the water fog starts to appear.  Range: DoubleRange[null,null]
- _Flags:_ rank: Noble

### `ColorEnvOption`
**Set Color Environment Options**
Sets an environmental color option on a player's view.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(optional)_ — Color hexadecimal — _Example: "#FF0000" (red)_
   - `[1]` `NUMBER` _(optional)_ — Opacity (%) — _Default = 100%_
   - **OR**
   - `[2]` `NONE` — Resets the environment option)
- **Tags:**
   - `Environment Option` (slot 26): `Ambient light color` (default), `Block light tint`, `Cloud color`, `Fog color`, `Night vision color`, `Sky color`, `Sky light color`, `Sunrise/sunset color`, `Water fog color`
       - `Ambient light color`: Tint and brightness of the world at light level 0.  Opacity allowed: No
       - `Block light tint`: Tint applied to light from blocks.  Opacity allowed: No
       - `Cloud color`: Color of clouds in the sky.  Opacity allowed: No  Opacity allowed: Yes
       - `Fog color`: Color of the fog that appears below the sky.  Opacity allowed: No
       - `Night vision color`: Tint and brightness of darkness when Night Vision is active.  Opacity allowed: No
       - `Sky color`: Color of the sky in the world.  Opacity allowed: No
       - `Sky light color`: Color of the light emitted by the sun and moon.  Opacity allowed: No
       - `Sunrise/sunset color`: Color and intensity of the sunrise and sunset.  Opacity allowed: No  Opacity allowed: Yes
       - `Water fog color`: Color of the fog that appears when underwater.  Opacity allowed: No
- _Flags:_ rank: Noble

### `DisplayEquipment`
**Display Equipment Change**
Displays equipment on an entity to a player. Equipment goes from slots 2-7 in order of Helmet, Chestplate, Leggings, Boots, Main Hand, Off Hand.
- _Info:_ Any block or item can render on the entity's head if placed in the 'helmet' slot.
- _Info:_ This equipment is client-side. The actual equipment is not changed.
- _Info:_ If the equipment slot is updated, it will be reset.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Entity UUID or name
   - **OR**
   - `[1]` `ITEM` _(plural)_ — Equipment
- _Flags:_ rank: Mythic

### `GiveExp`
**Give Experience**
Adds experience points or levels to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Experience to give
- **Tags:**
   - `Give Experience` (slot 26): `Points` (default), `Levels`, `Level Percentage`

### `FaceLocation`
**Face Location**
Rotates a player to look toward a location without teleporting them.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Location to face

### `ClearScoreboard`
**Clear Scoreboard**
Removes all scores from the scoreboard.

### `ActionBar`
**Show Action Bar Text**
Displays text directly above a player's hotbar.
- _Info:_ Multiple variables (of any type) will be merged into one message.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(plural)_ — Message to send
- **Tags:**
   - `Text Value Merging` (slot 26): `Add spaces`, `No spaces` (default)
   - `Inherit Styles` (slot 25): `True` (default), `False`
       - `True`: Text style of the last value will be inherited when merging multiple values.
       - `False`: Each value will have independent text style when merging multiple values.
- _Flags:_ rank: Mythic, requires tokens

### `SetChatTag`
**Set Chat Tag**
Sets a player's chat tag.
- _Info:_ Multiple text items will be merged into one chat tag.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(plural)_ — Chat tag
   - **OR**
   - `[1]` `NONE` — Resets chat tag)
- _Flags:_ rank: Mythic

### `ShiftWorldBorder`
**Shift World Border**
Changes a player's world border size if they have one active.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — New radius
   - `[1]` `NUMBER` _(optional)_ — Blocks per second
- _Flags:_ rank: Emperor

### `DisplaySignText`
**Display Sign Text**
Displays text on a sign to a player.
- _Info:_ Displaying a sign text will wipe any existing text on the sign for the player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Sign location
   - `[1]` `COMPONENT` _(optional, plural)_ — Text line(s)
- **Tags:**
   - `Sign Side` (slot 24): `Front` (default), `Back`
   - `Text Color` (slot 25): `White`, `Orange`, `Magenta`, `Light blue`, `Yellow`, `Lime`, `Pink`, `Gray`, `Light gray`, `Cyan`, `Purple`, `Blue`, `Brown`, `Green`, `Red`, `Black` (default)
   - `Glowing` (slot 26): `Enable`, `Disable` (default)
- _Flags:_ rank: Noble

### `ReloadInv`
**Reload Inventory**
Ensures a player's inventory matches what the server sees. Cleans up visual bugs.

### `SetSpeed`
**Set Movement Speed**
> ⚠ **LEGACY** → use `MovementAttribute` (Set walking/flying speed attributes.)
Sets a player's walking and/or flight speed.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Movement speed percentage (0% to 1000%)
- **Tags:**
   - `Speed Type` (slot 26): `Ground speed` (default), `Flight speed`, `Both`
- _Flags:_ rank: Noble

### `ExpandInv`
**Expand Inventory Menu**
Adds 3 more rows to a player's current inventory menu using the contents of the chest.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Items to display
- _Flags:_ rank: Emperor

### `LaunchProj`
**Launch Projectile**
Launches a projectile from a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PROJECTILE` — Projectile to launch
   - `[1]` `LOCATION` _(optional)_ — Launch point
   - `[2]` `COMPONENT` _(optional)_ — Projectile name
   - `[3]` `NUMBER` _(optional)_ — Speed
   - `[4]` `NUMBER` _(optional)_ — Inaccuracy — _Controls how much random motion is applied on launch Default = 1_

### `NoProjColl`
**Disable Projectile Collisions**
> ⚠ **LEGACY** → use `SetCollidable` (Set Collision tag to Disable.)
Prevents projectiles from hitting the player.

### `ShowDisguise`
**Show Own Disguise**
> ⚠ **LEGACY** → use `SetDisguiseVisible` (Set Disguise Visible tag to Enable.)
Shows the player's disguise on their screen.
- _Flags:_ rank: Overlord

### `ParticleCuboidA`
**Display Animated Particle Cuboid**
Displays an animated particle cuboid to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Corner 1
   - `[2]` `LOCATION` — Corner 2
   - `[3]` `NUMBER` _(optional)_ — Effect spacing — _Default = 0.5 blocks_
   - `[4]` `NUMBER` _(optional)_ — Animation duration — _Default = 40 ticks_
- **Tags:**
   - `Fill Type` (slot 26): `Wireframe` (default), `Hollow`, `Solid`
- _Flags:_ rank: Mythic

### `PlaySound`
**Play Sound**
Plays a sound for a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `SOUND` _(plural)_ — Sound to play
   - `[1]` `LOCATION` _(optional)_ — Playback location
- **Tags:**
   - `Sound Source` (slot 26): `Master` (default), `Music`, `Jukebox/Note Blocks`, `Weather`, `Blocks`, `Hostile Creatures`, `Friendly Creatures`, `Players`, `Ambient/Environment`, `Voice/Speech`, `UI`

### `SetCompass`
**Set Compass Target**
Sets the location compasses point to for a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — New Target
- _Flags:_ requires tokens

### `RngTeleport`
**Randomized Teleport**
> ⚠ **LEGACY** → use `RandomLoc` (Pick a random location with Set Variable: Random Location and teleport normally.)
Teleports a player to a random location in the chest.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Locations to choose from
- **Tags:**
   - `Keep Current Rotation` (slot 26): `True`, `False` (default)

### `MobDisguise`
**Disguise as Mob**
Disguises a player as a mob.
- _Info:_ If the disguise is too tall it will be scaled down for the target player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `SPAWN_EGG` — Mob to disguise as
   - `[1]` `COMPONENT` _(optional)_ — Display name
- _Flags:_ rank: Overlord

### `EnableBlocks`
**Allow Placing/Breaking Blocks**
Allows a player to place and break certain blocks.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` _(optional, plural)_ — Blocks to allow — _If no blocks are given, allows all blocks_

### `OpenBlockInv`
**Open Container Inventory**
Opens a container's inventory. Also works with crafting tables.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
- _Flags:_ rank: Emperor

### `LockDisgRotation`
**Lock Disguise Rotation**
Locks a disguise's pitch or yaw values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Pitch to lock to — _Default = 0_
   - `[1]` `NUMBER` _(optional)_ — Yaw to lock to — _Default = 0_
- **Tags:**
   - `Pitch` (slot 25): `Lock`, `Unlock`, `No Change` (default)
   - `Yaw` (slot 26): `Lock`, `Unlock`, `No Change` (default)
- _Flags:_ rank: Overlord

### `ParticleCircleA`
**Display Animated Particle Circle**
Displays an animated circle of particles to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Center location
   - `[2]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
   - `[3]` `NUMBER` _(optional)_ — Animation duration — _Default = 40 ticks_
- _Flags:_ rank: Mythic

### `RemoveBossBar`
**Remove Boss Bar** · aliases: `ClearBars`
> ⚠ **LEGACY** → use `" RemoveBossBar "`
Removes a boss health bar from a player's screen.
- **Tags:**
   - `Boss Bar Slot` (slot 26): `All boss bars` (default), `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`
       - `1`: Removes the boss bar in slot 1 and any boss bars below it.
       - `2`: Removes the boss bar in slot 2 and any boss bars below it.
       - `3`: Removes the boss bar in slot 3 and any boss bars below it.
       - `4`: Removes the boss bar in slot 4 and any boss bars below it.
       - `5`: Removes the boss bar in slot 5 and any boss bars below it.
       - `6`: Removes the boss bar in slot 6 and any boss bars below it.
       - `7`: Removes the boss bar in slot 7 and any boss bars below it.
       - `8`: Removes the boss bar in slot 8 and any boss bars below it.
       - `9`: Removes the boss bar in slot 9 and any boss bars below it.
- _Flags:_ rank: Mythic

### `SetEquipment`
**Set Equipment Item**
Sets the item in one of the equipment slots (armor and held items) of a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional)_ — Item to set
- **Tags:**
   - `Equipment Slot` (slot 26): `Main hand` (default), `Off hand`, `Head`, `Chest`, `Legs`, `Feet`

### `GiveRngItem`
**Give Random Item**
> ⚠ **LEGACY** → use `GiveItems` (Pick a random item with Set Variable: Set to Random Value and give it normally.)
Gives a player a random item or stack of items from the chest.
- _Info:_ Will give a different item to each targeted player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Items to pick from

### `SetDropsEnabled`
**Set Death Drops Enabled**
Sets whether a player drops their items when dead.
- **Tags:**
   - `Spawn Death Drops` (slot 26): `Enable` (default), `Disable`

### `SendToPlot`
**Send to Plot**
Sends a player to another plot.
- _Info:_ The player will be prompted to be sent to the specified plot. The prompt may be denied by the player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Plot handle or ID
- _Flags:_ rank: Emperor

### `RemovePotion`
**Remove Potion Effect** · aliases: `RemoveEffect`
Removes one or more potion effects from a player.
- _Info:_ Only the potion's type needs to match; amplifier and duration are disregarded.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `POTION` _(plural)_ — Effect(s) to remove

### `DisplayFracture`
**Display Block Fracture**
Displays a block fracture effect at a location to a player.
- _Info:_ Disappears after 20s.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Block(s) to fracture
   - `[1]` `NUMBER` _(optional)_ — Fracture level — _0-10 (default = 0)_
- **Tags:**
   - `Overwrite Previous Fracture` (slot 26): `True` (default), `False`

### `SetEntityHidden`
**Set Entity Hidden**
Sets if an entity is hidden to a target.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Entity UUIDs
- **Tags:**
   - `Hidden` (slot 26): `Enable` (default), `Disable`
   - `Ignore Formatting` (slot 25): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.
- _Flags:_ rank: Mythic

### `SetSidebar`
**Set Sidebar Visible**
Sets whether the scoreboard sidebar is visible to a player.
- **Tags:**
   - `Sidebar` (slot 26): `Enable` (default), `Disable`

### `AllowDrops`
**Allow Item Dropping**
> ⚠ **LEGACY** → use `DropItem` (Use Player Drop Item Event to detect dropping items, and cancel the event with Game Action: Cancel Event.)
Allows the player to drop items from their inventory.

### `DisplayHighlighter`
**Display Block Highlighter**
> ⚠ **LEGACY** → use `` (Block highlighters were removed from Minecraft in 1.21.9. Can be somewhat replicated by using colored text display backgrounds for each block face.)
Highlights a specific block for a player. The highlight is overlaid on a block, showing text on top if provided.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Block location
   - `[1]` `TEXT` _(optional)_ — Color hexadecimal — _Example: "#FF0000" (red)_
   - `[2]` `TEXT` _(optional)_ — Name
   - `[3]` `NUMBER` _(optional)_ — Opacity in percentage
   - `[4]` `NUMBER` _(optional)_ — Duration (milliseconds)
- _Flags:_ rank: Noble

### `Vibration`
**Display Vibration Effect**
> ⚠ **LEGACY** → use `Particle` (Use the Vibration particle.)
Displays a Sculk Sensor vibration to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Origin location
   - `[1]` `LOCATION` — Target location
   - `[2]` `NUMBER` _(optional)_ — Arrival time — _Default = 20 ticks_

### `SetSlot`
**Set Hotbar Slot**
Sets a player's selected hotbar slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — New slot — _1 (left) to 9 (right)_

### `ParticleRay`
**Display Particle Ray**
Displays a ray of particles to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Ray location
   - `[2]` `VECTOR` — Ray vector
   - `[3]` `NUMBER` _(optional)_ — Effect spacing — _Default = 0.5 blocks_

### `ParticleCuboid`
**Display Particle Cuboid**
Displays a particle cuboid as a solid, hollow or wireframe shape to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Corner 1
   - `[2]` `LOCATION` — Corner 2
   - `[3]` `NUMBER` _(optional)_ — Effect spacing — _Default = 0.5 blocks_
- **Tags:**
   - `Fill Type` (slot 26): `Wireframe` (default), `Hollow`, `Solid`
- _Flags:_ rank: Noble

### `SendMessageSeq`
**Send Message Sequence** · aliases: `SendDialogue`
Sends a series of messages in chat to a player, with a delay after each message.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(plural)_ — Messages to send
   - `[1]` `NUMBER` _(optional)_ — Message delay ticks — _Default = 60_
- **Tags:**
   - `Alignment Mode` (slot 26): `Regular` (default), `Centered`
- _Flags:_ rank: Noble, requires tokens

### `SetNamePrefix`
**Set Name Prefix / Suffix**
Sets the prefix or suffix for the player's name.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(optional)_ — Prefix/suffix text
- **Tags:**
   - `Text Type` (slot 26): `Prefix` (default), `Suffix`
- _Flags:_ rank: Noble

### `ClearDispBlock`
**Clear Display Block**
Displays the real block at a location to a player, effectively removing any client-side blocks.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Block location, or start of region
   - `[1]` `LOCATION` _(optional)_ — End of region — _Region size limit: 500 blocks_
- _Flags:_ rank: Noble

### `SetRainLevel`
**Set Rain Level**
Sets the heaviness of rain and storm visible to a player.
- _Info:_ The player's weather must be set to Downfall to take effect.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Rain level (%)
   - `[1]` `NUMBER` — Storm level (%)

### `Undisguise`
**Undisguise**
Removes a player's disguise.
- _Flags:_ rank: Overlord

### `ParticleSpiralA`
**Display Animated Particle Spiral**
Displays an animated spiral of particles to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Base location
   - `[2]` `NUMBER` _(optional)_ — Length — _Default = 10 blocks_
   - `[3]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
   - `[4]` `NUMBER` _(optional)_ — Effect count — _Default = 50_
   - `[5]` `NUMBER` _(optional)_ — Rotations — _Default = 4_
   - `[6]` `NUMBER` _(optional)_ — Animation duration — _Default = 40 ticks_
- _Flags:_ rank: Mythic

### `InstantRespawn`
**Set Instant Respawn**
Sets if a player is instantly respawned upon dying.
- _Info:_ Also respawns a player if they are dead.
- **Tags:**
   - `Instant Respawn` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `SetScore`
**Set Scoreboard Score**
Sets a score on the scoreboard.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Score name
   - `[1]` `NUMBER` _(optional)_ — Score value — _Default = 0_

### `SetNameColor`
**Set Name Color**
Sets the color a player's name tag appears in.
- **Tags:**
   - `Name Color` (slot 26): `Black` (default), `Dark blue`, `Dark green`, `Dark aqua`, `Dark red`, `Dark purple`, `Gold`, `Gray`, `Dark gray`, `Blue`, `Green`, `Aqua`, `Red`, `Light purple`, `Yellow`, `White`, `None`
- _Flags:_ rank: Noble

### `ReachAttribute`
**Set Reach Attribute**
Sets one of the player's reach-related attributes such as block and entity interaction ranges.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Block interaction range` (default), `Entity interaction range`
       - `Block interaction range`: Base: 4.5 Range: 0.0-64.0
       - `Entity interaction range`: Base: 3.0 Range: 0.0-64.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `SetAtkSpeed`
**Set Base Attack Speed**
> ⚠ **LEGACY** → use `CombatAttribute` (Set the attack speed attribute.)
Sets a player's attack speed.
- _Info:_ The target's attack speed is equal to the number of times they can attack per second.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Attack speed — _Default = 4_
- _Flags:_ rank: Mythic

### `OpenDialog`
**Open Dialog**
Opens a dialog for a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `DIALOG` — Dialog to open

### `DisablePvp`
**Disable PVP**
> ⚠ **LEGACY** → use `SetAllowPVP` (Set PVP tag to Disable.)
Prevents the player from damaging other players.

### `SetTickRate`
**Set Tick Rate**
Changes the tick rate of a player.
- _Info:_ Lower tick rate makes the player move in slower speed, and makes various animations slower.
- _Info:_ Tick rate of 0 does not affect the player's movement and entities will appear to be broken.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Ticks per second (0-20) — _Default = 20_
- _Flags:_ rank: Noble

### `PlayEntitySound`
**Play Sound from Entity**
Plays a sound that follows a moving entity or player.
- _Info:_ Some sounds are not supported to be played from an entity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `SOUND` _(plural)_ — Sound to play
   - **OR**
   - `[1]` `TEXT` _(plural)_ — Target UUID
- **Tags:**
   - `Sound Source` (slot 26): `Master` (default), `Music`, `Jukebox/Note Blocks`, `Weather`, `Blocks`, `Hostile Creatures`, `Friendly Creatures`, `Players`, `Ambient/Environment`, `Voice/Speech`, `UI`
   - `Ignore Formatting` (slot 25): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `ReplaceProj`
**Replace Projectile**
> ⚠ **LEGACY** → use `SetEventProj` (Use inside Player Shoot Bow Event.)
Replaces the projectile fired in the Shoot Bow Event.
- _Works with:_ Player Shoot Bow Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PROJECTILE` _(optional)_ — Projectile to launch
   - `[1]` `TEXT` _(optional)_ — Projectile name
   - `[2]` `PARTICLE` _(optional)_ — Launch trail

### `SetExp`
**Set Experience**
Sets a player's experience level, points or progress.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Experience to set
- **Tags:**
   - `Set Experience` (slot 26): `Points`, `Level` (default), `Level Percentage`

### `MiningAttribute`
**Set Mining Attribute**
Sets one of the player's mining-related attributes such as break speed and mining efficiency.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Block break speed` (default), `Mining efficiency`, `Submerged mining speed`
       - `Block break speed`: Base: 1.0 Range: 0.0-1024.0
       - `Mining efficiency`: Base: 0.0 Range: 0.0-1024.0
       - `Submerged mining speed`: Base: 0.2 Range: 0.0-20.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `KBAttribute`
**Set Knockback Attribute**
Sets one of the player's knockback-related attributes such as knockback resistance.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Knockback resistance` (default), `Explosion knockback resistance`
       - `Knockback resistance`: Base: 0.0 Range: 0.0-1.0
       - `Explosion knockback resistance`: Base: 0.0 Range: 0.0-1.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `MovementAttribute`
**Set Movement Attribute**
Sets one of the player's movement-related attributes, such as walking speed and jump height.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Walking speed` (default), `Flying speed`, `Jump strength`, `Sneaking speed`, `Step height`, `Movement efficiency`, `Water movement efficiency`, `Air drag modifier`, `Bounciness`, `Friction modifier`
       - `Walking speed`: Base: 0.2 Range: -1.0-1.0
       - `Flying speed`: Base: 0.1 Range: -1.0-1.0
       - `Jump strength`: Base: 0.42 Range: 0.0-32.0
       - `Sneaking speed`: Base: 0.3 Range: 0.0-1.0
       - `Step height`: Base: 0.6 Range: 0.0-10.0
       - `Movement efficiency`: Base: 0.0 Range: 0.0-1.0
       - `Water movement efficiency`: Base: 0.0 Range: 0.0-1.0
       - `Air drag modifier`: Base: 1.0 Range: 0.0-2048.0
       - `Bounciness`: Base: 0.0 Range: 0.0-1.0
       - `Friction modifier`: Base: 1.0 Range: 0.0-2048.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `ParticleSpiral`
**Display Particle Spiral**
Displays a spiral of particles at a location to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Base location
   - `[2]` `NUMBER` _(optional)_ — Length — _Default = 10 blocks_
   - `[3]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
   - `[4]` `NUMBER` _(optional)_ — Effect count — _Default = 50_
   - `[5]` `NUMBER` _(optional)_ — Rotations — _Default = 4_
- _Flags:_ rank: Noble

### `FallingAttribute`
**Set Falling Attribute**
Sets one of the player's falling-related attributes, such as gravity and fall damage multiplier.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Gravity` (default), `Safe fall distance`, `Fall damage multiplier`
       - `Gravity`: Base: 0.08 Range: -1.0-1.0
       - `Safe fall distance`: Base: 3.0 Range: -1024.0-1024.0
       - `Fall damage multiplier`: Base: 1.0 Range: 0.0-100.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `SetAllowFlight`
**Set Allow Flight**
Sets whether a player is able to enter and exit flight mode by double tapping jump.
- **Tags:**
   - `Allow Flight` (slot 26): `Enable` (default), `Disable`

### `SetMaxHealth`
**Set Maximum Health**
Sets a player's maximum health.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Maximum health — _❤ = 2 Health_
- **Tags:**
   - `Heal Player to Max Health` (slot 26): `True`, `False` (default)

### `" RemoveBossBar "`
**Remove Boss Bar**
Removes a boss health bar from a player's screen.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Boss bar position
   - **OR**
   - `[1]` `NONE` — Removes all boss bars)
- _Flags:_ rank: Mythic

### `AdventureMode`
**Set to Adventure Mode** · aliases: `GmAdventure`
Sets a player's game mode to Adventure.

### `SpectatorMode`
**Set to Spectator Mode**
Sets a player's game mode to Spectator.
- _Flags:_ rank: Mythic

### `DispHeadTexture`
**Display Head Texture**
Changes a head's texture at a location for a player.
- _Info:_ The texture is client-side but will NOT revert if interacted with.
- _Info:_ Requires a player head to already be placed.
- _Info:_ Works on client-side player heads.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Head location
   - **OR**
   - `[1]` `ITEM` — Player Head
   - **OR**
   - `[2]` `TEXT` — Head owner — _Player name, UUID, or texture_
- _Flags:_ rank: Noble

### `ClearPotions`
**Clear Potion Effects** · aliases: `ClearEffects`
Removes all active potion effects from a player.

### `SetTabListInfo`
**Set Player List Info**
Sets the text to be displayed above or below a player's player list shown when pressing Tab.
- _Info:_ Multiple variables (of any type) will be merged into one message.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(optional, plural)_ — Header/footer text
- **Tags:**
   - `Player List Field` (slot 26): `Header` (default), `Footer`
   - `Text Value Merging` (slot 25): `Add spaces`, `No spaces` (default)
   - `Inherit Styles` (slot 24): `True` (default), `False`
       - `True`: Text style of the last value will be inherited when merging multiple values.
       - `False`: Each value will have independent text style when merging multiple values.
- _Flags:_ rank: Noble

### `EnablePvp`
**Enable PVP**
> ⚠ **LEGACY** → use `SetAllowPVP` (Set PVP tag to Enable.)
Allows the player to damage other players.

### `PromptPurchase`
**Prompt Purchase**
Prompts the player to purchase a plot product.
- _Info:_ Plot products must be approved before they can be sold.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Product ID

### `HideDisguise`
**Hide Own Disguise**
> ⚠ **LEGACY** → use `SetDisguiseVisible` (Set Disguise Visible tag to Disable.)
Hides the player's disguise on their screen.
- _Flags:_ rank: Overlord

### `ScoreLineFormat`
**Set Scoreboard Line Number Format**
Sets the number format of a single line in the player's scoreboard.
- _Info:_ Put an appropriate value in the chest depending on the number format type.
- _Info:_ Line number format overrides the default number format of the whole scoreboard.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Score name
   - **OR**
   - `[1]` `COMPONENT` — Content or style
   - **OR**
   - `[2]` `NONE` — Uses the default number format)
- **Tags:**
   - `Number Format` (slot 26): `Fixed` (default), `Styled`, `Blank`, `Reset`
       - `Fixed`: Displays a constant text.
       - `Styled`: Displays a score number with the specified text style.
       - `Blank`: Does not display anything.
       - `Reset`: Uses the default number format.

### `" SetBossBar "`
**Set Boss Bar**
Creates or modifies a custom boss health bar at the top of a player's screen.
- _Info:_ Boss bar positions are relative; unused positions are not shown.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(optional)_ — Title
   - `[1]` `NUMBER` _(optional)_ — Current health
   - `[2]` `NUMBER` _(optional)_ — Maximum health — _Default = 100_
   - `[3]` `NUMBER` _(optional)_ — Boss bar position — _Default = 1 (top)_
- **Tags:**
   - `Bar Style` (slot 25): `Solid` (default), `6 segments`, `10 segments`, `12 segments`, `20 segments`
       - `Solid`: The boss bar has no segments.
       - `6 segments`: Each segment covers 16.66% of the boss bar.
       - `10 segments`: Each segment covers 10% of the boss bar.
       - `12 segments`: Each segment covers 8.33% of the boss bar.
       - `20 segments`: Each segment covers 5% of the boss bar.
   - `Sky Effect` (slot 24): `None` (default), `Create fog`, `Darken sky`, `Both`
   - `Bar Color` (slot 26): `Red`, `Purple` (default), `Pink`, `Blue`, `Green`, `Yellow`, `White`
- _Flags:_ rank: Mythic

### `SetSkin`
**Set Skin**
Sets the player's skin.
- _Info:_ To get a skin use /item head <player name> OR /item mshead
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Player head
   - **OR**
   - `[1]` `NONE` — Clears Skin)
- _Flags:_ rank: Overlord

### `SpectatorCollision`
**Set Spectator Collision**
Toggles whether a player collides with blocks in spectator mode.
- **Tags:**
   - `Spectator Collision` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Mythic

### `SetNameVisible`
**Set Name Tag Visible**
Sets whether a player's name tag is visible.
- **Tags:**
   - `Name Tag Visible` (slot 26): `Enable`, `Disable` (default)

### `SetInvulTicks`
**Set Invulnerability Ticks**
Sets the currently remaining ticks until a player can next be hurt.
- _Info:_ This value is set to 10 upon taking damage.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks

### `EnableFlight`
**Enable Flight**
> ⚠ **LEGACY** → use `SetAllowFlight` (Set Allow Flight tag to Enable.)
Allows the player to fly.

### `Mimic`
**Mimic Entity**
Disguises a player as another currently existing entity or player.
- _Info:_ The disguise will match the target exactly, including things like color or equipment.
- _Info:_ If the disguise is too tall it will be scaled down for the target player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — UUID of target to disguise as
- **Tags:**
   - `Remove Original Entity` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Overlord

### `SetStingsStuck`
**Set Bee Stings Stuck**
Sets the amount of bee stings sticking out of a player's character model.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Sting Count — _Default = 0_

### `RemoveScore`
**Remove Scoreboard Score**
Removes a score from the scoreboard.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Score name

### `DisallowDrops`
**Disallow Item Dropping**
> ⚠ **LEGACY** → use `DropItem` (Use Player Drop Item Event to detect dropping items, and cancel the event with Game Action: Cancel Event.)
Prevents the player from dropping items from their inventory.

### `SetExhaustion`
**Set Exhaustion Level**
Sets a player's exhaustion level.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Exhaustion level (0-4)

### `ParticleCircle`
**Display Particle Circle**
Displays a circle of particles to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Center location
   - `[2]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
- _Flags:_ rank: Noble

### `DisplayBlock`
**Display Block**
Displays a block at a location to a player.
- _Info:_ This block is client-side. The actual block is not changed.
- _Info:_ Interacting with a client-side block causes it to update.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Block to display
   - `[1]` `LOCATION` — Block location, or start of region
   - `[2]` `LOCATION` _(optional)_ — End of region — _Region size limit: 250,000 blocks_
   - `[3]` `BLOCK_TAG` _(optional, plural)_ — Block data — _Example: "facing=up", "half=top"_
- _Flags:_ rank: Noble

### `RideEntity`
**Ride Entity**
Mounts a player on top of another player or entity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Target UUID
   - **OR**
   - `[1]` `NONE` — Dismounts player)
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.
- _Flags:_ rank: Noble

### `WeatherRain`
**Set Weather: Downfall**
> ⚠ **LEGACY** → use `SetPlayerWeather` (Set Weather tag to Downfall.)
Sets the weather to downfall (rain) for the target only.

### `RmWorldBorder`
**Remove World Border**
Removes a player's world border.
- _Flags:_ rank: Emperor

### `ResourcePack`
**Send Resource Pack**
Send a resource pack to a player.
- _Info:_ Resource packs may only be sent once per player.
- _Info:_ Resource packs are limited to 10 MiB in size, but can be upgraded to 40 MiB with /plot addons.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Resource Pack URL — _Must link directly to a .zip file._
- _Flags:_ rank: Overlord

### `" SetInvName "`
**Set Inventory Menu Name**
Renames a player's current inventory menu.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Inventory name
- **Tags:**
   - `Alignment Mode` (slot 26): `Regular` (default), `Centered`
- _Flags:_ rank: Emperor

### `GiveExhaustion`
**Give Exhaustion**
Adds exhaustion to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Exhaustion to give

### `Teleport`
**Teleport**
Teleports a player to a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — New position
- **Tags:**
   - `Keep Current Rotation` (slot 26): `True`, `False` (default)
   - `Keep Velocity` (slot 25): `True`, `False` (default)

### `SetAllowPVP`
**Set PvP Allowed**
Sets whether a player can hurt or be hurt by other players.
- **Tags:**
   - `PVP` (slot 26): `Enable`, `Disable` (default)

### `DisableFlight`
**Disable Flight**
> ⚠ **LEGACY** → use `SetAllowFlight` (Set Allow Flight tag to Disable.)
Prevents the player from flying.

### `SetVisualFire`
**Set Visual Fire**
Sets whether a player should appear on fire.
- _Info:_ The affected player's fire ticks won't change and won't take any damage.
- **Tags:**
   - `On Fire` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `SetDisguiseVisible`
**Set Own Disguise Visibility**
Sets a player's ability to see their own disguise. It is recommended that it is almost always hidden.
- **Tags:**
   - `Disguise Visible` (slot 26): `Enable`, `Disable` (default)
- _Flags:_ rank: Overlord

### `DisguiseShiftVert`
**Shift Disguise Vertically**
Shifts the disguise of a player up or down relative to the player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Y-Offset
- _Flags:_ rank: Overlord

### `SetArrowsStuck`
**Set Arrows Stuck**
Sets the amount of arrows sticking out of a player's character model.
- _Info:_ These arrows cannot be used or picked up.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Arrow Count — _Default = 0_

### `GetItemCooldown`
**Get Item Cooldown**
Gets the remaining cooldown on an item type.
- _Info:_ The cooldown applies to all items of the given type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item type to check
- **Returns:** `NUMBER` Cooldown left in ticks

### `SetItems`
**Set Items**
> ⚠ **LEGACY** → use `SetInventory`
Changes a player's inventory according to the items in the parameter chest.
- _Info:_ The top chest row is the hotbar. The other rows fill the inventory from the top down.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to give

### `KeepInv`
**Keep Items on Death**
> ⚠ **LEGACY** → use `SetInventoryKept` (Set Inventory Kept tag to Enable.)
When this code block is executed, the player will keep the contents of their inventory when they die.

### `ReplaceItems`
**Replace Items** · aliases: `ReplaceItem`
Replaces items in a player's inventory with the given item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Item(s) to replace
   - `[1]` `ITEM` — Item to replace with
   - `[2]` `NUMBER` _(optional)_ — Amount of items to replace

### `SendMessage`
**Send Message**
Sends a chat message to a player.
- _Info:_ Multiple values (of any type) will be merged together.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(optional, plural)_ — Message to send
- **Tags:**
   - `Alignment Mode` (slot 26): `Regular` (default), `Centered`
   - `Text Value Merging` (slot 25): `Add spaces` (default), `No spaces`
   - `Inherit Styles` (slot 24): `True` (default), `False`
       - `True`: Text style of the last value will be inherited when merging multiple values.
       - `False`: Each value will have independent text style when merging multiple values.

### `SetSlotItem`
**Set Item in Slot**
Sets the item in a slot of a player's inventory.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional)_ — Item to set
   - `[1]` `NUMBER` — Slot to set — _1-9 Hotbar 10-36 Inventory (Top to bottom) 37-40 Armor (foot to head) 41 Offhand_

### `PlaySoundSeq`
**Play Sound Sequence**
Plays a sequence of sounds to a player, with a delay between each sound.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `SOUND` _(plural)_ — Sounds to play
   - `[1]` `NUMBER` _(optional)_ — Sound delay (ticks, default = 60)
   - `[2]` `LOCATION` _(optional)_ — Playback location
- **Tags:**
   - `Sound Source` (slot 26): `Master` (default), `Music`, `Jukebox/Note Blocks`, `Weather`, `Blocks`, `Hostile Creatures`, `Friendly Creatures`, `Players`, `Ambient/Environment`, `Voice/Speech`, `UI`

### `OpenTradeMenu`
**Open Villager Trade Menu**
Opens the trading menu of a villager.
- _Info:_ Showing a villagers trading menu to a player will close it for any other player currently in the menu.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Villager UUID
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `ParticleLineA`
**Display Animated Particle Line**
Displays an animated line of particles between two locations to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Start location
   - `[2]` `LOCATION` — End location
   - `[3]` `NUMBER` _(optional)_ — Effect spacing — _Default = 0.5 blocks_
   - `[4]` `NUMBER` _(optional)_ — Animation duration — _Default = 40 ticks_
- _Flags:_ rank: Mythic

### `Respawn`
**Respawn**
> ⚠ **LEGACY** → use `Respawn`
Respawns a player if they are dead.
- _Flags:_ rank: Mythic

### `SetInvName`
**Set Inventory Menu Name**
> ⚠ **LEGACY** → use `" SetInvName "`
Renames a player's current inventory menu.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(plural)_ — Inventory name
- _Flags:_ rank: Emperor

### `SetItemCooldown`
**Set Item Cooldown**
Applies a cooldown visual effect to an item type.
- _Info:_ The cooldown applies to all items of the given type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item type to affect
   - `[1]` `NUMBER` — Cooldown in ticks

### `SetPlayerWeather`
**Set Player Weather**
Sets the type of weather visible to a player.
- **Tags:**
   - `Weather` (slot 26): `Clear`, `Downfall` (default)
       - `Clear`: Displays sunny weather.
       - `Downfall`: Displays rain, or snow (in cold biomes).

### `SendHover`
**Send Message with Hover**
> ⚠ **LEGACY** → use `` (SendMessage Use a Styled Text with a hover event e.g. <hover:show_text:'Hi!'>Hover!)
Sends a message to a player. When the player moves over it with their cursor, a second message appears.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Message to send in chat
   - `[1]` `TEXT` — Message to see on hover
- _Flags:_ rank: Mythic

### `SetShoulder`
**Set Visual Shoulder Parrot**
Displays a parrot on the targets' shoulders.
- _Info:_ Parrot will not dismount automatically.
- **Tags:**
   - `Shoulder` (slot 26): `Left` (default), `Right`
   - `Type` (slot 25): `Remove` (default), `Red`, `Blue`, `Green`, `Cyan`, `Gray`
- _Flags:_ rank: Noble

### `SetAirTicks`
**Set Remaining Air**
Sets a player's remaining breath ticks.
- _Info:_ Each bubble is equal to 30 air ticks.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Breath ticks

### `DisplayPickup`
**Display Pick Up Animation**
Displays a pickup animation of one entity being collected by another entity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Entity UUID
   - **OR**
   - `[1]` `TEXT` — Collector UUID
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.
- _Flags:_ rank: Mythic

### `SetWorldBorder`
**Set World Border**
Creates a world border only visible to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Center position
   - `[1]` `NUMBER` — Radius in blocks
   - `[2]` `NUMBER` _(optional)_ — Warning distance — _Default = 0_
- _Flags:_ rank: Emperor

### `SetPlayerTime`
**Set Player Time**
Sets the time of day visible to a player.
- _Info:_ Add 24000 ticks to add one moon phase.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Daylight ticks — _Day: 1000 Noon: 6000 Night: 13000 Midnight: 18000_
   - **OR**
   - `[1]` `NONE` — Resets player time)

### `GiveFood`
**Give Food**
Adds food to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Food to give

### `NatRegen`
**Enable Natural Regeneration**
> ⚠ **LEGACY** → use `PlayerHeal` (Use the Player Heal Event, make sure it is not canceled.)
Allows the player's health to regenerate naturally.

### `GivePotion`
**Give Potion Effect** · aliases: `GiveEffect`
Gives one or more potion effects to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `POTION` _(plural)_ — Effect(s) to give
- **Tags:**
   - `Show Icon` (slot 24): `True` (default), `False`
       - `True`: Shows the effect's icon.
       - `False`: Hides the effect's icon. It can still be seen in the inventory.
   - `Overwrite Effect` (slot 25): `True` (default), `False`
       - `True`: Effects of the same type will always be replaced.
       - `False`: Weaker effects of the same type will be hidden.
   - `Effect Particles` (slot 26): `Regular` (default), `Ambient`, `None`
       - `Ambient`: Ambient particles are translucent and make the icon border blue.

### `RemoveItems`
**Remove Items**
Removes items from a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to remove

### `BoostElytra`
**Boost Elytra**
Boosts a player's elytra using a firework rocket.
- _Info:_ The player must be gliding with an elytra.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Firework

### `SaveInv`
**Save Current Inventory**
Saves a player's inventory. It can be loaded later with 'Load Saved Inventory'.
- _Flags:_ rank: Noble

### `OpenBook`
**Open Book**
Opens a written book menu for a player.
- _Info:_ Opened book and quills cannot be edited and cannot display some special formats.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Book item
- _Flags:_ rank: Emperor

### `SetHealth`
**Set Current Health**
Sets a player's current health.
- _Info:_ Does not trigger a heal or damage event.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Current health — _❤ = 2 Health_

### `BlockDisguise`
**Disguise as Block**
Disguises a player as a block.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Block to disguise as
   - `[1]` `COMPONENT` _(optional)_ — Display name
- _Flags:_ rank: Overlord

### `RollbackBlocks`
**Roll Back Block Changes**
Undoes the interactions with blocks by a player.
- _Info:_ The rollback time argument specifies how far back in time block changes should be reverted.
- _Info:_ Please note that the rollback time argument is in SECONDS!
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Rollback time
- _Flags:_ rank: Overlord

### `NoDeathDrops`
**Disable Death Drops**
> ⚠ **LEGACY** → use `SetDropsEnabled` (Set Spawn Death Drops tag to Disable.)
When this code block is executed, the player will no longer drop the contents of their inventory when they die.

### `WalkSpeed`
**Set Walk Speed**
> ⚠ **LEGACY** → use `MovementAttribute` (Set walking speed attribute.)
Sets a player's walk speed.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — % of normal walk speed (0 to 500)
- _Flags:_ rank: Noble

### `ClearHighlighters`
**Clear Block Highlighters**
> ⚠ **LEGACY** → use `` (Block highlighters were removed from Minecraft in 1.21.9. Can be somewhat replicated by using colored text display backgrounds for each block face.)
Clears all block highlighters displayed to a player.
- _Flags:_ rank: Noble

### `SetCollidable`
**Set Collidable**
Sets whether a player is able to collide with other entities.
- _Info:_ Useful in conjunction with projectiles.
- **Tags:**
   - `Collision` (slot 26): `Enable`, `Disable` (default)

### `LaunchFwd`
**Launch Forward**
Launches a player forward or backward.
- _Info:_ A negative launch power will launch the player backward.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Launch power
- **Tags:**
   - `Add to Current Velocity` (slot 25): `True` (default), `False`
   - `Launch Axis` (slot 26): `Pitch and Yaw` (default), `Yaw Only`

### `SetFallDistance`
**Set Fall Distance**
Sets a player's fall distance, affecting fall damage upon landing.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Fall distance (blocks)

### `CreativeMode`
**Set to Creative Mode**
Sets a player's game mode to Creative.
- _Flags:_ requires tokens

### `AttackAnimation`
**Send Player Attack Animation**
Makes a player perform an attack animation.
- **Tags:**
   - `Animation Arm` (slot 26): `Swing main arm` (default), `Swing off arm`
- _Flags:_ rank: Noble

### `DisplayHologram`
**Display Hologram**
Displays a floating name tag at a location to a player.
- _Info:_ If the player already has a hologram displayed at the location, its text is replaced.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Display location
   - **OR**
   - `[1]` `COMPONENT` — Text to display
   - **OR**
   - `[2]` `NONE` — Removes hologram)
- _Flags:_ rank: Emperor

### `" SetAbsorption "`
**Set Absorption Health**
Sets a player's absorption health (golden hearts).
- _Info:_ The target does not need to have an absorption effect.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Absorption health — _❤ = 2 Health_

### `DeathDrops`
**Enable Death Drops**
> ⚠ **LEGACY** → use `SetDropsEnabled` (Set Spawn Death Drops tag to Enable.)
When this code block is executed, the player will drop the contents of their inventory when they die.

### `ShowInv`
**Show Inventory Menu**
Opens a custom inventory for a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Items to display
- _Flags:_ rank: Emperor

### `SetFoodLevel`
**Set Food Level**
Sets a player's food hunger level.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Food level (1-20)

### `PlayerDisguise`
**Disguise as Player**
Disguises a player as another player.
- _Info:_ To get a skin use /item head <player name> OR /item mshead
- _Info:_ If the disguise is too tall it will be scaled down for the target player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Player name to disguise as
   - `[1]` `ITEM` _(optional)_ — Display skin
- _Flags:_ rank: Overlord

### `SetSaturation`
**Set Saturation Level**
Sets a player's saturation level.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Saturation level (1-20)

### `WeatherClear`
**Set Weather: Clear**
> ⚠ **LEGACY** → use `SetPlayerWeather` (Set Weather tag to Clear.)
Sets the weather to clear weather for the target only.

### `SendTitle`
**Show Title Text** · aliases: `Title`
Displays text in the center of a player's screen.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Title text
   - `[1]` `COMPONENT` _(optional)_ — Subtitle text
   - `[2]` `NUMBER` _(optional)_ — Title duration — _Default = 60 ticks_
   - `[3]` `NUMBER` _(optional)_ — Fade in length — _Default = 20 ticks_
   - `[4]` `NUMBER` _(optional)_ — Fade out length — _Default = 20 ticks_
- _Flags:_ rank: Emperor, requires tokens

### `ScoreDefFormat`
**Set Scoreboard Default Number Format**
Sets the default number format of the player's scoreboard.
- _Info:_ Put an appropriate value in the chest depending on the number format type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Content or style
   - **OR**
   - `[1]` `NONE` — Uses the default number format)
- **Tags:**
   - `Number Format` (slot 26): `Fixed` (default), `Styled`, `Blank`, `Reset`
       - `Fixed`: Displays a constant text.
       - `Styled`: Displays a score number with the specified text style.
       - `Blank`: Does not display anything.
       - `Reset`: Uses the default number format.

### `StopSound`
**Stop Sounds**
Stops all or specific sounds for a player.
- _Info:_ Stopping sounds from the Master source stops every sound effect.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `SOUND` _(optional, plural)_ — Sounds to stop
- **Tags:**
   - `Sound Source` (slot 26): `Master` (default), `Music`, `Jukebox/Note Blocks`, `Weather`, `Blocks`, `Hostile Creatures`, `Friendly Creatures`, `Players`, `Ambient/Environment`, `Voice/Speech`, `UI`

### `HealthAttribute`
**Set Health Attribute**
Sets one of the player's health-related attributes such as max health and armor defense points.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Maximum health` (default), `Maximum absorption health`, `Armor`, `Armor toughness`
       - `Maximum health`: Base: 20.0 Range: 1.0-1024.0
       - `Maximum absorption health`: Base: 0.0 Range: 0.0-2048.0
       - `Armor`: Base: 0.0 Range: 0.0-30.0
       - `Armor toughness`: Base: 0.0 Range: 0.0-20.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `ParticleLine`
**Display Particle Line**
Displays a line of particles between two locations to a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Start location
   - `[2]` `LOCATION` — End location
   - `[3]` `NUMBER` _(optional)_ — Effect spacing — _Default = 0.5 blocks_
