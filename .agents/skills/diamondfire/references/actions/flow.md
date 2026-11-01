# DiamondFire actions — Flow, selection and structure (`control`, `repeat`, `select_obj`, `else`, `func`, `process`, `call_func`, `start_process`)

_Generated from actiondump (patch 8.13). Action names are exact wire strings; names in double quotes include **leading/trailing spaces** that must be kept._

## Block `control` — Control (8 actions in this file)

Used to control the execution of some or all code blocks after it.

### `StopRepeat`
**Stop Repeat**
Stops a Repeat sequence and continues to the next code block.
- _Works with:_ Repeat

### `Return`
**Return From Function**
Skips the rest of a Function sequence and returns to the block it was called from.
- _Info:_ This occurs automatically at the end of a Function.

### `PrintDebug`
**Send Debug Message**
Sends a formatted message to the specified plot staff group regardless of which mode they're currently in. Clicking on the message will teleport you to this block.
- _Info:_ Players must be on the plot to receive debug messages.
- _Info:_ Multiple values (of any type) will be merged together.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ANY_TYPE` _(optional, plural)_ — Message to format
- **Tags:**
   - `Permission` (slot 22): `Owner`, `Developer` (default), `Builder`, `Developer or builder`, `Whitelisted`, `All`
       - `Owner`: Sends to the plot owner. Works even if not on the plot.
       - `Developer`: Sends to players that have dev perms.
       - `Builder`: Sends to players that have build perms.
       - `Developer or builder`: Sends to players that have dev or build perms.
       - `Whitelisted`: Sends to players that are whitelisted on the plot.
       - `All`: Sends to all players that are on the plot.
   - `Text Value Merging` (slot 23): `Add Spaces` (default), `No Spaces`
   - `Highlighting` (slot 24): `None` (default), `Error`, `Warning`, `Other`
       - `None`: Don't highlight this code block.
       - `Error`: Highlight this code block red.
       - `Warning`: Highlight this code block orange.
       - `Other`: Highlight this code block white.
   - `Sound` (slot 25): `None`, `Default` (default), `Success`, `Error`, `Warning`, `LagSlayer`
       - `None`: No sound is played.
       - `Default`: Plays the default sound for the message style.
       - `Success`: Plays Player Level Up
       - `Error`: Plays Shulker Hurt (Closed)
       - `Warning`: Plays Blaze Hurt
       - `LagSlayer`: Plays Iron Golem Repair
   - `Message Style` (slot 26): `Custom`, `Debug` (default), `Error`, `Warning`, `Info`, `LagSlayer`
       - `Custom`: No message style is applied. Preview: Example
       - `Debug`: Sends a debug message. Preview: • Example
       - `Error`: Sends an error. Plays a sound by default. Preview: Error: Example
       - `Warning`: Sends a warning. Plays a sound by default. Preview: [Warning] Example
       - `Info`: Sends an info message. Preview: [Info] Example
       - `LagSlayer`: Sends a LagSlayer message. Plays a sound by default. Preview: [LagSlayer] Example

### `ReturnNTimes`

### `Skip`
**Skip Iteration**
Skips the rest of this repeat statement's code and continues to the next repetition.
- _Works with:_ Repeat

### `End`
**End Thread**
Stops the current event thread. Any code after this block will not be executed.
- _Info:_ This occurs automatically when a thread has no valid event targets or selection. A player target is invalid if they have left the game.

### `EndAllThreads`
**End All Threads**
Ends all currently active threads, including active lines, loops, etc.
- **Tags:**
   - `End Current Thread` (slot 26): `True` (default), `False`
       - `True`: All threads including this one will be ended.
       - `False`: All threads except this one will be ended.

### `Wait`
**Wait**
Pauses the current code sequence for a duration of ticks, seconds, or minutes.
- _Info:_ It is not possible to wait fractions of a tick.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Wait duration — _Default = 1_
- **Tags:**
   - `Time Unit` (slot 26): `Ticks` (default), `Seconds`, `Minutes`

## Block `repeat` — Repeat (12 actions in this file)

Used to repeat the code inside it.

### `Adjacent`
**Repeat Adjacently**
Repeats code once for each block adjacent to a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Gets the current block location each iteration
   - `[1]` `LOCATION` — Center block
- **Tags:**
   - `Change Location Rotation` (slot 24): `True`, `False` (default)
   - `Include Origin Block` (slot 25): `True`, `False` (default)
   - `Pattern` (slot 26): `Cardinal (4 blocks)`, `Square (8 blocks)`, `Adjacent (6 blocks)` (default), `Cube (26 blocks)`

### `Path`
**Repeat On Path**
Repeats code once for each interpolated point in a path of locations.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Gets the current path location each iteration
   - `[1]` `LOCATION` _(plural)_ — Path locations
   - `[2]` `NUMBER` _(optional)_ — Point spacing — _Default = 0.5 blocks_
- **Tags:**
   - `Rotate Location` (slot 26): `True`, `False` (default)
       - `True`: Sets the rotation of each returned location to face the next given path point.
       - `False`: Keeps the rotation of the given path points.

### `Multiple`
**Repeat Multiple Times** · aliases: `N Times`
Repeats code multiple times.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` _(optional)_ — Gets the current index each iteration
   - `[1]` `NUMBER` — Amount

### `Grid`
**Repeat On Grid**
Repeats code once for each block in a region in order: X → Z → Y. Iterates from the first to the second location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Gets the current block location each iteration
   - `[1]` `LOCATION` — Start of region
   - `[2]` `LOCATION` — End of region

### `DoWhile`
**Repeat Do While**
Repeats code as long as a condition is true. The condition is evaluated at the end of each loop iteration.
- Condition sub-blocks allowed: `if_player`, `if_entity`, `if_var`, `if_game`

### `While`
**Repeat While** · aliases: `WhileCond`
Repeats code as long as a condition is true. The condition is evaluated before each loop iteration.
- Condition sub-blocks allowed: `if_player`, `if_entity`, `if_var`, `if_game`

### `Range`
**Repeat On Range**
> ⚠ **LEGACY** → use `" Range "`
Repeats code once for each number on a number line.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` _(optional)_ — Gets the current number each iteration
   - `[1]` `NUMBER` — Start of range
   - `[2]` `NUMBER` — End of range
   - `[3]` `NUMBER` _(optional)_ — Interval — _Default = 1_

### `ForEach`
**Repeat For Each in List**
Repeats code once for each index of a list.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Gets the current value each iteration
   - `[1]` `LIST` — List to repeat through
- **Tags:**
   - `Allow List Changes` (slot 26): `True` (default), `False (copy list)`

### `Sphere`
**Repeat On Sphere**
Repeats code once for every evenly distributed sphere point.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Gets the current sphere location each iteration
   - `[1]` `LOCATION` — Sphere center
   - `[2]` `NUMBER` — Sphere radius
   - `[3]` `NUMBER` _(optional)_ — Sphere points
- **Tags:**
   - `Point Locations Inwards` (slot 26): `True`, `False` (default)

### `Forever`
**Repeat Forever**
Repeats code indefinitely.
- _Info:_ The Control: Wait block can be used for a delay.

### `" Range "`
**Repeat On Range**
Repeats code once for each number on a number line.
- _Info:_ To iterate in the negative direction, you must use a negative step size.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` _(optional)_ — Gets the current number each iteration
   - `[1]` `NUMBER` — Start of range
   - `[2]` `NUMBER` — End of range
   - `[3]` `NUMBER` _(optional)_ — Step — _Default = 1_

### `ForEachEntry`
**Repeat For Each Dictionary Entry**
Repeats code once per entry in a dictionary
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Gets the current key each iteration
   - `[1]` `VARIABLE` — Gets the current value each iteration
   - `[2]` `DICT` — Dictionary to repeat through

## Block `select_obj` — Select Object (29 actions in this file)

Used to change the selection on the current line of code, which will affect the targets of most code blocks.

### `LastMob`

### `RandomPlayer`
**Select Random Player**
Creates a selection using one or more random players in the game.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Selection size — _Default = 1_

### `LastEntity`
**Select Last-Spawned Entity**
Creates a selection using the most recently spawned entity.

### `Shooter`
**Select Shooter**
> ⚠ **LEGACY** → use `EventTarget` (Set Event Target to Shooter.)
Selects the shooter in a projectile-related event.
- _Works with:_ Damage Events; Kill Events; Player Projectile Hit Event

### `AllMobs`
**Select All Mobs**
> ⚠ **LEGACY** → use `EntitiesCond` (Filter by If Entity: Is Mob.)
Selects all mobs on the plot.

### `EntityName`
**Select Entities by Name**
Creates a selection using all entities in the game whose name or UUID matches.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — UUID to check for
   - **OR**
   - `[1]` `COMPONENT` _(plural)_ — Name to check for
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `FilterRandom`
**Filter Selection Randomly** · aliases: `RandomSelected`
Filters the selection by randomly picking one or more objects from it.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Selection size — _Default = 1_

### `EntityUUID`
**Select Entities by UUID**
Creates a selection of all entities in the game with the given UUIDs.
- _Info:_ This action selects entities directly and is much faster than normal selections.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — UUID(s) to check for

### `DefaultEntity`
**Select Default Entity**
> ⚠ **LEGACY** → use `EventTarget` (Set Event Target to Default Entity.)
Selects the main entity involved in the current event, or the last spawned entity if nothing else is applicable.

### `PlayerName`
**Select Players by Name**
Creates a selection using all players in the game whose name or UUID matches.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Name or UUID

### `AllEntities`
**Select All Entities**
Creates a selection of all entities in the game.

### `Damager`
**Select Damager**
> ⚠ **LEGACY** → use `EventTarget` (Set Event Target to Damager.)
Selects the damager in a damage-related event. The damager can be a player or an entity.
- _Works with:_ Damage Events

### `FilterDistance`
**Filter Selection by Distance**
Filters the selection to the objects that are nearest or farthest to a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Location to compare to
   - `[1]` `NUMBER` _(optional)_ — Selection size — _Default = 1_
- **Tags:**
   - `Ignore Y-Axis` (slot 25): `True`, `False` (default)
   - `Compare Mode` (slot 26): `Nearest` (default), `Farthest`

### `FilterRay`
**Filter Selection by Raycast**
Filters the selected objects to the objects that intersect a ray.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` _(optional)_ — Gets the end or final hit location
   - `[1]` `LOCATION` — Ray origin
   - `[2]` `NUMBER` — Ray distance
   - `[3]` `NUMBER` _(optional)_ — Ray width — _Default = 0.0_
   - `[4]` `NUMBER` _(optional)_ — Selection size — _Default = 1_
- **Tags:**
   - `Block Collision` (slot 26): `All blocks`, `Non-fluid blocks`, `Solid blocks` (default), `None`
       - `All blocks`: Passes through air only.
       - `Non-fluid blocks`: Passes through fluids (water and lava).
       - `Solid blocks`: Passes through non-solid blocks such as tall grass, and through fluids.
       - `None`: Passes through all blocks.

### `Reset`
**Reset Selection** · aliases: `None`
Deactivates the selection. Code that follows will run normally with event targets.

### `EventTarget`
**Select Event Target**
Creates a selection using a target involved in this Event.
- **Tags:**
   - `Event Target` (slot 26): `Default` (default), `Killer`, `Damager`, `Victim`, `Shooter`, `Projectile`
       - `Default`: The target that triggered this event.
       - `Killer`: The target that killed the victim in this event.
       - `Damager`: The target that harmed the victim in this event.
       - `Victim`: The target that was harmed or killed in this event.
       - `Shooter`: The target that fired a projectile in this event.
       - `Projectile`: The projectile in this event.

### `Killer`
**Select Killer**
> ⚠ **LEGACY** → use `EventTarget` (Set Event Target to Killer.)
Selects the killer in a kill-related event. The killer can be a player or an entity.
- _Works with:_ Kill Events

### `Victim`
**Select Victim**
> ⚠ **LEGACY** → use `EventTarget` (Set Event Target to Victim.)
Selects the victim in a kill-related or damage-related event. The victim can be a player or an entity.
- _Works with:_ Damage Events; Kill Events; Player Click Player Event; Player Click Entity Event

### `EntitiesCond`
**Select Entities by Condition**
Creates a selection of all entities in the game that meet a condition.
- _Info:_ Can be inverted with the NOT Arrow.
- Condition sub-blocks allowed: `if_entity`, `if_var`, `if_game`

### `AllPlayers`
**Select All Players**
Creates a selection of all players in the game.

### `Invert`
**Invert Selection**
Creates a new selection by inverting the selection that is currently active.
- _Info:_ If the current selection contains players, all other players are selected.
- _Info:_ If the current selection contains entities, all other entities are selected.

### `RandomEntity`
**Select Random Entity**
> ⚠ **LEGACY** → use `AllEntities` (Then use Filter Selection Randomly.)
Selects a random entity.
- _Info:_ Does not select players.

### `FilterCondition`
**Filter Selection by Condition** · aliases: `FilterSelect`
Filters the selection to the objects that meet a certain condition.
- _Info:_ Can be inverted with the NOT Arrow.
- Condition sub-blocks allowed: `if_player`, `if_entity`, `if_var`, `if_game`

### `MobsCond`
**Select Mobs by Condition**
> ⚠ **LEGACY** → use `EntitiesCond` (Filter by If Entity: Is Mob, then filter by another condition.)
Creates a selection of all entities in the game that are mobs and meet a condition.
- _Info:_ Can be inverted with the NOT Arrow.
- Condition sub-blocks allowed: `if_entity`, `if_var`, `if_game`

### `FilterSort`
**Filter Selection by Sort**
Filters the selection by sorting the value of each object in order.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ANY_TYPE` — Value to compare
   - `[1]` `NUMBER` _(optional)_ — Selection size — _Default = 1_
- **Tags:**
   - `Sort Order` (slot 26): `Ascending` (default), `Descending`

### `Projectile`
**Select Projectile**
> ⚠ **LEGACY** → use `EventTarget` (Set Event Target to Projectile.)
Selects the projectile in a projectile-related event.
- _Works with:_ Projectile Damage Player Event; Player Projectile Hit Event; Projectile Damage Mob Event; Projectile Kill Mob Event

### `DefaultPlayer`
**Select Default Player**
> ⚠ **LEGACY** → use `EventTarget` (Set Event Target to Default Player.)
Selects the main player involved in the current event.
- _Works with:_ Player Events; Processes

### `PlayersCond`
**Select Players by Condition**
Creates a selection of all players in the game that meet a condition.
- _Info:_ Can be inverted with the NOT Arrow.
- Condition sub-blocks allowed: `if_player`, `if_var`, `if_game`

### `MobName`
**Select Mobs by Name**
> ⚠ **LEGACY** → use `EntityName`
Selects all mobs whose names are equal to the given text.
- _Info:_ Works with UUIDs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Name of mobs to select

## Block `else` — Else (0 actions in this file)

Used to execute the code inside it if the condition directly before it was not met. Else must be placed directly after any If block's closing bracket.

## Block `func` — Function (1 actions in this file)

Used to define a line of code that can be called with a Call Function block.

- Set the function name by right clicking the code block sign with a string.
- Set the name color, icon, and lore of the function by putting an item in the chest.
- Put parameter items to specify the parameters of the function.
- Functions can also be used for continuing lines of code that are out of space.

### `dynamic`
- **Tags:**
   - `Is Hidden` (slot 26): `True`, `False` (default)

## Block `process` — Process (1 actions in this file)

Used to execute code when the process is started using a Start Process block.

- Set the process name by right clicking the code block sign with a string.
- Set the name color, icon, and lore of the function by putting an item in the chest.
- Put parameter items to specify the parameters of the process.
- Processes are not a replacement for Functions. Processes are much more CPU intensive than functions, and should only be used when needed.

### `dynamic`
- **Tags:**
   - `Is Hidden` (slot 26): `True`, `False` (default)

## Block `call_func` — Call Function (1 actions in this file)

Used to call functions declared by a Function block. Code will not continue past this block until the Function completes/returns.

### `dynamic`

## Block `start_process` — Start Process (1 actions in this file)

Used to start processes declared by a Process block. Waits inside the Process will not stop this code line, unlike Functions.

### `dynamic`
- **Tags:**
   - `Target Mode` (slot 26): `With current targets` (default), `With current selection`, `With no targets`, `For each in selection`
       - `With current targets`: Starts a process with the same event targets as the current thread.
       - `With current selection`: Starts a process with the same selection as the current thread. This process requires no targets to keep running.
       - `With no targets`: Starts a process without any targets or selection. This process requires no targets to keep running.
       - `For each in selection`: Starts multiple processes; one for each target in the current selection. This is the default target of each started process.
   - `Local Variables` (slot 25): `Don't copy` (default), `Copy`, `Share`
