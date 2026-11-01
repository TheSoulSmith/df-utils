# DiamondFire actions — Game Action (`game_action`)

_Generated from actiondump (patch 8.13). Action names are exact wire strings; names in double quotes include **leading/trailing spaces** that must be kept._

## Block `game_action` — Game Action (110 actions in this file)

Used to do something related to the plot and everyone playing it.

### `FillContainer`
**Fill Container**
Fills the container at a location with items.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - `[1]` `ITEM` _(plural)_ — Item(s) to fill with

### `BreakBlock`
**Break Block**
Breaks the block at a location as if it was broken by a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Block(s) to break

### `L PFX Spiral`
**Create Particle Spiral**
> ⚠ **LEGACY** → use `ParticleSpiral`
Creates a spiral of particles at a certain location.
- _Info:_ The rotation of the spiral is determined by the pitch and yaw of the location in the chest.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Particle to spawn
   - `[1]` `LOCATION` — Spiral base location
   - `[2]` `NUMBER` _(optional)_ — Spiral length (default = 10 blocks)
   - `[3]` `NUMBER` _(optional)_ — Spiral diameter (default = 2 blocks)
- _Flags:_ rank: Noble

### `ParticleSphere`
**Create Particle Sphere** · aliases: `PFX Sphere`
> ⚠ **LEGACY** → use `ParticleSphere`
Creates a sphere of particles at a location for all players.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Center location
   - `[2]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
- _Flags:_ rank: Noble

### `EventAirChange`
**Set Event Air Change**
Sets the air change in this event.
- _Works with:_ Air Change Events
- _Info:_ Each bubble is equal to 30 air ticks.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — New air change

### `EventKnockback`
**Set Event Knockback**
Sets the knockback in this event.
- _Works with:_ Knockback Events
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VECTOR` — New knockback

### `ChangeSign`
**Change Sign Text**
Changes a line of text on a sign.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Sign location
   - `[1]` `NUMBER` — Line number
   - `[2]` `COMPONENT` _(optional)_ — New text
- **Tags:**
   - `Sign Side` (slot 26): `Front` (default), `Back`

### `WebRequest`
**Send Web Request**
Sends a web request to a URL.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — URL to request
   - `[1]` `TEXT` _(optional)_ — Content body
- **Tags:**
   - `Request Method` (slot 25): `Post` (default), `Get`, `Put`, `Delete`
   - `Content Type` (slot 26): `text/plain` (default), `application/json`
- _Flags:_ rank: Overlord, advanced

### `SimulationDistance`
**Set World Simulation Distance**
Sets the simulation distance in the world.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Simulation distance — _Limit: 2-32_
- _Flags:_ world-plot only

### `ClearScBoard`
**Clear Scoreboard**
> ⚠ **LEGACY** → use `ClearScoreboard`
Removes all scores from the scoreboard.

### `HideSidebar`
**Hide Scoreboard Sidebar**
> ⚠ **LEGACY** → use `SetSidebar` (Set Sidebar tag to Disable.)
Disables the scoreboard sidebar on the plot.

### `SetEventDeathMsg`
**Set Event Death Message**
Sets the death message in this event.
- _Works with:_ Player Death Event; Player Kill Player Event; Mob Kill Player Event
- _Info:_ Also sets the message that appears in the death screen.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — New death message

### `SpawnItemDisplay`
**Spawn Item Display**
> ⚠ **LEGACY** → use `SpawnItemDisp`
Spawns an item display entity.
- _Info:_ Notable defaults: ⏵ Billboard = Fixed
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `ITEM` — Displayed item
- _Flags:_ rank: Mythic

### `WriteTransaction`
**Write Transaction**
Adds blocks to the next transaction; a method of queuing up block operations so that they can be sent simultaneously.
- _Info:_ Requires the Game Action: Apply Transaction in order to apply changes to the world.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Block to set
   - `[1]` `LOCATION` — Corner 1
   - `[2]` `LOCATION` — Corner 2
   - `[3]` `BLOCK_TAG` _(optional)_ — Block data (comma separated) — _Example: "facing=up,half=top"_
- _Flags:_ rank: Overlord, advanced

### `SetWorldTime`
**Set Time**
Sets the time in the world.
- _Info:_ Add 24000 ticks to add one moon phase.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Daylight ticks — _Day: 1000 Noon: 6000 Night: 13000 Midnight: 18000_
- _Flags:_ world-plot only

### `ParticleSpiral`
**Create Particle Spiral** · aliases: `PFX Spiral`
> ⚠ **LEGACY** → use `ParticleSpiral`
Creates a spiral of particles at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect to play
   - `[1]` `LOCATION` — Base location
   - `[2]` `NUMBER` _(optional)_ — Length — _Default = 10 blocks_
   - `[3]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
   - `[4]` `NUMBER` _(optional)_ — Effect count — _Default = 50_
   - `[5]` `NUMBER` _(optional)_ — Rotations — _Default = 4_
- _Flags:_ rank: Noble

### `SetBlockData`
**Set Block Data**
Sets a data tag value of the block at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Location
   - `[1]` `BLOCK_TAG` _(plural)_ — Block data — _Example: "facing=up"_
- **Tags:**
   - `Overwrite Existing Data` (slot 26): `True`, `False` (default)

### `Firework`
**Spawn Firework**
Launches a firework rocket at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Firework rocket
   - `[1]` `LOCATION` — Spawn location
- **Tags:**
   - `Instant` (slot 25): `True`, `False` (default)
       - `True`: Instantly explodes.
   - `Movement` (slot 26): `Upwards` (default), `Directional`

### `SetEventDamage`
**Set Event Damage**
Sets the damage dealt in this event.
- _Works with:_ Damage Events
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — New damage amount — _❤ = 2 Health_

### `GetDialogText`
**Get Dialog Text**
Gets the text or selected option ID for a dialog input. Boolean inputs return "true" or "false"
- _Works with:_ Player Click Dialog Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Input ID

### `AdvanceWeather`
**Set Advance Weather**
Sets if the world has a natural weather cycle.
- **Tags:**
   - `Has Weather Cycle` (slot 26): `Enable` (default), `Disable`
- _Flags:_ world-plot only

### `FireSpreadRadius`
**Set Fire Spread Radius**
Sets the radius fire spreads around a player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Radius in blocks — _Set to -1 to disable fire spreading. Set to 0 to allow fire spreading away from players._
- _Flags:_ world-plot only

### `SpawnItem`
**Spawn Item**
Spawns an item at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to spawn
   - `[1]` `LOCATION` — Spawn location
   - `[2]` `COMPONENT` _(optional)_ — Custom name
- **Tags:**
   - `Apply Item Motion` (slot 26): `True` (default), `False`

### `SignColor`
**Set Sign Text Color**
Changes the text color of a sign.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Sign location
- **Tags:**
   - `Sign Side` (slot 24): `Front` (default), `Back`
   - `Text Color` (slot 25): `White`, `Orange`, `Magenta`, `Light blue`, `Yellow`, `Lime`, `Pink`, `Gray`, `Light gray`, `Cyan`, `Purple`, `Blue`, `Brown`, `Green`, `Red`, `Black` (default)
   - `Glowing` (slot 26): `Enable`, `Disable` (default)

### `ShulkerBullet`
**Spawn Shulker Bullet**
Spawns a shulker bullet at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn Location

### `FireworkEffect`
**Play Firework Explosion**
> ⚠ **LEGACY** → use `Firework` (Enable the Instant tag.)
Creates an instant firework explosion at a location.
- _Info:_ The explosion deals damage.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Firework rocket
   - `[1]` `LOCATION` — Location to spawn at

### `MobSpawning`
**Set Mob Spawning**
Sets if the world spawns mobs.
- _Info:_ Setting this to enable will also enable despawning.
- **Tags:**
   - `Mob Spawning` (slot 26): `Enable` (default), `Disable`
- _Flags:_ world-plot only

### `SetContainer`
**Set Container Contents**
Sets the contents of the container at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - `[1]` `ITEM` _(plural)_ — Item(s) to set

### `SpawnInteraction`
**Spawn Interaction Entity**
Spawns an invisible hitbox with the specified size.
- _Info:_ Detect interactions with: ⏵ Player Damage Entity Event ⏵ Player Right Click Entity Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `NUMBER` _(optional)_ — Hitbox width
   - `[2]` `NUMBER` _(optional)_ — Hitbox height
- **Tags:**
   - `Responsive` (slot 26): `Enable`, `Disable` (default)
       - `Enable`: Player swings main hand and plays the attack sound.
       - `Disable`: No visual or audio response.

### `SetItemInSlot`
**Set Item in Container Slot**
Sets the item in a slot of the container at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - `[1]` `ITEM` _(optional)_ — Item to set
   - `[2]` `NUMBER` — Slot

### `CloneRegion`
**Clone Region** · aliases: `CopyBlocks`
Copies a region of blocks to another region, including air.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Corner 1
   - `[1]` `LOCATION` — Corner 2
   - `[2]` `LOCATION` — Position to copy from
   - `[3]` `LOCATION` — Position to paste to
- **Tags:**
   - `Ignore Air` (slot 25): `True`, `False` (default)
   - `Clone Block Entities` (slot 26): `True` (default), `False`
       - `True`: Blocks such as chests will have their contents saved.  This takes up more server CPU.
       - `False`: Blocks such as chests will not have their contents saved.
- _Flags:_ rank: Overlord

### `SetBookSigning`
**Set Book Signing**
Sets whether a book will be signed after the player is done editing it in any way.
- _Works with:_ Player Edit Book Event
- **Tags:**
   - `Will Sign` (slot 26): `True`, `False` (default)

### `UncancelEvent`
**Uncancel Event**
Uncancels the initial event that triggered this line of code.
- _Info:_ Events cannot be uncancelled after a Control: Wait block.

### `SetLecternBook`
**Set Lectern Book**
Sets the book and the displayed page of a Lectern.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Lectern location
   - `[1]` `ITEM` _(optional)_ — Book to put
   - `[2]` `NUMBER` _(optional)_ — Displayed page

### `SpawnArmorStand`
**Spawn Armor Stand**
Spawns an armor stand at a location.
- _Info:_ Options to set the pose and tags are in Entity Action » Appearance.
- _Info:_ Equipment goes from the bottom left corner towards the right:  Helmet, Chestplate, Leggings, Boots, Right Hand, Left Hand
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `COMPONENT` _(optional)_ — Custom name
   - `[2]` `ITEM` _(optional, plural)_ — Equipment
- **Tags:**
   - `Visibility` (slot 26): `Visible` (default), `Visible (No hitbox)`, `Invisible`, `Invisible (No hitbox)`
- _Flags:_ rank: Mythic

### `SpawnBlockDisp`
**Spawn Block Display**
Spawns a block display entity.
- _Info:_ Notable defaults: ⏵ Billboard = Fixed
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `BLOCK` — Displayed block
   - `[2]` `BLOCK_TAG` _(optional, plural)_ — Block data — _Example: "facing=up", "half=top"_
- _Flags:_ rank: Mythic

### `RedstoneStrength`
**Set Event Redstone Current Strength**
Sets the current strength in this event.
- _Works with:_ Redstone Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — New current strength — _Range: 0-15_
- _Flags:_ world-plot only

### `GetDialogNumber`
**Get Dialog Number**
Gets the number entered for a dialog input.
- _Works with:_ Player Click Dialog Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Input ID

### `ClearContainer`
**Clear Container**
Empties a container at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location

### `TraderSpawning`
**Set Wandering Trader Spawning**
Sets if the world spawns wandering traders.
- **Tags:**
   - `Trader Spawning` (slot 26): `Enable` (default), `Disable`
- _Flags:_ world-plot only

### `CancelEvent`
**Cancel Event**
Cancels the initial event that triggered this line of code.
- _Info:_ Events cannot be cancelled after a Control: Wait block.

### `ParticleEffect`
**Play Particle Effect** · aliases: `Particle FX`
> ⚠ **LEGACY** → use `Particle`
Plays a particle effect at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` _(plural)_ — Effects to play
   - `[1]` `LOCATION` — Effect location

### `SpawnFangs`
**Spawn Evoker Fangs**
Spawns evoker fangs at a location.
- _Info:_ Evoker Fangs deal damage and remove themselves a second later.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `COMPONENT` _(optional)_ — Custom name

### `SetEventSound`
**Set Event Sound**
Sets the sound to play for this event, replacing the original sound.
- _Works with:_ Entity Death Events
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `SOUND` — New sound

### `SetEventXP`
**Set Event Experience**
Sets the amount of experience this event should drop.
- _Works with:_ Entity Death Events; Fish Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Experience

### `LockContainer`
**Set Container Lock**
Sets the lock key of the container at a location.
- _Info:_ Lock keys determine the name of the item used to open the container.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - **OR**
   - `[1]` `TEXT` — Lock key
   - **OR**
   - `[2]` `NONE` — Unlocks the container)

### `RemoveScore`
**Remove Scoreboard Score**
> ⚠ **LEGACY** → use `RemoveScore`
Removes a score from the scoreboard.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Score name

### `CreateHologram`
**Create Hologram**
> ⚠ **LEGACY** → use `DisplayHologram` (Use 'Display Hologram' (Player Action) to display a hologram to players. Run it for each player that should see it.)
Creates a legacy hologram (armor stand name tag).
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Hologram location
   - `[1]` `TEXT` — Hologram text
- _Flags:_ rank: Mythic

### `SetExhaustion`
**Set Event Exhaustion**
Sets the exhaustion gained in this event.
- _Works with:_ Exhaustion Events
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — New exhaustion amount

### `ParticleCircle`
**Create Particle Circle** · aliases: `PFX Circle`
> ⚠ **LEGACY** → use `ParticleCircle`
Creates a circle of particles at a location for all players.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Center location
   - `[2]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
- _Flags:_ rank: Noble

### `AdvanceTime`
**Set Advance Time**
Sets if the world has a natural time cycle.
- **Tags:**
   - `Advance Time` (slot 26): `Enable` (default), `Disable`
- _Flags:_ world-plot only

### `PFX Line [A]`
**Create Animated Particle Line**
> ⚠ **LEGACY** → use `ParticleLineA`
Creates a line of particles between two locations.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Particle to create
   - `[1]` `LOCATION` — Start location
   - `[2]` `LOCATION` — End location
   - `[3]` `NUMBER` _(optional)_ — Animation duration (ticks)
- _Flags:_ rank: Mythic

### `ClearItems`
**Clear Container Items**
Removes all of an item from the container at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - `[1]` `ITEM` _(plural)_ — Item(s) to clear

### `StartLoop`
**Start Loop**
> ⚠ **LEGACY** → use `` (Loops have been removed, start a process when a player joins, and use a Repeat: Forever to loop, set the delay with Control: Wait)
Activates your plot's Loop Block if it has one.

### `SetFurnaceSpeed`
**Set Furnace Cook Time**
Sets the amount of ticks it takes for a furnace block to cook an item.
- _Works with:_ Furnace; Blast Furnace; Smoker
- _Info:_ By default, a furnace cooks in 200 ticks.
- _Info:_ Fuel duration is unaffected by cooking time.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Furnace location
   - `[1]` `NUMBER` — Ticks

### `BlockDropsOn`
**Enable Block Drops**
Enables blocks dropping as items when broken.

### `SpawnMannequin`
**Spawn Mannequin**
Spawns a mannequin at a location.
- _Info:_ To get a skin use /item head <player name> OR /item mshead
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Mannequin player head
   - **OR**
   - `[1]` `TEXT` — Mannequin player name or UUID
   - **OR**
   - `[2]` `LOCATION` — Spawn location
   - `[3]` `COMPONENT` _(optional)_ — Description — _Only shown if name is visible_
- **Tags:**
   - `Movable` (slot 26): `True` (default), `False`
   - `Main Hand` (slot 25): `Left Hand` (default), `Right Hand`
- _Flags:_ rank: Overlord

### `ViewDistance`
**Set World View Distance**
Sets the view distance in the world.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — View distance — _Limit: 2-32_
- _Flags:_ world-plot only

### `BoneMeal`
**Bone Meal Block**
Applies bone meal to a block.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Block(s) to bone meal
   - `[1]` `NUMBER` _(optional)_ — Number of uses
- **Tags:**
   - `Show Particles` (slot 26): `True` (default), `False`
- _Flags:_ rank: Noble, requires tokens

### `SetDifficulty`
**Set Difficulty**
Sets the world's difficulty.
- **Tags:**
   - `Difficulty` (slot 26): `Peaceful` (default), `Easy`, `Normal`, `Hard`
- _Flags:_ world-plot only

### `DebugStackTrace`
- _Flags:_ rank: Dev

### `FallingBlock`
**Spawn Falling Block**
Spawns a falling block at a location.
- _Info:_ Falling blocks automatically disappear after 30 seconds.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Block location
   - **OR**
   - `[1]` `BLOCK` — Block material
   - **OR**
   - `[2]` `NONE` — Converts the block at the location to a falling block)
   - `[3]` `BLOCK_TAG` _(optional, plural)_ — Block data
- **Tags:**
   - `Hurt Hit Entities` (slot 26): `True`, `False` (default)
   - `Reform on Impact` (slot 25): `True` (default), `False`

### `SetDisplayedItem`
**Set Event Displayed Item**
Sets the item displayed in this event.
- _Works with:_ Vault Display Item Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item to display
- _Flags:_ world-plot only

### `DiscordWebhook`
**Send Discord Webhook Message**
Sends a message to a Discord webhook.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Webhook URL
   - `[1]` `TEXT` — Message content
- _Flags:_ rank: Overlord, advanced

### `TickBlock`
**Random Tick Block**
Causes a block to get "random ticked", which could cause a block update.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Block(s) to tick
   - `[1]` `NUMBER` _(optional)_ — Number of ticks

### `ReplaceItems`
**Replace Container Items**
Replaces items in the container at a location with the given item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - `[1]` `ITEM` _(optional, plural)_ — Item(s) to replace
   - `[2]` `ITEM` — Item to replace with
   - `[3]` `NUMBER` _(optional)_ — Amount of items to replace

### `PruneChunks`
**Prune World**
Deletes all unloaded chunks from the world.
- _Info:_ Recently unloaded chunks might not be deleted until the next server restart.
- _Flags:_ world-plot only

### `SetEventProj`
**Set Event Projectile**
Replaces the projectile fired in the Shoot Bow Event.
- _Works with:_ Shoot Bow Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PROJECTILE` _(optional)_ — Projectile to launch

### `RandomTickSpeed`
**Set World Random Tick Speed**
Sets the random tick speed in the world.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Tick speed
- _Flags:_ world-plot only

### `Explosion`
**Create Explosion**
Creates an explosion at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Explosion location
   - `[1]` `NUMBER` _(optional)_ — Explosion power (0-4) — _Default = 4_

### `SpawnMob`
**Spawn Mob**
Spawns a mob at a location.
- _Info:_ Equipment goes from the bottom left corner towards the right:  Main Hand, Helmet, Chestplate, Leggings, Boots, Off Hand
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `SPAWN_EGG` — Mob type
   - `[1]` `LOCATION` — Spawn location
   - `[2]` `NUMBER` _(optional)_ — Health
   - `[3]` `COMPONENT` _(optional)_ — Custom name
   - `[4]` `POTION` _(optional, plural)_ — Effect(s)
   - `[5]` `ITEM` _(optional, plural)_ — Equipment

### `SetBrushableItem`
**Set Item in Brushable Block**
Sets the item buried in a suspicious sand or gravel.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Block location
   - `[1]` `ITEM` _(optional)_ — Item

### `ParticleLineA`
**Create Animated Particle Line**
> ⚠ **LEGACY** → use `ParticleLineA`
Creates an animated line of particles between two locations.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect to play
   - `[1]` `LOCATION` — Start location
   - `[2]` `LOCATION` — End location
   - `[3]` `NUMBER` _(optional)_ — Effect spacing — _Default = 0.5 blocks_
   - `[4]` `NUMBER` _(optional)_ — Animation duration — _Default = 40 ticks_
- _Flags:_ rank: Mythic

### `SpawnEnderEye`
**Spawn Eye of Ender**
Spawns an eye of ender at a location, which (if specified) will float towards its destination.
- _Info:_ If the destination is further than 12 blocks away, the eye's path always goes upwards.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Location to spawn at
   - `[1]` `LOCATION` _(optional)_ — Destination
   - `[2]` `NUMBER` _(optional)_ — Lifespan (ticks)
   - `[3]` `COMPONENT` _(optional)_ — Custom name
- **Tags:**
   - `End of Lifespan` (slot 26): `Drop item`, `Shatter`, `Random` (default)
       - `Drop item`: The eye always drops itself as an item.
       - `Shatter`: The eye always shatters.
       - `Random`: The eye has a 20% chance of shattering.

### `ShowSidebar`
**Show Scoreboard Sidebar**
> ⚠ **LEGACY** → use `SetSidebar` (Set Sidebar tag to Enable.)
Enables the scoreboard sidebar on the plot.

### `SpawnPotionCloud`
**Spawn Area of Effect Cloud**
Spawns a lingering potion cloud at a location that imbues effects onto entities who enter it.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `POTION` _(optional, plural)_ — Effect to apply
   - `[2]` `COMPONENT` _(optional)_ — Custom name
   - `[3]` `NUMBER` _(optional)_ — Radius (blocks) — _Default = 3_
   - `[4]` `NUMBER` _(optional)_ — Duration (ticks) — _Default = 200_
- _Flags:_ rank: Mythic

### `LaunchProj`
**Launch Game Projectile**
Launches a projectile.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PROJECTILE` — Projectile to launch
   - `[1]` `LOCATION` — Launch point
   - `[2]` `COMPONENT` _(optional)_ — Custom name
   - `[3]` `NUMBER` _(optional)_ — Speed
   - `[4]` `NUMBER` _(optional)_ — Inaccuracy — _Controls how much random motion is applied on launch Default = 1_

### `" SetBlock "`
**Set Block**
Sets the block at a location.
- _Info:_ Will cause block updates. Use SetRegion to not update nearby blocks and save CPU.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Block to set
   - `[1]` `LOCATION` _(plural)_ — Block location(s)
   - `[2]` `BLOCK_TAG` _(optional, plural)_ — Block data — _Example: "facing=up", "half=top"_

### `SpawnItemDisp`
**Spawn Item Display**
Spawns an item display entity.
- _Info:_ Notable defaults: ⏵ Billboard = Fixed
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `ITEM` — Displayed item
- _Flags:_ rank: Mythic

### `SetBlockGrowth`
**Set Block Growth**
Sets the growth stage of the block (eg. carrots) at a location.
- _Info:_ Most crops have growth stages from 0 to 7.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Block location
   - `[1]` `NUMBER` _(optional)_ — Growth stage — _Default = 0_
- **Tags:**
   - `Growth Unit` (slot 26): `Growth Stage Number` (default), `Growth Percentage`

### `Wait`
**Wait**
> ⚠ **LEGACY** → use `Wait`
Pauses the current code sequence for a duration of ticks, seconds, or minutes.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Wait duration — _Default = 1_
- **Tags:**
   - `Delay Unit` (slot 26): `Ticks` (default), `Seconds`, `Minutes`

### `SetContainerName`
**Set Container Name**
Sets the name of the container at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - `[1]` `COMPONENT` — Name

### `SetHead`
**Set Player Head**
Sets the block at a location to a player head.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Head location
   - **OR**
   - `[1]` `ITEM` — Player Head
   - **OR**
   - `[2]` `TEXT` — Head owner — _Player name or UUID_

### `RemoveHologram`
**Remove Hologram**
> ⚠ **LEGACY** → use `DisplayHologram`
Removes a hologram created by legacy hologram actions.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Hologram location
   - `[1]` `TEXT` _(optional)_ — Hologram name
- _Flags:_ rank: Mythic

### `RemoveItems`
**Remove Container Items**
Removes items from the container at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - `[1]` `ITEM` _(plural)_ — Item(s) to remove

### `SpawnRngItem`
**Spawn Random Item**
> ⚠ **LEGACY** → use `SpawnItem` (Pick a random item with Set Variable: Set to Random Value and spawn it normally.)
Randomly spawns an item at a certain location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Items to pick from
   - `[1]` `LOCATION` — Location to spawn at
   - `[2]` `TEXT` _(optional)_ — Item name
- **Tags:**
   - `Apply Item Motion` (slot 26): `True` (default), `False`

### `SetRegion`
**Set Region** · aliases: `SetBlock`
Fills a region with a type of block.
- _Info:_ Can set up to 100,000 blocks per action.
- _Info:_ Will not cause block updates. A 1x1x1 SetRegion may save CPU compared to SetBlock in certain situations.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Block to set
   - `[1]` `LOCATION` — Corner 1
   - `[2]` `LOCATION` — Corner 2
   - `[3]` `BLOCK_TAG` _(optional)_ — Block data (comma separated) — _Example: "facing=up,half=top"_

### `ParticleCircleA`
**Create Animated Particle Circle** · aliases: `PFX Circle [A]`
> ⚠ **LEGACY** → use `ParticleCircleA`
Creates an animated circle of particles at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect to play
   - `[1]` `LOCATION` — Center location
   - `[2]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
   - `[3]` `NUMBER` _(optional)_ — Animation duration — _Default = 40 ticks_
- _Flags:_ rank: Mythic

### `SleepPercentage`
**Set Players Sleeping Percentage**
Sets the percentage of players that must sleep in order to skip the night.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Percentage of players — _Default = 0% Range: 0% (1 player) to 100% (all players)_
- _Flags:_ world-plot only

### `VineSpreading`
**Set Vine Spreading**
Sets if the world spreads vines.
- **Tags:**
   - `Vine Spreading` (slot 26): `Enable` (default), `Disable`
- _Flags:_ world-plot only

### `SpawnTNT`
**Spawn Primed TNT**
Spawns primed TNT at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `NUMBER` _(optional)_ — TNT power (0-4) — _Default = 4_
   - `[2]` `NUMBER` _(optional)_ — Fuse duration — _Default = 80_
   - `[3]` `COMPONENT` _(optional)_ — Custom name

### `SpawnExpOrb`
**Spawn Experience Orb**
Spawns an experience orb at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `NUMBER` _(optional)_ — Experience amount
   - `[2]` `COMPONENT` _(optional)_ — Custom name

### `SetBiome`
**Set Biome**
Sets the biome of a region.
- _Info:_ Can set up to 100,000 blocks per action.
- _Info:_ Players may have to reload the chunks on their client to see a change.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Corner 1
   - `[1]` `LOCATION` — Corner 2
   - `[2]` `TEXT` — Biome to set — _Example: "basalt_deltas", "plains"_
- _Flags:_ rank: Noble

### `SetEventHeal`
**Set Event Heal Amount**
Sets the amount of health regained in this event.
- _Works with:_ Player Heal Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — New healing amount — _❤ = 2 Health_

### `PFX Path`
**Create Particle Path**
> ⚠ **LEGACY** → use `ParticleLine` (Repeat through the locations and display a line between them.)
Creates a path of particles that goes through each location in the chest from first to last.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Particle to create
   - `[1]` `LOCATION` _(plural)_ — Path locations

### `ApplyTransaction`
**Apply Transaction**
Applies the current transaction and generates a new one.
- _Info:_ Can set up to 1,000,000 blocks per transaction.
- _Info:_ Only 1 transaction can be active at a time.
- _Info:_ Transactions are asynchronous, and may override blocks set after them.
- _Flags:_ rank: Overlord, advanced

### `ParticleRay`
**Create Particle Ray** · aliases: `PFX Ray`
> ⚠ **LEGACY** → use `ParticleRay`
Creates a ray of particles along a vector.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect to play
   - `[1]` `LOCATION` — Start location
   - `[2]` `VECTOR` — Ray vector
   - `[3]` `NUMBER` _(optional)_ — Effect spacing — _Default = 0.5 blocks_

### `GenerateTree`
**Generate Tree**
Generates a tree at a location.
- _Info:_ The location is the northwest corner of 2x2 trees.
- _Info:_ Trees will not grow on blocks that do not normally support tree growth.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Tree location (bottom log block)
- **Tags:**
   - `Tree Type` (slot 26): `Oak Tree` (default), `Big Oak Tree`, `Swamp Tree`, `Spruce Tree`, `Slightly Taller Spruce Tree`, `Big Spruce Tree`, `Birch Tree`, `Tall Birch Tree`, `Jungle Tree`, `Big Jungle Tree`, `Jungle Bush`, `Acacia Tree`, `Dark Oak Tree`, `Pale Oak Tree`, `Creaking Pale Oak Tree`, `Mangrove Tree`, `Tall Mangrove Tree`, `Cherry Tree`, `Azalea Tree`, `Red Mushroom`, `Brown Mushroom`, `Crimson Fungus`, `Warped Fungus`, `Chorus Plant`

### `StopLoop`
**Stop Loop**
> ⚠ **LEGACY** → use `` (Loops have been removed, start a process when a player joins, and use a Repeat: Forever to loop, set the delay with Control: Wait)
Deactivates your plot's Loop Block if it has one.

### `SetScObj`
**Set Scoreboard Objective Name**
> ⚠ **LEGACY** → use `SetScoreObj`
Sets the objective name of the scoreboard on your plot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Objective name — _128 characters or less_

### `SpawnCrystal`
**Spawn End Crystal**
Spawns an end crystal at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `COMPONENT` _(optional)_ — Custom name
- **Tags:**
   - `Show Bottom` (slot 26): `True` (default), `False`

### `SetCampfireItem`
**Set Campfire Item**
Sets the item being cooked in one of a campfire's slots.
- _Info:_ After the cooking time, the item drops from the campfire (unless the campfire is unlit).
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Campfire location
   - `[1]` `ITEM` — Campfire item
   - `[2]` `NUMBER` _(optional)_ — Cooking time (ticks) — _Default = 600_
- **Tags:**
   - `Campfire Slot` (slot 26): `1` (default), `2`, `3`, `4`
       - `1`: Sets the item on the southwest corner of the campfire.
       - `2`: Sets the item on the northwest corner of the campfire.
       - `3`: Sets the item on the northeast corner of the campfire.
       - `4`: Sets the item on the southeast corner of the campfire.

### `SpawnTextDisplay`
**Spawn Text Display**
Spawns a text display entity.
- _Info:_ Notable defaults: ⏵ Line Width = 200 ⏵ Text Alignment = Center ⏵ Billboard = Center
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Spawn location
   - `[1]` `COMPONENT` _(plural)_ — Displayed text
- **Tags:**
   - `Text Value Merging` (slot 26): `Add spaces`, `No spaces` (default)
   - `Inherit Styles` (slot 25): `True` (default), `False`
       - `True`: Text style of the last value will be inherited when merging multiple values.
       - `False`: Each value will have independent text style when merging multiple values.
- _Flags:_ rank: Mythic

### `SpawnVehicle`
**Spawn Vehicle**
Spawns a vehicle at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VEHICLE` — Vehicle type
   - `[1]` `LOCATION` — Spawn location
   - `[2]` `COMPONENT` _(optional)_ — Custom name

### `Lightning`
**Summon Lightning**
Strikes lightning at a location.
- _Info:_ The lightning's damage can be detected using the "lightning" damage event cause.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Impact location

### `ParticleSpiralA`
**Create Animated Particle Spiral** · aliases: `PFX Spiral [A]`
> ⚠ **LEGACY** → use `ParticleSpiralA`
Creates an animated spiral of particles at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect to play
   - `[1]` `LOCATION` — Base location
   - `[2]` `NUMBER` _(optional)_ — Length — _Default = 10 blocks_
   - `[3]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
   - `[4]` `NUMBER` _(optional)_ — Particle count — _Default = 50_
   - `[5]` `NUMBER` _(optional)_ — Rotations — _Default = 4_
   - `[6]` `NUMBER` _(optional)_ — Animation duration — _Default = 40 ticks_
- _Flags:_ rank: Mythic

### `SetScore`
**Set Scoreboard Score**
> ⚠ **LEGACY** → use `SetScore`
Sets a score on the scoreboard.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Score name — _32 characters or less_
   - `[1]` `NUMBER` _(optional)_ — Score value — _Default = 0_

### `ParticleCluster`
**Create Particle Cluster** · aliases: `PFX Cluster`, `L PFX Cluster`
> ⚠ **LEGACY** → use `Particle`
Randomly spawns particles around a certain location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Particle to create
   - `[1]` `LOCATION` — Center location
   - `[2]` `NUMBER` _(optional)_ — Diameter — _Default = 2 blocks_
   - `[3]` `NUMBER` _(optional)_ — Particle density — _Default = 5 per block_
- _Flags:_ rank: Noble

### `BlockDropsOff`
**Disable Block Drops**
Disables blocks dropping as items when broken.

### `ParticleLine`
**Create Particle Line** · aliases: `PFX Line`
> ⚠ **LEGACY** → use `ParticleLine`
Creates a line of particles between two locations for all players.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PARTICLE` — Effect
   - `[1]` `LOCATION` — Start location
   - `[2]` `LOCATION` — End location
   - `[3]` `NUMBER` _(optional)_ — Effect spacing — _Default = 0.5 blocks_
