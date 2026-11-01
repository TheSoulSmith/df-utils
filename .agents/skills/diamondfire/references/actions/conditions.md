# DiamondFire actions — Conditions (`if_player`, `if_entity`, `if_game`, `if_var`)

_Generated from actiondump (patch 8.13). Action names are exact wire strings; names in double quotes include **leading/trailing spaces** that must be kept._

## Block `if_player` — If Player (42 actions in this file)

Used to execute the code inside it if a certain condition related to a player is met.

### `HasRoomForItem`
**Has Room For Item** · aliases: `PHasRoomForItem` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if a player's inventory has room for one or more items to be given.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item(s) to check with
   - **OR**
   - `[1]` `NONE` — Checks for empty slot)
- **Tags:**
   - `Check Mode` (slot 26): `Has Room for Any Item` (default), `Has Room for All Items`
   - `Checked Slots` (slot 25): `Entire inventory`, `Main inventory` (default), `Upper inventory`, `Hotbar`, `Armor`

### `IsHoldingOff`
**Is Holding Item (Off hand)**
> ⚠ **LEGACY** → use `IsHolding`
Checks if a player is holding an item in their off hand.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Item(s) to check for

### `NoItemCooldown`
**Item Is Not On Cooldown**
Checks if a player does not have a cooldown applied to an item type.
- _Info:_ The check will succeed if any of the given items are not on cooldown.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item type(s) to check

### `IsUsingItem`
**Is Using Item**
Checks if a player is currently using an item (eg. bow).
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Item(s) to check — _Checks material only_

### `HasAllItems`
**Has All Items**
> ⚠ **LEGACY** → use `HasItem` (Set Check Mode tag to Has All Items.)
Checks if a player has all of the items in the chest.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to check for

### `HasItem`
**Has Item**
Checks if a player has an item in their inventory.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to check for
- **Tags:**
   - `Check Mode` (slot 26): `Has Any Item` (default), `Has All Items`

### `BlockEquals`
**Block Equals**
> ⚠ **LEGACY** → use `EventBlockEquals`
Checks if the block in a block interaction event is a certain block.
- _Works with:_ Player Place Block Event; Player Break Block Event
- _Info:_ If multiple items are in the chest, the block can be any of them.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` _(plural)_ — Block(s) to check for
- _Flags:_ rank: Emperor

### `IsWearing`
**Is Wearing Item** · aliases: `PIsWearing` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if a player is wearing an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to check for
- **Tags:**
   - `Check Mode` (slot 26): `Is Wearing Some` (default), `Is Wearing All`

### `IsNear`
**Is Near Location** · aliases: `PIsNear` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if a player is within a range of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Center location
   - `[1]` `NUMBER` _(optional)_ — Radius — _Default = 5 blocks_
- **Tags:**
   - `Shape` (slot 26): `Sphere` (default), `Circle`, `Cube`, `Square`
       - `Sphere`: Checks if √(X²+Y²+Z²) is within the radius.
       - `Circle`: Checks if √(X²+Z²) is within the radius.
       - `Cube`: Checks if the distance on the X, Y or Z axis is within the radius.
       - `Square`: Checks if the distance on the X or Z axis is within the radius.

### `IsRiding`
**Is Riding Entity**
> ⚠ **LEGACY** → use `" IsRiding "`
Checks if a player is riding an entity (including vehicles).
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(optional, plural)_ — Entity type (e.g. "pig"), name, or UUID to compare ridden entity to
- **Tags:**
   - `Compare Text To` (slot 26): `Entity type` (default), `Name or UUID`

### `CmdEquals`
**Command Equals**
> ⚠ **LEGACY** → use `CommandEquals`
Checks if the command entered in the Command Event is equal to the given string.
- _Works with:_ Command Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — String(s) to check for
- **Tags:**
   - `Check Mode` (slot 26): `Check Entire Command` (default), `Check First Word`
   - `Ignore Case` (slot 25): `True` (default), `False`
- _Flags:_ rank: Emperor

### `CursorItem`
**Cursor Item Equals**
Checks if the item that is being moved with a player's cursor is the given item.
- _Info:_ If multiple items are in the chest, the target can have any of them on their cursor.
- _Info:_ When used on the Player Click Item in Own Inventory Event, 'Cursor Item Equals' checks the previous cursor item, not the item that was clicked.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Items(s) to check for

### `SlotEquals`
**Hotbar Slot Equals**
Checks if a player's currently selected hotbar slot equals the given slot ID.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Slot ID to check — _1 (left) to 9 (right)_

### `ItemEquals`
**Item Equals**
> ⚠ **LEGACY** → use `ItemEquals`
Checks if the items in one of the events below is a certain item.
- _Works with:_ Player Click Item Events; Player Pickup Item Event; Player Drop Item Event; Player Consume Item Event
- _Info:_ If multiple items are in the chest, the event item can be any of them.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Items(s) to check for

### `MenuSlotEquals`
**Inventory Menu Slot Equals**
Checks if a player's currently open inventory menu contains an item in the given slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(plural)_ — Slot(s) to check
   - `[1]` `ITEM` _(optional, plural)_ — Item(s) to check for

### `HasPermission`
**Has Plot Permission**
Checks if a player has a certain level of access on this plot, such as builder or owner.
- **Tags:**
   - `Permission` (slot 26): `Owner`, `Developer`, `Builder`, `Developer or builder` (default), `Whitelisted`
       - `Owner`: Checks if the target is the plot owner.
       - `Developer`: Checks if the target has dev perms.
       - `Builder`: Checks if the target has build perms.
       - `Developer or builder`: Checks if the target has dev or build perms.
       - `Whitelisted`: Checks if the target is whitelisted on the plot.

### `" IsRiding "`
**Is Riding Entity**
Checks if a player is riding another entity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ENTITY_TYPE` _(plural)_ — Spawn egg, projectile, or vehicle
   - **OR**
   - `[1]` `TEXT` _(plural)_ — Entity UUID
   - **OR**
   - `[2]` `NONE` — Checks if riding any entity
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `MainHandEquals`
**Main Hand Equals**
Checks if a player's main hand is their left or right hand.
- **Tags:**
   - `Main Hand` (slot 26): `Left Hand` (default), `Right Hand`

### `IsSneaking`
**Is Sneaking**
Checks if a player is sneaking.

### `MovementKey`
**Is Movement Key Pressed** · aliases: `PMovementKey` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if a player is holding a specific movement key.
- **Tags:**
   - `Forward (W)` (slot 20): `Pressed`, `Released`, `Don't check` (default)
   - `Backward (S)` (slot 21): `Pressed`, `Released`, `Don't check` (default)
   - `Left (A)` (slot 22): `Pressed`, `Released`, `Don't check` (default)
   - `Right (D)` (slot 23): `Pressed`, `Released`, `Don't check` (default)
   - `Jump (Space)` (slot 24): `Pressed`, `Released`, `Don't check` (default)
   - `Sneak (Left Shift)` (slot 25): `Pressed`, `Released`, `Don't check` (default)
   - `Sprint (Left Control)` (slot 26): `Pressed`, `Released`, `Don't check` (default)

### `IsFlying`
**Is Flying**
Checks if a player is flying.

### `InvOpen`
**Inventory Type Open**
Checks if a player has a certain inventory type open.
- _Info:_ Does not work with special screens such as the death screen or the player's own inventory.
- **Tags:**
   - `Inventory Type` (slot 26): `Any Inventory` (default), `Plot Menu`, `Crafting Table`, `Chest`, `Double Chest`, `Ender Chest`, `Shulker Box`, `Barrel`, `Furnace (any)`, `Furnace`, `Blast Furnace`, `Smoker`, `Dropper`, `Dispenser`, `Beacon`, `Hopper`, `Anvil`, `Brewing Stand`, `Cartography Table`, `Smithing Table`, `Loom`, `Grindstone`, `Stonecutter`, `Enchanting Table`, `Trader Menu (any)`, `Villager Menu`, `Wandering Trader Menu`, `Horse Inventory`, `Llama Inventory`

### `HasSlotItem`
**Has Item in Slot**
Checks if a player has an item in the given inventory slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(plural)_ — Slot(s) to check
   - `[1]` `ITEM` _(optional, plural)_ — Item(s) to check for

### `IsSprinting`
**Is Sprinting**
Checks if a player is sprinting or using the sprint key to swim.

### `IsGliding`
**Is Gliding**
Checks if a player is gliding with elytra.

### `CmdArgEquals`
**Command Argument Equals**
> ⚠ **LEGACY** → use `CmdArgEquals`
Checks if a certain argument of a player's command is equal to a certain text when the Player Command Event is executed.
- _Works with:_ Player Command Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Text(s) to check for
   - `[1]` `NUMBER` — Argument number
- **Tags:**
   - `Ignore Case` (slot 26): `True` (default), `False`
- _Flags:_ rank: Emperor

### `IsLookingAt`
**Is Looking at Block**
Checks if a player is looking at the given block or location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` _(optional, plural)_ — Block to check for
   - **OR**
   - `[1]` `LOCATION` _(plural)_ — Location to check for
   - `[2]` `NUMBER` _(optional)_ — Maximum distance from target block/location — _This is distance from the player's selected block_
- **Tags:**
   - `Fluid Mode` (slot 26): `Ignore fluids` (default), `Detect fluids`

### `InWorldBorder`
**Is In World Border**
Checks if a player (or a location) is within their world border.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Location to check
   - **OR**
   - `[1]` `NONE` — Checks player location
- _Flags:_ rank: Emperor

### `IsInGameMode`
**Is in Game Mode**
Checks if a player is in a specific game mode.
- **Tags:**
   - `Game Mode` (slot 26): `Survival` (default), `Creative`, `Adventure`, `Spectator`

### `UsingPack`
**Is Using Resource Pack**
Checks if a player is using a plot resource pack.
- _Flags:_ rank: Overlord

### `IsSwimming`
**Is Swimming**
Checks if a player is in water or lava.
- _Info:_ Use 'Is Sprinting' to check if a player is swimming with the swimming animation.

### `StandingOn`
**Is Standing on Block** · aliases: `PStandingOn` · **wire `subAction` = alias** (name collides across if-blocks)
> ⚠ **LEGACY** → use `" StandingOn "`
Checks if a player is standing on the given block or location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` _(optional, plural)_ — Block to check for
   - **OR**
   - `[1]` `LOCATION` _(plural)_ — Location to check for

### `" StandingOn "`
**Is Standing on Block** · aliases: `" PStandingOn "` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if a player is standing on the given block or location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` _(optional, plural)_ — Block to check for
   - **OR**
   - `[1]` `LOCATION` _(plural)_ — Location to check for

### `IsGrounded`
**Is Grounded**
Checks if a player is supported by a block.

### `OwnsProduct`
**Owns Product**
Checks if a player owns a plot product or is a developer or builder.
- _Info:_ Only works with one-time purchase products.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Product ID

### `IsHoldingMain`
**Is Holding Item (Main hand)**
> ⚠ **LEGACY** → use `IsHolding`
Checks if a player is holding an item in their main hand.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Item(s) to check for

### `IsHolding`
**Is Holding Item**
Checks if a player is holding an item in their hand.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Item(s) to check for
- **Tags:**
   - `Hand Slot` (slot 26): `Either hand` (default), `Main hand`, `Off hand`

### `IsBlocking`
**Is Blocking**
Checks if a player is blocking with a shield.

### `IsHitboxNear`
**Is Hitbox Near Location** · aliases: `PHitboxIsNear` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if a player's hitbox is within a range of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Center location
   - `[1]` `NUMBER` _(optional)_ — Range — _Default = 5 blocks_
- **Tags:**
   - `Shape` (slot 26): `Sphere` (default), `Circle`, `Cube`, `Square`
       - `Sphere`: Checks if √(X²+Y²+Z²) is within the radius.
       - `Circle`: Checks if √(X²+Z²) is within the radius.
       - `Cube`: Checks if the distance on the X, Y or Z axis is within the radius.
       - `Square`: Checks if the distance on the X or Z axis is within the radius.

### `HasPotion`
**Has Potion Effect** · aliases: `HasEffect`
Checks if a player has a potion effect of the given type active.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `POTION` _(plural)_ — Effect(s) to check for
- **Tags:**
   - `Check Properties` (slot 25): `None` (default), `Amplifier`, `Duration`, `Amplifier and duration`
       - `None`: Checks only if the effect type matches.
       - `Amplifier`: The amplifier of the player's effect must be equal or higher.
       - `Duration`: The duration of the player's effect must be equal or higher.
       - `Amplifier and duration`: The amplifier and duration of the player's effect must be equal or higher.
   - `Check Mode` (slot 26): `Has any effect` (default), `Has all effects`

### `NameEquals`
**Name Equals** · aliases: `PNameEquals` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if a player's username is equal to one of the given usernames (case insensitive).
- _Info:_ Works with UUIDs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Name(s) to check for

### `CanFly`
**Can Fly**
Checks if a player can fly.

## Block `if_entity` — If Entity (20 actions in this file)

Used to execute the code inside it if a certain condition related to an entity or multiple entities is met.

### `IsVehicle`
**Is Vehicle**
Checks if an entity is a boat or minecart.

### `IsOxidized`
**Is Oxidized**
Checks what stage of oxidization a copper golem is at.
- _Works with:_ Copper Golem
- **Tags:**
   - `Oxidization State` (slot 26): `Unaffected` (default), `Exposed`, `Weathered`, `Oxidized`

### `IsProj`
**Is Projectile**
Checks if an entity is a projectile.

### `HasCustomTag`
**Has Custom Tag**
Checks if an entity has a given custom tag, and (if provided) whether the tag matches the given value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Tag name
   - **OR**
   - `[1]` `NUMBER` _(optional)_ — Tag value
   - **OR**
   - `[2]` `TEXT` _(optional)_ — Tag value

### `IsSheared`
**Is Sheared**
Checks if a sheep is sheared.
- _Works with:_ Sheep

### `IsNear`
**Is Near Location** · aliases: `EIsNear` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if an entity is within a range of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Center location
   - `[1]` `NUMBER` _(optional)_ — Range — _Default = 5 blocks_
- **Tags:**
   - `Shape` (slot 26): `Sphere` (default), `Circle`, `Cube`, `Square`
       - `Sphere`: Checks if √(X²+Y²+Z²) is within the radius.
       - `Circle`: Checks if √(X²+Z²) is within the radius.
       - `Cube`: Checks if the distance on the X, Y or Z axis is within the radius.
       - `Square`: Checks if the distance on the X or Z axis is within the radius.

### `IsRiding`
**Is Riding Entity**
> ⚠ **LEGACY** → use `" IsRiding "`
Checks if a player is riding an entity (including vehicles).
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(optional, plural)_ — Entity type (e.g. "pig"), name, or UUID to compare ridden entity to
- **Tags:**
   - `Compare Text To` (slot 26): `Entity type` (default), `Name or UUID`

### `StandingOn`
**Is Standing on Block** · aliases: `EStandingOn` · **wire `subAction` = alias** (name collides across if-blocks)
> ⚠ **LEGACY** → use `" StandingOn "`
Checks if an entity is standing on the given block or location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` _(plural)_ — Block to check for
   - **OR**
   - `[1]` `LOCATION` _(plural)_ — Location to check for

### `" StandingOn "`
**Is Standing on Block** · aliases: `" EStandingOn "` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if an entity is standing on the given block or location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` _(plural)_ — Block to check for
   - **OR**
   - `[1]` `LOCATION` _(plural)_ — Location to check for

### `IsGrounded`
**Is Grounded** · aliases: `EIsGrounded` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if an entity is supported by a block.

### `IsType`
**Is Type**
Checks if an entity is the given type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ENTITY_TYPE` _(plural)_ — Spawn egg, projectile, or vehicle

### `IsMob`
**Is Mob**
Checks if an entity is a mob.

### `IsCritical`
**Is Critical**
Checks if an arrow is critical. A critical arrow deals extra damage.
- _Works with:_ Arrows; Tridents

### `IsItem`
**Is Item**
Checks if an entity is an item.

### `" IsRiding "`
**Is Riding Entity** · aliases: `EIsRiding` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if an entity is riding another entity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ENTITY_TYPE` _(plural)_ — Spawn egg, projectile, or vehicle
   - **OR**
   - `[1]` `TEXT` _(plural)_ — Entity UUID
   - **OR**
   - `[2]` `NONE` — Checks if riding any entity)
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `Exists`
**Exists**
Checks if an entity still exists in the world.
- _Info:_ Entities which have been removed still remain in selections.

### `IsHitboxNear`
**Is Hitbox Near Location** · aliases: `EHitboxIsNear` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if an entity's hitbox is within a range of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Center location
   - `[1]` `NUMBER` _(optional)_ — Range — _Default = 5 blocks_
- **Tags:**
   - `Shape` (slot 26): `Sphere` (default), `Circle`, `Cube`, `Square`
       - `Sphere`: Checks if √(X²+Y²+Z²) is within the radius.
       - `Circle`: Checks if √(X²+Z²) is within the radius.
       - `Cube`: Checks if the distance on the X, Y or Z axis is within the radius.
       - `Square`: Checks if the distance on the X or Z axis is within the radius.

### `HasPotion`
**Has Potion Effect** · aliases: `EHasPotion` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if an entity has a potion effect of a certain type active.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `POTION` _(plural)_ — Effect(s) to check for
- **Tags:**
   - `Check Properties` (slot 25): `None` (default), `Amplifier`, `Duration`, `Amplifier and duration`
       - `None`: Checks only if the effect type matches.
       - `Amplifier`: The amplifier of the entity's effect must be equal or higher.
       - `Duration`: The duration of the entity's effect must be equal or higher.
       - `Amplifier and duration`: The amplifier and duration of the entity's effect must be equal or higher.
   - `Check Mode` (slot 26): `Has any effect` (default), `Has all effects`

### `NameEquals`
**Name Equals** · aliases: `ENameEquals` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if an entity's name or custom name is equal to the given text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — UUID to check for
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `IsWaxed`
**Is Waxed**
Checks if a copper golem is waxed.
- _Works with:_ Copper Golem

## Block `if_game` — If Game (22 actions in this file)

Used to execute the code inside it if a certain condition related to the plot is met.

### `SignHasTxt`
**Sign Contains Text**
> ⚠ **LEGACY** → use `" SignHasTxt "`
Checks if the sign at a location has the given text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Sign location
   - `[1]` `TEXT` _(plural)_ — Text to check for
- **Tags:**
   - `Sign Line` (slot 26): `1`, `2`, `3`, `4`, `All lines` (default)
   - `Check Mode` (slot 25): `Contains` (default), `Equals`

### `IsSigningBook`
**Is Signing Book**
Checks if the book in the event is currently being signed.
- _Works with:_ Player Edit Book Event

### `HasEntitiesName`
**Game Has Entities with Name**
Checks if entities with the given names are currently in the game.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Name or UUID
- **Tags:**
   - `Check Mode` (slot 26): `Has All Entities` (default), `Has Any Entity`

### `HasRoomForItem`
**Container Has Room For Item**
Checks if the container at a location has room for one or more items to be given.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - **OR**
   - `[1]` `ITEM` — Item(s) to check with
   - **OR**
   - `[2]` `NONE` — Checks for empty slot)
- **Tags:**
   - `Check Mode` (slot 26): `Has Room for Any Item` (default), `Has Room for All Items`

### `EventBlockEquals`
**Event Block Equals**
Checks if the block in a block related event is the given block.
- _Works with:_ Block Events; Click Events
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` _(plural)_ — Block(s) to check for

### `CommandEquals`
**Command Equals**
Checks if the command entered in the Command Event is equal to the given string.
- _Works with:_ Command Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — String(s) to check for
- **Tags:**
   - `Check Mode` (slot 26): `Check entire command` (default), `Check beginning`
   - `Ignore Case` (slot 25): `True` (default), `False`
- _Flags:_ rank: Emperor

### `EventItemEquals`
**Event Item Equals**
Checks if the item in a item related event is the given item.
- _Works with:_ Item Events; Right Click Event; Left Click Event; Change Slot Event; Place Block Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Item(s) to check for
- **Tags:**
   - `Comparison Mode` (slot 26): `Exactly equals`, `Ignore stack size/durability` (default), `Material only`

### `" SignHasTxt "`
**Sign Contains Text**
Checks if the sign at a location has the given text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Sign location
   - `[1]` `COMPONENT` _(plural)_ — Text to check for — _Formatting is ignored._
- **Tags:**
   - `Sign Line` (slot 26): `1`, `2`, `3`, `4`, `All lines` (default)
   - `Sign Side` (slot 25): `Front` (default), `Back`
   - `Check Mode` (slot 24): `Contains` (default), `Equals`

### `AttackIsCrit`
**Attack Is Critical**
Checks if an event attack is critical.

### `ContainerHas`
**Container Has Item**
Checks if the container at a location has the given item.
- _Info:_ If multiple items are in the chest, the container only needs to have one of them.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - `[1]` `ITEM` _(plural)_ — Item(s) to check for

### `DialogInputTrue`
**Dialog Input Is True**
Checks if a boolean dialog input is checked.
- _Works with:_ Player Click Dialog Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Input ID

### `HasEntitiesUUID`
**Game Has Entities with UUID**
Checks if entities with the given UUIDs are currently in the game.
- _Info:_ This action finds entities directly and is faster than Game Has Entities with Name.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — UUID
- **Tags:**
   - `Check Mode` (slot 26): `Has All Entities` (default), `Has Any Entity`

### `EventChunkNew`
**Is Chunk New**
Checks if the chunk being loaded in this event is a new chunk.
- _Works with:_ Chunk Load Event
- _Flags:_ world-plot only

### `BlockEquals`
**Block Equals** · aliases: `GBlockEquals` · **wire `subAction` = alias** (name collides across if-blocks)
Checks if the block at a location is the given block.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Check location
   - `[1]` `BLOCK` _(optional, plural)_ — Block(s) to check for
   - `[2]` `BLOCK_TAG` _(optional, plural)_ — Block data — _Example: "facing=up"_

### `InBlock`
**Location Is In Block**
Checks if a location collides with the hitbox of the nearest block.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Check location

### `MovementKey`
**Event Movement Key Equals**
Checks if specific movement keys changed state in the current event.
- _Works with:_ Movement Key Change Event
- **Tags:**
   - `Movement Key` (slot 25): `Forward (W)` (default), `Backward (S)`, `Left (A)`, `Right (D)`, `Jump (Space)`, `Sneak (Left Shift)`, `Sprint (Left Control)`
   - `Action` (slot 26): `Just pressed` (default), `Just released`
       - `Just pressed`: The key was just pressed in this event call.
       - `Just released`: The key was just released in this event call.

### `BlockPowered`
**Block Is Powered**
Checks if the block at a location is powered by redstone.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(plural)_ — Check location(s)
- **Tags:**
   - `Redstone Power Mode` (slot 26): `Direct power` (default), `Indirect power`

### `HasPlayer`
**Game Has Player**
Checks if there is currently a player in the game with one of the given names or UUIDs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Name or UUID

### `ContainerHasAll`
**Container Has All Items**
Checks if the container at a location has all of the given items.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Container location
   - `[1]` `ITEM` _(plural)_ — Item(s) to check for

### `CmdArgEquals`
**Command Argument Equals**
Checks if a part of the command entered in the Command Event is equal to the given string.
- _Works with:_ Command Event
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — String(s) to check for
   - `[1]` `NUMBER` — Argument number
- **Tags:**
   - `Ignore Case` (slot 26): `True` (default), `False`
- _Flags:_ rank: Emperor

### `EventCancelled`
**Event Is Cancelled**
Checks if the current event is cancelled.

### `IsChunkLoaded`
**Is Chunk Loaded**
Checks if the chunk at a location is currently loaded.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Location in chunk

## Block `if_var` — If Variable (34 actions in this file)

Used to execute the code inside it if a certain condition related to the value of a variable is met.

### `<=`
**Number ≤**
Checks if a number value is less than or equal to another number.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Number to check
   - `[1]` `NUMBER` — Number to compare to

### `ItemHasEnchant`
**Item Has Enchantment**
Checks if an item has a given enchantment, or, if no enchantment is specified, checks if it has any.
- _Info:_ A level of 0 will work for any level.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item to check
   - `[1]` `TEXT` _(optional)_ — Enchantment
   - `[2]` `NUMBER` _(optional)_ — Level

### `ItemIsBlock`
**Item Is Block**
Checks if an item is able to be placed.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item to check

### `DictValueEquals`
**Dictionary Value Equals**
Checks if a dictionary's value for the given key is equal to any of the given values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `DICT` — Dictionary to check
   - `[1]` `TEXT` — Key to check
   - `[2]` `ANY_TYPE` _(plural)_ — Values to compare with

### `ItemHasTag`
**Item Has Custom Tag**
Checks if an item has a given custom tag, and (if provided) whether the tag matches the given value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item to check
   - `[1]` `TEXT` — Tag name
   - **OR**
   - `[2]` `NUMBER` _(optional)_ — Tag value
   - **OR**
   - `[3]` `TEXT` _(optional)_ — Tag value

### `ListSizeEquals`
**List Size Equals** · aliases: `ListIsEmpty`
Checks if a list is of a given size.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LIST` — List to check
   - `[1]` `NUMBER` _(optional)_ — Size to compare — _Default = 0_

### `StringMatches`
**String Matches** · aliases: `" TextMatches "`
Checks if a string value matches other values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — String or source expression to match
   - `[1]` `TEXT` _(plural)_ — String to compare
- **Tags:**
   - `Regular Expressions` (slot 26): `Enable`, `Disable` (default)
   - `Ignore Case` (slot 25): `True`, `False` (default)

### `IsFiltered`
**String Is Filtered**
Checks if the input string would be filtered by the selected chat filters.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — String to check
- **Tags:**
   - `Link Filter` (slot 26): `True` (default), `False`
   - `Swear Filter` (slot 25): `True` (default), `False`
   - `Caps Filter` (slot 24): `True` (default), `False`
   - `Character Spacing Filter` (slot 23): `True` (default), `False`
   - `Character Drag Filter` (slot 22): `True` (default), `False`

### `StartsWith`
**String Starts With**
Checks if the first part of a string value matches a certain string.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — String to check
   - `[1]` `TEXT` _(plural)_ — String to start with
- **Tags:**
   - `Ignore Case` (slot 26): `True`, `False` (default)

### `ListValueEq`
**List Value Equals**
Checks if a list's value at an index is equal to a value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LIST` — List to check in
   - `[1]` `NUMBER` — Index to check at
   - `[2]` `ANY_TYPE` _(plural)_ — Variable to compare to

### `VarIsType`
**Value Is Type**
Checks if a value is of a certain type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ANY_TYPE` — Value to check
- **Tags:**
   - `Variable Type` (slot 26): `Number` (default), `String`, `Styled Text`, `Location`, `Item`, `List`, `Potion effect`, `Sound`, `Particle`, `Vector`, `Dictionary`

### `TextMatches`
**Text Matches** · aliases: `EqIgnoreCase`
> ⚠ **LEGACY** → use `StringMatches`
Checks if a text value matches other values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Text or source expression to match
   - `[1]` `TEXT` _(plural)_ — Text to compare
- **Tags:**
   - `Regular Expressions` (slot 26): `Enable`, `Disable` (default)
   - `Ignore Case` (slot 25): `True`, `False` (default)

### `IsNear`
**Variable Is Near** · aliases: `VIsNear` · **wire `subAction` = alias** (name collides across if-blocks)
> ⚠ **LEGACY** → use `" InRange "`
Checks if a number is within a certain range of another number or if a location is near another location.
- _Works with:_ Numbers; Locations
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ANY_TYPE` — Check value
   - `[1]` `ANY_TYPE` — Center value
   - `[2]` `NUMBER` — Range value

### `" InRange "`
**Value Is Within Range**
Checks if a number value is in between 2 other numbers or a location value is within the region of 2 other locations.
- _Works with:_ Number; Location
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ANY_TYPE` — Check value
   - `[1]` `ANY_TYPE` — Minimum value
   - `[2]` `ANY_TYPE` — Maximum value
- **Tags:**
   - `Location Handling` (slot 26): `Block`, `Exact` (default)
       - `Block`: Block locations are compared. Does not affect number ranges.
       - `Exact`: Exact decimal locations are compared. Does not affect number ranges.

### `ValueIsEmpty`
**Value Is Empty** · aliases: `VarIsEmpty`
Checks if a value is empty.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — String to check if empty
   - **OR**
   - `[1]` `COMPONENT` — Text to compare content length to 0
   - **OR**
   - `[2]` `LIST` — List to compare size to 0
   - **OR**
   - `[3]` `DICT` — Dictionary to compare size to 0
   - **OR**
   - `[4]` `ITEM` — Item to compare to air

### `VarExists`
**Variable Exists**
Checks if a variable exists.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to check

### `DictHasKeys`
**Dictionary Has Keys**
Checks if a dictionary has the given keys.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `DICT` — Dictionary to check
   - `[1]` `TEXT` _(plural)_ — Key(s) to look for
- **Tags:**
   - `Check Mode` (slot 26): `Has Any Key` (default), `Has All Keys`

### `Legacy !=`
**Value !=**
> ⚠ **LEGACY** → use `!=`

### `BlockIsSolid`
**Block Is Solid**
Checks if a material will collide with entities.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Block to check for

### `ItemEquals`
**Item Equals** · aliases: `VItemEquals` · **wire `subAction` = alias** (name collides across if-blocks)
Works the same as Value = but has a few extra options for item comparison.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item to check
   - `[1]` `ITEM` _(optional, plural)_ — Item(s) to compare to
- **Tags:**
   - `Comparison Mode` (slot 26): `Exactly equals` (default), `Ignore stack size`, `Ignore durability and stack size`, `Material only`

### `ListContains`
**List Contains Value**
Checks if any of a list's contents match the given value.
- _Info:_ If multiple values are given, the condition will return true if one of the values are found.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LIST` — List to check in
   - `[1]` `ANY_TYPE` _(plural)_ — Value to find
- **Tags:**
   - `Check Mode` (slot 26): `Has Any Value` (default), `Has All Values`

### `Legacy =`
**Value =**
> ⚠ **LEGACY** → use `=`
Checks if a value is equal to one of the given values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - **OR**
   - **OR**

### `IsTooltipVisible`
**Item Tooltip Is Visible**
Checks if an item's tooltip is visible.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item to check

### `InRange`
**Value Is Within Range**
> ⚠ **LEGACY** → use `" InRange "`
Checks if a number value is in between 2 other numbers or a location value is within the region of 2 other locations.
- _Works with:_ Number; Location
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ANY_TYPE` — Check value
   - `[1]` `ANY_TYPE` — Minimum value
   - `[2]` `ANY_TYPE` — Maximum value

### `LocIsNear`
**Location Is Near**
Checks if a location is near another location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Location to check
   - `[1]` `LOCATION` _(plural)_ — Location(s) to compare to
   - `[2]` `NUMBER` — Radius
- **Tags:**
   - `Shape` (slot 26): `Sphere` (default), `Circle`, `Cube`, `Square`
       - `Sphere`: Checks if √(X²+Y²+Z²) is within the radius.
       - `Circle`: Checks if √(X²+Z²) is within the radius.
       - `Cube`: Checks if the distance on the X, Y or Z axis is within the radius.
       - `Square`: Checks if the distance on the X or Z axis is within the radius.

### `ItemIsUnbreakable`
**Item Is Unbreakable**
Checks if an item is unbreakable.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Item to check

### `Contains`
**String Contains**
Checks if a string value contains another string.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — String to check
   - `[1]` `TEXT` _(plural)_ — String to check for
- **Tags:**
   - `Ignore Case` (slot 26): `True`, `False` (default)

### `!=`
**Value !=**
Checks if a value does not equal another value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ANY_TYPE` — Value to check
   - `[1]` `ANY_TYPE` _(plural)_ — Values to compare to

### `<`
**Number <**
Checks if a number value is less than another number.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Number to check
   - `[1]` `NUMBER` — Number to compare to

### `=`
**Value =** · aliases: `" = "`
Checks if a value is equal to one of the given values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ANY_TYPE` — Value to check
   - `[1]` `ANY_TYPE` _(plural)_ — Values to compare to

### `>`
**Number >**
Checks if a number value is greater than another number.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Number to check
   - `[1]` `NUMBER` — Number to compare to

### `EndsWith`
**String Ends With**
Checks if the last part of a string value matches a certain string.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — String to check
   - `[1]` `TEXT` _(plural)_ — String to end with
- **Tags:**
   - `Ignore Case` (slot 26): `True`, `False` (default)

### `>=`
**Number ≥**
Checks if a number value is greater than or equal to another number.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Number to check
   - `[1]` `NUMBER` — Number to compare to

### `DictHasKey`
**Dictionary Has Key**
Checks if a dictionary has the given key.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `DICT` — Dictionary to check
   - `[1]` `TEXT` — Key to look for
