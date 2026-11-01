# DiamondFire actions — Set Variable (`set_var`), part 2

_Generated from actiondump (patch 8.13). Action names are exact wire strings; names in double quotes include **leading/trailing spaces** that must be kept._

## Block `set_var` — Set Variable (178 actions in this file)

Used to set the value of a variable.

### `SetCoords`
**Set to Coordinates/Location**
> ⚠ **LEGACY** → use `SetAllCoords`
Sets a variable to a set of coordinates (location).
- _Info:_ Locations can be used instead of numbers for values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — X Coordinate
   - `[2]` `NUMBER` — Y Coordinate
   - `[3]` `NUMBER` — Z Coordinate
   - `[4]` `NUMBER` _(optional)_ — Pitch (-90 to 90) — _-90 = up, 90 = down_
   - `[5]` `NUMBER` _(optional)_ — Yaw (-180 to 180) — _0 = south, 90 = west, etc._

### `GetParticleSize`
**Get Particle Effect Size**
Gets a particle effect's particle size.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get size of
- **Returns:** `NUMBER` Particle size

### `SetParticlePower`
**Set Particle Effect Power**
Sets a particle effect's power.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `NUMBER` — Particle power

### `SetBundleItems`
**Set Bundle Contents**
Sets the contents of a bundle.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Bundle
   - `[2]` `ITEM` _(optional, plural)_ — Contents

### `SetParticleType`
**Set Particle Effect Type**
> ⚠ **LEGACY** → use `" GetParticleType "` (Particles are now represented using their key identifiers.)
Sets a particle effect's type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `TEXT` — Type

### `SetItemEnchants`
**Set Item Enchantments**
> ⚠ **LEGACY** → use `" SetItemEnchants "`
Sets an item's enchantment list.
- _Info:_ Refer to the format of the 'Get Item Enchantments' action.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `LIST` — Enchantments

### `DialogBodyPlain`
**Add Plain Message Dialog Body**
Adds a plain message dialog body element to a dialog.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `COMPONENT` — Text to display
   - `[2]` `NUMBER` _(optional)_ — Element width — _Default = 200 (1-1024)_

### `ClearItemTag`
**Clear Item Custom Tags**
Removes all item custom tags.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change

### `" AddItemAttribute "`
**Add Item Attribute**
Adds an attribute modifier to an item, which is active in a certain equipment slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
   - `[2]` `NUMBER` — Modifier Amount
- **Tags:**
   - `Attribute` (slot 24): `Air drag modifier` (default), `Armor`, `Armor toughness`, `Attack damage`, `Attack knockback`, `Attack speed`, `Bounciness`, `Burning time`, `Camera distance`, `Explosion knockback resistance`, `Fall damage multiplier`, `Flying speed`, `Follow range`, `Friction modifier`, `Gravity`, `Jump strength`, `Knockback resistance`, `Luck`, `Maximum absorption health`, `Maximum health`, `Movement efficiency`, `Name tag distance`, `Walking speed`, `Oxygen bonus`, `Safe fall distance`, `Scale`, `Step height`, `Water movement efficiency`, `Tempt range`, `Block break speed`, `Block interaction range`, `Entity interaction range`, `Mining efficiency`, `Sneaking speed`, `Submerged mining speed`, `Sweeping damage ratio`, `Zombie spawn reinforcements`
       - `Air drag modifier`: Base: 1.0 Range: 0.0-2048.0
       - `Armor`: Base: 0.0 Range: 0.0-30.0
       - `Armor toughness`: Base: 0.0 Range: 0.0-20.0
       - `Attack damage`: Base: 2.0 Range: 0.0-2048.0
       - `Attack knockback`: Base: 0.0 Range: 0.0-5.0
       - `Attack speed`: Base: 4.0 Range: 0.0-1024.0
       - `Bounciness`: Base: 0.0 Range: 0.0-1.0
       - `Burning time`: Base: 1.0 Range: 0.0-1024.0
       - `Camera distance`: Base: 4.0 Range: 0.0-32.0
       - `Explosion knockback resistance`: Base: 0.0 Range: 0.0-1.0
       - `Fall damage multiplier`: Base: 1.0 Range: 0.0-100.0
       - `Flying speed`: Base: 0.4 Range: 0.0-1024.0
       - `Follow range`: Base: 32.0 Range: 0.0-2048.0
       - `Friction modifier`: Base: 1.0 Range: 0.0-2048.0
       - `Gravity`: Base: 0.08 Range: -1.0-1.0
       - `Jump strength`: Base: 0.42 Range: 0.0-32.0
       - `Knockback resistance`: Base: 0.0 Range: 0.0-1.0
       - `Luck`: Base: 0.0 Range: -1024.0-1024.0
       - `Maximum absorption health`: Base: 0.0 Range: 0.0-2048.0
       - `Maximum health`: Base: 20.0 Range: 1.0-1024.0
       - `Movement efficiency`: Base: 0.0 Range: 0.0-1.0
       - `Name tag distance`: Base: 64.0 Range: 0.0-512.0
       - `Walking speed`: Base: 0.2 Range: 0.0-1024.0
       - `Oxygen bonus`: Base: 0.0 Range: 0.0-1024.0
       - `Safe fall distance`: Base: 3.0 Range: -1024.0-1024.0
       - `Scale`: Base: 1.0 Range: 0.0625-16.0
       - `Step height`: Base: 0.6 Range: 0.0-10.0
       - `Water movement efficiency`: Base: 0.0 Range: 0.0-1.0
       - `Tempt range`: Base: 10.0 Range: 0.0-2048.0
       - `Block break speed`: Base: 1.0 Range: 0.0-1024.0
       - `Block interaction range`: Base: 4.5 Range: 0.0-64.0
       - `Entity interaction range`: Base: 3.0 Range: 0.0-64.0
       - `Mining efficiency`: Base: 0.0 Range: 0.0-1024.0
       - `Sneaking speed`: Base: 0.3 Range: 0.0-1.0
       - `Submerged mining speed`: Base: 0.2 Range: 0.0-20.0
       - `Sweeping damage ratio`: Base: 0.0 Range: 0.0-1.0
       - `Zombie spawn reinforcements`: Base: 0.0 Range: 0.0-1.0
   - `Operation` (slot 25): `Add number` (default), `Add percentage to base`, `Multiply modifier by percentage`
       - `Add number`: Increases the attribute value by the given amount.  Equation: mod + (amount)
       - `Add percentage to base`: Increases the base attribute value by the given percentage.  Equation: base * (100% + amount)
       - `Multiply modifier by percentage`: Increases the attribute value by the given percentage  Equation: mod * (100% + amount)
   - `Active Equipment Slot` (slot 26): `Any`, `Main hand` (default), `Off hand`, `Either hand`, `Head`, `Chest`, `Legs`, `Feet`, `Body`, `Saddle`
- _Flags:_ advanced

### `PurgeVars`
**Purge Matching Variables**
Clears all variables with names that match the given text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Name to match
- **Tags:**
   - `Match Requirement` (slot 25): `Entire name`, `Full word(s) in name` (default), `Any part of name`
   - `Ignore Case` (slot 26): `True`, `False` (default)

### `ShiftAllAxes`
**Shift Location on All Axes** · aliases: `ShiftLocCoord`
Shifts a location's coordinates on the X, Y, and Z axes.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `NUMBER` _(optional)_ — X Change
   - `[3]` `NUMBER` _(optional)_ — Y Change
   - `[4]` `NUMBER` _(optional)_ — Z Change

### `AbsoluteValue`
**Set to Absolute Value**
Makes a number positive if it is negative.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Number input
- **Returns:** `NUMBER` Result

### `ShiftOnVector`
**Shift Location on Vector** · aliases: `AddVectorLoc`
Shifts a location along a vector.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to shift
   - `[2]` `VECTOR` — Shift vector
   - **OR**
   - `[3]` `NUMBER` — Shift distance
   - **OR**
   - `[4]` `NONE` — Adds full vector)
- **Tags:**
   - `Add Location Rotation` (slot 26): `True`, `False` (default)
       - `True`: The location's rotation will be added to the vector's rotation.
       - `False`: The location's rotation will not be added to the vector's rotation.

### `SegmentString`
**Segment String**
Segments a string into chunks of a specified size.
- _Info:_ Unlike Split String, emojis and other special characters stay intact.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — String to segment
   - `[2]` `NUMBER` _(optional)_ — Segment size — _Default = 1_
- **Returns:** `LIST` Contains one String for each segment

### `ShiftRotation`
**Shift Location Rotation** · aliases: `RotateLocation`
Rotates a location by shifting its pitch (up/down) or yaw (left/right) value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to shift
   - `[2]` `NUMBER` — Rotation amount
- **Tags:**
   - `Rotation Axis` (slot 26): `Pitch` (default), `Yaw`

### `RemoveDialogBody`
**Remove Dialog Body By Index**
Removes a dialog body element by its index.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `NUMBER` — Index

### `Bitwise`
**Set to Bitwise Operation**
Sets a variable to the result of a bitwise operation.
- _Info:_ Input values are treated as whole numbers. Decimal value is ignored.
- _Info:_ Numbers are stored as 64-bit signed integers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Operand 1
   - `[2]` `NUMBER` _(optional)_ — Operand 2
- **Tags:**
   - `Operator` (slot 26): `|` (default), `&`, `~`, `^`, `<<`, `>>`, `>>>`
       - `|`: Bitwise Or
       - `&`: Bitwise And
       - `~`: Bitwise Not (no 2nd operand required)
       - `^`: Bitwise Xor
       - `<<`: Bitwise Left Shift
       - `>>`: Bitwise Right Shift
       - `>>>`: Bitwise Unsigned Right Shift
   - `Bit Precision` (slot 25): `Default` (default), `64-bit`
       - `Default`: Default precision (long / 1000)
       - `64-bit`: 64-bit precision (long), numbers are "multiplied" by 1000
- **Returns:** `NUMBER` Result
- _Flags:_ advanced

### `ParseX`
**Parse X Coordinate**
> ⚠ **LEGACY** → use `GetCoord` (Set Coordinate tag to X.)
Sets a variable to the block X coordinate of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location

### `ParseY`
**Parse Y Coordinate**
> ⚠ **LEGACY** → use `GetCoord` (Set Coordinate tag to Y.)
Sets a variable to the block Y coordinate of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location

### `ParseZ`
**Parse Z Coordinate**
> ⚠ **LEGACY** → use `GetCoord` (Set Coordinate tag to Z.)
Sets a variable to the block Z coordinate of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location

### `AddItemAttribute`
**Add Item Attribute**
Adds an attribute modifier to an item, which is active in a certain equipment slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
   - `[2]` `NUMBER` — Modifier Amount
- **Tags:**
   - `Attribute` (slot 24): `Air drag modifier` (default), `Armor`, `Armor toughness`, `Attack damage`, `Attack knockback`, `Attack speed`, `Bounciness`, `Burning time`, `Camera distance`, `Explosion knockback resistance`, `Fall damage multiplier`, `Flying speed`, `Follow range`, `Friction modifier`, `Gravity`, `Jump strength`, `Knockback resistance`, `Luck`, `Maximum absorption health`, `Maximum health`, `Movement efficiency`, `Name tag distance`, `Walking speed`, `Oxygen bonus`, `Safe fall distance`, `Scale`, `Step height`, `Water movement efficiency`, `Tempt range`, `Block break speed`, `Block interaction range`, `Entity interaction range`, `Mining efficiency`, `Sneaking speed`, `Submerged mining speed`, `Sweeping damage ratio`, `Zombie spawn reinforcements`
       - `Air drag modifier`: Base: 1.0 Range: 0.0-2048.0
       - `Armor`: Base: 0.0 Range: 0.0-30.0
       - `Armor toughness`: Base: 0.0 Range: 0.0-20.0
       - `Attack damage`: Base: 2.0 Range: 0.0-2048.0
       - `Attack knockback`: Base: 0.0 Range: 0.0-5.0
       - `Attack speed`: Base: 4.0 Range: 0.0-1024.0
       - `Bounciness`: Base: 0.0 Range: 0.0-1.0
       - `Burning time`: Base: 1.0 Range: 0.0-1024.0
       - `Camera distance`: Base: 4.0 Range: 0.0-32.0
       - `Explosion knockback resistance`: Base: 0.0 Range: 0.0-1.0
       - `Fall damage multiplier`: Base: 1.0 Range: 0.0-100.0
       - `Flying speed`: Base: 0.4 Range: 0.0-1024.0
       - `Follow range`: Base: 32.0 Range: 0.0-2048.0
       - `Friction modifier`: Base: 1.0 Range: 0.0-2048.0
       - `Gravity`: Base: 0.08 Range: -1.0-1.0
       - `Jump strength`: Base: 0.42 Range: 0.0-32.0
       - `Knockback resistance`: Base: 0.0 Range: 0.0-1.0
       - `Luck`: Base: 0.0 Range: -1024.0-1024.0
       - `Maximum absorption health`: Base: 0.0 Range: 0.0-2048.0
       - `Maximum health`: Base: 20.0 Range: 1.0-1024.0
       - `Movement efficiency`: Base: 0.0 Range: 0.0-1.0
       - `Name tag distance`: Base: 64.0 Range: 0.0-512.0
       - `Walking speed`: Base: 0.2 Range: 0.0-1024.0
       - `Oxygen bonus`: Base: 0.0 Range: 0.0-1024.0
       - `Safe fall distance`: Base: 3.0 Range: -1024.0-1024.0
       - `Scale`: Base: 1.0 Range: 0.0625-16.0
       - `Step height`: Base: 0.6 Range: 0.0-10.0
       - `Water movement efficiency`: Base: 0.0 Range: 0.0-1.0
       - `Tempt range`: Base: 10.0 Range: 0.0-2048.0
       - `Block break speed`: Base: 1.0 Range: 0.0-1024.0
       - `Block interaction range`: Base: 4.5 Range: 0.0-64.0
       - `Entity interaction range`: Base: 3.0 Range: 0.0-64.0
       - `Mining efficiency`: Base: 0.0 Range: 0.0-1024.0
       - `Sneaking speed`: Base: 0.3 Range: 0.0-1.0
       - `Submerged mining speed`: Base: 0.2 Range: 0.0-20.0
       - `Sweeping damage ratio`: Base: 0.0 Range: 0.0-1.0
       - `Zombie spawn reinforcements`: Base: 0.0 Range: 0.0-1.0
   - `Operation` (slot 25): `Add number` (default), `Add percentage to base`, `Multiply modifier by percentage`
       - `Add number`: Increases the attribute value by the given amount.  Equation: mod + (amount)
       - `Add percentage to base`: Increases the base attribute value by the given percentage.  Equation: base * (100% + amount)
       - `Multiply modifier by percentage`: Increases the attribute value by the given percentage  Equation: mod * (100% + amount)
   - `Active Equipment Slot` (slot 26): `Any`, `Main hand` (default), `Off hand`, `Head`, `Body`, `Legs`, `Feet`
- _Flags:_ advanced

### `GetCenterLoc`
**Set to Center Location** · aliases: `FindCenter`
Finds an average position (center) of the given locations.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(plural)_ — Locations to center
- **Returns:** `LOCATION` Center location

### `" SetPotionType "`
**Set Potion Effect Type**
Sets a potion effect's type.
- _Info:_ Keys taken are the same as the ones used in /potion get.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `POTION` _(optional)_ — Potion to change
   - `[2]` `TEXT` — Potion key (e.g. "minecraft:speed")

### `RemDialogButtonID`
**Remove Dialog Input By ID**
Removes a dialog body element by its ID.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `TEXT` — ID — _Can only use lowercase characters, numbers, and '_'_

### `GetSoundVolume`
**Get Sound Volume**
Gets a sound's volume.
- _Info:_ Sound volume decreases based on distance to the sound's origin.
- _Info:_ The maximum distance to hear a sound is equal to 16.0 × volume.
- _Info:_ At volumes below 1.0, the sound's loudness at its origin decreases.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` — Sound to get volume of
- **Returns:** `NUMBER` Sound volume

### `RandomNumber`
**Set to Random Number**
Sets a variable to a random number between two other numbers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Minimum number
   - `[2]` `NUMBER` — Maximum number
- **Tags:**
   - `Rounding Mode` (slot 26): `Whole number` (default), `Decimal number`
- **Returns:** `NUMBER` Random number

### `GzipDecompress`
**Decompress from Gzip**
Decompresses Gzip-compressed bytes as a list of numbers.
- _Info:_ Returns 0 if the result is too large.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `BYTE` _(plural)_ — Byte(s) to decompress
- **Tags:**
   - `Signed` (slot 26): `True` (default), `False`
- **Returns:** `LIST` Bytes as a list of numbers
- _Flags:_ advanced

### `ContainerName`
**Get Container Name**
Gets a container's name at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Container location
- **Returns:** `COMPONENT` Container name

### `SetConsumable`
**Set Consumable Properties**
Adds consumable properties to an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `SOUND` _(optional)_ — Consuming sound
   - `[3]` `NUMBER` _(optional)_ — Nutrition
   - `[4]` `NUMBER` _(optional)_ — Saturation
   - `[5]` `NUMBER` _(optional)_ — Use duration (seconds) — _Default = 1.6_
- **Tags:**
   - `Can Always Eat` (slot 26): `True`, `False` (default)
   - `Show Particles` (slot 25): `True` (default), `False`
   - `Consuming Animation` (slot 24): `None`, `Eat` (default), `Drink`, `Block`, `Bow`, `Trident`, `Crossbow`, `Spyglass`, `Toot horn`, `Brush`, `Bundle`, `Spear`

### `LoadBucket`
**Load Variable Bucket**
Loads a given bucket, the bucket stays loaded until it is unloaded with Set Variable: Save & Unload Bucket
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Load result
   - `[1]` `TEXT` — Bucket key
   - `[2]` `TEXT` _(optional)_ — Namespace alias — _Default = Default alias_
- **Returns:** `TEXT` Load result "success" if the bucket was successfully loaded. Any of the following if it failed: "active_lock", "bucket_key_too_long", "namespace_bucket_limit_reached", "unknown_namespace", "no_access", "loaded_buckets_limit_reached", "selected_more_than_one_player", "internal", "throttled"
- _Flags:_ advanced

### `GetBlockData`
**Get Block Data**
Gets a block state tag value at a location.
- _Info:_ Sets the variable to "0" if the tag is not present on the block.
- _Info:_ Always returns a string value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Block location
   - `[2]` `TEXT` — Tag name
- **Returns:** `TEXT` Tag value

### `CrossProduct`
**Set to Cross Product**
Sets a variable to the cross product of two vectors.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` — Vector 1
   - `[2]` `VECTOR` — Vector 2
- **Returns:** `VECTOR` Cross product

### `GetParticleRoll`
**Get Particle Effect Roll**
Gets a particle effect's roll.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get roll of
- **Returns:** `NUMBER` Particle roll

### `Noise`
**Get Noise**
Gets a seeded noise value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Noise location
   - `[2]` `NUMBER` _(optional)_ — Frequency — _Modifies the scale that the noise generates at. Default = 1_
   - `[3]` `NUMBER` _(optional)_ — Octaves — _Adds an additional layer of noise to the output. 1-16, Default = 1_
   - `[4]` `NUMBER` _(optional)_ — Lacunarity — _Every time a new octave is added, the frequency is multiplied by this value. Default = 1.5_
   - `[5]` `NUMBER` _(optional)_ — Gain — _Every time a new octave is added, the weight of it is multiplied by this value. Default = 0.5_
   - `[6]` `NUMBER` _(optional)_ — Generation seed
- **Tags:**
   - `Return Type` (slot 26): `Simplex` (default), `Perlin`, `Voronoi`, `Worley`, `Value`
       - `Simplex`: Provides the average of a honeycomb of gradient vectors.  Using a honeycomb instead of a lattice like that used in Perlin Noise, removes directional artifacts and increases efficiency.
       - `Perlin`: Provides the average of a lattice of gradient vectors.  Using vectors rather than values like those seen in Value Noise, provides more detail at the cost of lower efficiency.
       - `Voronoi`: Provides the identifier of the closest cell with cellular noise.
       - `Worley`: Provides the distance from the closest cell origin with cellular noise.
       - `Value`: Provides the average of a lattice (grid) of random values.
   - `Dimensions` (slot 25): `3D` (default), `2D`
       - `3D`: Generates using the X, Y and Z axis.
       - `2D`: Generates using the X and Z axis only.
- **Returns:** `NUMBER` A decimal value between 0 and 1
- _Flags:_ advanced

### `MinNumber`
**Set to Minimum Number** · aliases: `MinValue`, `Min`
Sets a variable to the lowest number in a set.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(plural)_ — Number set to compare
- **Returns:** `NUMBER` Result

### `SetItemName`
**Set Item Name**
> ⚠ **LEGACY** → use `" SetItemName "`
Sets an item's name.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `COMPONENT` _(plural)_ — Name

### `ListLength`
**Get List Length**
Gets the number of values a list has.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` — List to measure
- **Returns:** `NUMBER` List length

### `Sine`
**Set to Sine**
Sets a variable to the trigonometric sine function of a number.
- _Info:_ Use the variant tag to swap between sin, asin and sinh functions.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Number input
- **Tags:**
   - `Sine Variant` (slot 25): `Sine` (default), `Inverse sine (arcsine)`, `Hyperbolic sine`
   - `Input` (slot 26): `Degrees` (default), `Radians`
- **Returns:** `NUMBER` Result
- _Flags:_ advanced

### `DirectionName`
**Set to Direction Name**
Sets a variable to the name of the direction of a vector.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` — Direction
- **Returns:** `TEXT` One of the following: "north", "east", "south", "west", "up", "down"

### `DedupList`
**Remove Duplicate List Elements**
Removes list elements that appear more than once.
- _Info:_ Only the first appearance of a value is kept.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` _(optional)_ — List to de-duplicate
- **Returns:** `LIST` Modified list

### `CreateDict`
**Create Dictionary**
Creates a dictionary with the given keys and values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` _(optional)_ — Key list
   - `[2]` `LIST` _(optional)_ — Value list
- **Returns:** `DICT` A dictionary of specified keys and values

### `GetItemLeftover`
**Get Item Use Leftover**
Gets an item's leftover item that gets replaced on use, e.g. drinking stew gives a bowl.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get remainder of
- **Returns:** `ITEM` Remainder item

### `GetBlockByMCTag`
**Get Blocks by Minecraft Tag**
Gets a list of blocks that belongs to the specified block tag.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Block tag ID — _Example: "#mineable/pickaxe"_
- **Tags:**
   - `Return Value Type` (slot 26): `Material ID (golden_apple)` (default), `Material Name (Golden Apple)`, `Item`
- **Returns:** `LIST` Materials
- _Flags:_ advanced

### `RoundNumber`
**Round Number**
> ⚠ **LEGACY** → use `" RoundNumber "`
Rounds a number to a whole number or multiple.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Number to round
   - `[2]` `NUMBER` _(optional)_ — Round multiple — _Default = 1 (Whole number)_
- **Tags:**
   - `Round Mode` (slot 26): `Floor`, `Nearest` (default), `Ceiling`
       - `Floor`: Rounds the number down.
       - `Nearest`: Rounds the number to the nearest value.
       - `Ceiling`: Rounds the number up.
- **Returns:** `NUMBER` Result

### `SetVectorLength`
**Set Vector Length**
Sets a vector's length. This affects all components.
- _Info:_ Vectors with an X, Y, and Z of zero will not be affected.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(optional)_ — Vector to change
   - `[2]` `NUMBER` _(optional)_ — Length — _Default = 1_

### `CreateDialog`
**Create Dialog**
Creates an empty dialog.
- _Info:_ Dialogs cannot be saved. Any Dialogs in saved variables will disappear when your plot saves its variables.
- _Info:_ Dialogs have a default close button that can be overridden by using Set Variable: Add Dialog Button and setting the "Button Position: Bottom" button.
- _Info:_ Dialogs are experimental and their behavior is subject to change in future Minecraft updates.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — ID — _Can only use lowercase characters, numbers, and '_'_
   - `[2]` `COMPONENT` _(optional)_ — Title — _Default = "Dialog"_
- **Tags:**
   - `Can Close With Escape` (slot 24): `True` (default), `False`
       - `True`: Players can still exit the dialog by using the bottom button.
       - `False`: Players can still exit the dialog by using the bottom button.
   - `After Action` (slot 25): `Close` (default), `Wait for Response`, `None`
       - `Close`: Closes the dialog upon pressing a button.
       - `Wait for Response`: Keeps the player in a loading screen upon pressing a button until the dialog is closed by the plot or a new dialog or menu is opened.
       - `None`: Keeps the dialog open upon pressing a button.
   - `Type` (slot 26): `Multi Action` (default), `Multi Action with Exit Button`, `Confirmation`, `Notice`
       - `Multi Action`: Arranges an unlimited number of buttons in multiple rows where each row can have a limited number of buttons, this can be changed with the Set Variable: Set Dialog Button Columns block.
       - `Multi Action with Exit Button`: Arranges buttons the same way "Multi Action" does, however the first button is instead displayed at the very bottom as an "Exit Button".
       - `Confirmation`: The first button is considered the "Yes" the, second the "No" button, the yes button is displayed in the bottom right, the no button in the bottom right of the dialog.
       - `Notice`: Uses only the first button, which is displayed at the bottom of the dialog
- **Returns:** `DIALOG` An empty dialog

### `SetPotionDur`
**Set Potion Effect Duration**
Sets a potion effect's duration.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `POTION` _(optional)_ — Potion to change
   - `[2]` `NUMBER` — Duration (ticks)

### `BlockResistance`
**Get Block Blast Resistance**
Gets a block's blast resistance.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Block to check
   - **OR**
   - `[2]` `LOCATION` — Location to check
- **Returns:** `NUMBER` Blast resistance

### `GetRegexGroup`
**Get Regex Capture Group**
Gets the value of a specific Regex Group in a Regex match.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Input string
   - `[2]` `TEXT` — Regex pattern
   - **OR**
   - `[3]` `TEXT` — Group name
   - **OR**
   - `[4]` `NUMBER` — Group index
- **Returns:** `TEXT` Group content
- _Flags:_ advanced

### `GetParticlePower`
**Get Particle Effect Power**
Gets a particle effect's power.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get power of
- **Returns:** `NUMBER` Particle power

### `SetItemDura`
**Set Item Durability**
Sets an item's durability.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
   - `[2]` `NUMBER` — Item durability
- **Tags:**
   - `Durability Type` (slot 26): `Set Damage` (default), `Set Damage Percentage`, `Set Remaining`, `Set Remaining Percentage`

### `" SetHeadTexture "`
**Set Head Texture**
Sets a player head's texture using an owning player or custom texture.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Player head
   - `[2]` `TEXT` — Owner name, UUID or texture value
- **Tags:**
   - `Code Flow` (slot 26): `Synchronous` (default), `Asynchronous`
       - `Synchronous`: Waits for the texture to be loaded before continuing.
       - `Asynchronous`: Continues running code immediately.

### `SetBreakability`
**Set Item Breakability**
Sets whether an item is unbreakable.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
- **Tags:**
   - `Breakability` (slot 26): `Breakable`, `Unbreakable` (default)

### `" GetSignText "`
**Get Sign Text**
Gets a sign's text at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location
- **Tags:**
   - `Sign Line` (slot 26): `1` (default), `2`, `3`, `4`, `All lines`
   - `Sign Side` (slot 25): `Front` (default), `Back`
- **Returns:** `COMPONENT` Sign text; `None` ; `LIST` Contains one Styled Text for each line of the sign

### `RaycastEntity`
**Raycast to Entity**
> ⚠ **LEGACY** → use `FilterRay`
Shifts a location forward until an entity is hit and stores the hit entity UUID and location.
- _Info:_ If the starting location is in an entity's hitbox, the ray will hit them.
- _Info:_ Also hits players.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Gets the hit entity UUID (or "none")
   - `[1]` `VARIABLE` _(optional)_ — Gets the hit (or end) location
   - `[2]` `LOCATION` — Start location
   - `[3]` `NUMBER` _(optional)_ — Limit of 500 — _Default = 5_
   - `[4]` `NUMBER` _(optional)_ — Ray thickness
- _Flags:_ advanced

### `SetDictValue`
**Set Dictionary Value**
Sets the given key to the value.
- _Info:_ Setting a value to an existing key, will overwrite the old value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dictionary to add to
   - `[1]` `TEXT` — Key
   - `[2]` `ANY_TYPE` — Value

### `SetAllCoords`
**Set All Location Coordinates**
Sets a location's coordinates or creates a new location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `NUMBER` _(optional)_ — New X
   - `[3]` `NUMBER` _(optional)_ — New Y
   - `[4]` `NUMBER` _(optional)_ — New Z
   - `[5]` `NUMBER` _(optional)_ — New Pitch
   - `[6]` `NUMBER` _(optional)_ — New Yaw
- **Tags:**
   - `Coordinate Type` (slot 26): `Plot coordinate` (default), `World coordinate`

### `SaveUnloadBucket`
**Save & Unload Variable Bucket**
Saves and unloads a given bucket. Once the bucket is unloaded other plots can load this bucket if they have access to the namespace.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Save and unload result
   - `[1]` `TEXT` — Bucket key
   - `[2]` `TEXT` _(optional)_ — Namespace alias — _Default = Default alias_
- **Returns:** `TEXT` Save and unload result "success" if the bucket was successfully saved and unloaded. Any of the following if it failed: "active_lock", "bucket_not_loaded", "bucket_too_much_data", "namespace_too_much_data", "selected_more_than_one_player", "internal", "throttled"
- _Flags:_ advanced

### `GetAllItems`
**Get All Items**
Gets a list of every minecraft item material
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
- **Tags:**
   - `Include Items` (slot 26): `True` (default), `False`
       - `True`: Include items that can be held by a player but cannot be placed down.
       - `False`: Include items that can be held by a player but cannot be placed down.
   - `Include Blocks` (slot 25): `True` (default), `False`
       - `True`: Include items that can be held by a player and placed down in the world.
       - `False`: Include items that can be held by a player and placed down in the world.
- **Returns:** `LIST` List of materials

### `SetCanDestroy`
**Set Breakable Blocks**
Sets which blocks an item can break in Adventure Mode.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `BLOCK` _(plural)_ — Breakable blocks

### `" GetDirection "`
**Get Direction**
> ⚠ **LEGACY** → use `GetDirection`
Sets a variable to the direction this location is rotated in.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location
- **Tags:**
   - `Return Type` (slot 26): `Text (3D)` (default), `Text (2D)`, `Vector`
       - `Text (3D)`: "north", "south", "east", "west", "up", "down"
       - `Text (2D)`: "north", "south", "east", "west"

### `" GetItemLore "`
**Get Item Lore**
Gets an item's lore list.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get lore from
- **Returns:** `LIST` Contains one Styled Text for each line in the lore

### `RemoveListIndex`
**Remove List Value at Index**
Removes a list variable's value at an index and shifts all values after it to the left.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — List to change
   - `[1]` `NUMBER` _(plural)_ — Index to remove

### `CellularNoise`
**Get Cellular Noise**
Gets cellular noise: A type of noise based on distance from cell origins.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Noise location
   - `[2]` `NUMBER` _(optional)_ — Frequency — _Modifies the scale that the noise generates at. Default = 1_
   - `[3]` `NUMBER` _(optional)_ — Marbling — _Every 0.5 adds an additional ring to the noise output. Default = 0.5_
   - `[4]` `NUMBER` _(optional)_ — Octaves — _Adds an additional layer of cellular noise to the output. 1-16, Default = 1_
   - `[5]` `NUMBER` _(optional)_ — Lacunarity — _Every time a new octave is added, the frequency is multiplied by this value. Default = 1.5_
   - `[6]` `NUMBER` _(optional)_ — Gain — _Every time a new octave is added, the weight of it is multiplied by this value. Default = 0.5_
   - `[7]` `NUMBER` _(optional)_ — Warping — _Distorts the noise output. Default = 0_
   - `[8]` `NUMBER` _(optional)_ — Resonance — _Modifies the frequency of warping. Default = 1_
   - `[9]` `NUMBER` _(optional)_ — Domains — _Adds an additional layer of warping to the output. Default = 1_
   - `[10]` `NUMBER` _(optional)_ — Domain lacunarity — _Every time a new domain is added, the resonance is multiplied by this value. Default = 1.5_
   - `[11]` `NUMBER` _(optional)_ — Domain gain — _Every time a new domain is added, the weight of it is multiplied by this value. Default = 0.5_
   - `[12]` `NUMBER` _(optional)_ — Generation seed
- **Tags:**
   - `Return Type` (slot 26): `Voronoi` (default), `Worley`, `Secondary`, `Additive`, `Subtractive`, `Divisive`, `Multiplicative`, `Origin`
       - `Voronoi`: Provides the identifier of the closest cell.
       - `Worley`: Provides the distance from the closest cell origin.
       - `Secondary`: Provides the secondary distance from the nearest cell origin.
       - `Additive`: Provides the additive distance from the nearest cell origin.
       - `Subtractive`: Provides the subtractive distance from the nearest cell origin.
       - `Divisive`: Provides the divisive distance from the nearest cell origin.
       - `Multiplicative`: Provides the multiplicative distance from the nearest cell origin.
       - `Origin`: Provides a vector that points to the closest cell origin.
   - `Distance Calculation` (slot 25): `Euclidean` (default), `Manhattan`, `Natural`
       - `Euclidean`: Determines the closest cell using regular distance.
       - `Manhattan`: Determines the closest cell using distance measured orthogonally.
       - `Natural`: Determines the closest cell using Euclidean and Manhattan distance.
   - `Domain Fractal` (slot 24): `Progressive` (default), `Independent`
       - `Progressive`: Domains are calculated on top of one another.
       - `Independent`: Domains are calculated in isolation.
   - `Dimensions` (slot 23): `3D` (default), `2D`
       - `3D`: Generates using the X, Y and Z axis.
       - `2D`: Generates using the X and Z axis only.
- **Returns:** `NUMBER` A decimal value between 0 and 1; `None` ; `VECTOR` A vector pointing towards the cell origin.
- _Flags:_ advanced

### `Base64Encode`
**Encode to Base64**
Encodes bytes as a list of numbers to Base64.
- _Info:_ Returns 0 if the result is too large.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `BYTE` _(plural)_ — Byte(s) to encode
- **Returns:** `LIST` Bytes as a list of numbers
- _Flags:_ advanced

### `SetItemTag`
**Set Item Custom Tag**
Sets the value of or creates a custom stored tag value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` — Tag name
   - **OR**
   - `[3]` `NUMBER` — Tag value
   - **OR**
   - `[4]` `TEXT` — Tag value

### `TrimString`
**Trim String** · aliases: `TrimText`
Trims a string, starting and ending at the given positions.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` _(optional)_ — String to trim
   - `[2]` `NUMBER` — Start character position
   - `[3]` `NUMBER` _(optional)_ — End character position — _Default = String length_
- **Returns:** `TEXT` String trimmed from the original string

### `SetPotionAmp`
**Set Potion Effect Amplifier**
Sets a potion effect's amplifier.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `POTION` _(optional)_ — Potion to change
   - `[2]` `NUMBER` — Amplifier

### `GetCanDestroy`
**Get Breakable Blocks**
Gets which blocks an item can break in Adventure Mode.
- _Info:_ Output is in arbitrary order.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item
- **Returns:** `LIST` Contains one Item for each breakable block

### `RotateAroundAxis`
**Rotate Vector Around Axis**
Rotates a vector around an axis by an angle.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(optional)_ — Vector to rotate
   - `[2]` `NUMBER` — Angle
- **Tags:**
   - `Axis` (slot 25): `X` (default), `Y`, `Z`
   - `Angle Units` (slot 26): `Degrees` (default), `Radians`
- **Returns:** `VECTOR` Rotated vector

### `SetModelDataNums`
**Set Custom Model Data Numbers**
Sets the numerical model values on an item, used in resource packs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `NUMBER` _(plural)_ — Number model value(s)
- **Tags:**
   - `Model Value Type` (slot 26): `Floats` (default), `Flags`, `Colors`
       - `Floats`: Sets the "floats" array of the model data Valid input: Decimal numbers
       - `Flags`: Sets the "flags" array of the model data Valid input: 0 (false) or 1 (true)
       - `Colors`: Sets the "colors" array of the model data Valid input: RGB integers (e.g. 16733525)

### `SetBookGen`
**Set Book Generation**
Sets a book's generation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Book to set generation of
- **Tags:**
   - `Book Generation` (slot 26): `Original` (default), `Copy of Original`, `Copy of a Copy`, `Tattered`, `None`

### `RemDialogButton`
**Remove Dialog Button By Index**
Removes a dialog button element by its index.
- _Info:_ The bottom button cannot be removed.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `NUMBER` — Index

### `BlockHardness`
**Get Block Hardness**
Gets a block's hardness value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Block to check
   - **OR**
   - `[2]` `LOCATION` — Location to check
- **Returns:** `NUMBER` Block hardness

### `GetPotionAmp`
**Get Potion Effect Amplifier**
Gets a potion effect's amplifier.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `POTION` — Potion to get amplifier of
- **Returns:** `NUMBER` Potion amplifier

### `GetBlockShape`
**Get Block Collision Shape**
Gets a block collision shape at a location.
- _Info:_ Returns a list of location pairs. Each pair consists of a minimum and maximum location representing a bounding box.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Block location
- **Returns:** `LIST` Collision shape

### `GetDictSize`
**Get Dictionary Size**
Gets the number of entries in a dictionary.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `DICT` — Dictionary to measure
- **Returns:** `NUMBER` Dictionary size

### `SetItemAmount`
**Set Item Stack Size**
Sets an item's stack size.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `NUMBER` — Stack size

### `GetBookTitle`
**Get Book Title**
Gets a book's title.
- _Info:_ Returns 0 if the item doesn't have a title.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Book
- **Returns:** `TEXT` Title

### `SetCase`
**Set String Case**
Sets a string's capitalization (eg. to uppercase).
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` _(optional)_ — String to change
- **Tags:**
   - `Capitalization Type` (slot 26): `UPPERCASE` (default), `lowercase`, `Proper Case`, `iNVERT CASE`, `RAnDoM cASe`
- **Returns:** `TEXT` Modified string

### `DialogOpenButton`
**Add Open Dialog Button**
Adds a button element to a given dialog that opens another dialog.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `DIALOG` — Dialog to open
   - `[2]` `COMPONENT` — Label
   - `[3]` `COMPONENT` — Tooltip
   - `[4]` `NUMBER` _(optional)_ — Button width — _Default = 200 (1-1024)_

### `GetLight`
**Get Light Level**
Gets the light level at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Light location
- **Tags:**
   - `Light Type` (slot 26): `Combined light` (default), `Sky light`, `Block light`
- **Returns:** `NUMBER` Light level

### `GetDictValues`
**Get Dictionary Values**
Gets the list of values in this dictionary.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `DICT` — Dictionary to pull from
- **Returns:** `LIST` Contains one value for each entry of the dictionary

### `SetItemLore`
**Set Item Lore**
> ⚠ **LEGACY** → use `" SetItemLore "`
Sets an item's lore list.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - **OR**
   - `[2]` `COMPONENT` _(optional, plural)_ — Lore
   - **OR**
   - `[3]` `COMPONENT` _(optional)_ — Lore line text
   - `[4]` `NUMBER` — Line number

### `ParseMiniMessage`
**Parse MiniMessage Expression**
Parses a MiniMessage expression from a string value into a styled text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — String to parse
- **Tags:**
   - `Allowed Tags` (slot 26): `Style Only` (default), `Dynamic`, `Full`
       - `Style Only`: Resolves the following tags: • <color> • <gradient> • <rainbow> • <reset> • <transition> • <bold> • <italic> • <underline> • <obfuscated> • <space> • <newline> • <empty>
       - `Dynamic`: Resolves the following tags: • ...all tags from "Style Only" • <font> • <keybind> • <translatable>
       - `Full`: Resolves the following tags: • ...all tags from "Dynamic" • <click> • <hover> • <insertion>
   - `Parse Legacy Color Codes` (slot 25): `True`, `False` (default)
- **Returns:** `COMPONENT` Text parsed from the expressions

### `SetModelDataStrs`
**Set Custom Model Data Strings**
Sets the model string values on an item, used in resource packs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` _(plural)_ — String model value(s)

### `AddItemEnchant`
**Add Item Enchantment**
Adds an enchantment to an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` — Enchantment name
   - `[3]` `NUMBER` — Enchantment level

### `AddItemToolRule`
**Add Tool Mining Rule**
Adds a rule for breaking blocks to a tool item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `NUMBER` — Mining speed
   - **OR**
   - `[3]` `BLOCK` _(plural)_ — Blocks to apply the rule for
   - **OR**
   - `[4]` `TEXT` — Block tag ID to apply the rule for — _Example: "#mineable/pickaxe"_
- **Tags:**
   - `Correct Tool for Loot` (slot 26): `True` (default), `False`
       - `True`: If the mined block requires certain tools for item drops, this tool will cause the items to drop and increases the mining speed.
       - `False`: If the mined block requires certain tools for item drops, this tool will not cause the items to drop.

### `GetItemType`
**Get Item Material**
Gets an item's material.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get material of
- **Tags:**
   - `Return Value Type` (slot 26): `Item ID (golden_apple)` (default), `Item Name (Golden Apple)`, `Item`
- **Returns:** `TEXT` Material ID or name; `None` ; `ITEM` Material as item

### `GetDirection`
**Get Location Direction**
Gets a location's rotation (pitch and yaw) as a direction.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location to get direction of
- **Returns:** `VECTOR` Direction

### `GetLoreLine`
**Get Item Lore Line**
Gets a single line from an item's lore.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get lore from
   - `[2]` `NUMBER` — Lore line to get
- **Returns:** `COMPONENT` Lore text

### `SetItemMaxDura`
**Set Item Maximum Durability**
Sets an item's maximum durability.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
   - `[2]` `NUMBER` _(optional)_ — Maximum durability

### `RemoveString`
**Remove String** · aliases: `RemoveText`
Searches for part of a string and removes all instances of it.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` _(optional)_ — String to change
   - `[2]` `TEXT` _(plural)_ — String to remove
- **Tags:**
   - `Regular Expressions` (slot 26): `Enable`, `Disable` (default)
- **Returns:** `TEXT` Modified string

### `RemoveItemTag`
**Remove Item Custom Tag**
Removes a custom item tag.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` — Tag name

### `SetParticleSize`
**Set Particle Effect Size**
Sets a particle effect's particle size and size variation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `NUMBER` — Particle size
   - `[3]` `NUMBER` _(optional)_ — Size variation (%)

### `GetPotionDur`
**Get Potion Effect Duration**
Gets a potion effect's duration.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `POTION` — Potion to get duration of
- **Returns:** `NUMBER` Potion duration in ticks

### `SetSoundType`
**Set Sound Type**
> ⚠ **LEGACY** → use `" SetSoundType "` (Sounds are now represented using their key identifiers.)
Sets a sound's type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` _(optional)_ — Sound to change
   - `[2]` `TEXT` — Sound name (e.g. "pling")

### `ShiftDirection`
**Shift Location in Direction**
> ⚠ **LEGACY** → use `ShiftInDirection`
Shifts a location forwards or sideways. The direction is based on its rotation (pitch/yaw).
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to shift
   - `[2]` `NUMBER` _(optional)_ — Shift distance — _Default = 1_
- **Tags:**
   - `Direction` (slot 26): `Forwards 2D (X/Z)`, `Forwards 3D (X/Y/Z)` (default), `Sideways (-L / +R)`

### `DialogInputOption`
**Add Single Option Dialog Input**
Adds a single option dialog input element to a dialog.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `TEXT` — ID — _Can only use lowercase characters, numbers, and '_'_
   - `[2]` `COMPONENT` — Label
   - `[3]` `NUMBER` _(optional)_ — Element width — _Default = 200 (1-1024)_
   - `[4]` `TEXT` — Default option
   - `[5]` `TEXT` _(plural)_ — Option IDs — _Can only use lowercase characters and '_'_
   - `[6]` `COMPONENT` _(plural)_ — Option Labels
- **Tags:**
   - `Show Label` (slot 26): `True` (default), `False`

### `GetContainerName`
**Get Container Name**
> ⚠ **LEGACY** → use `ContainerName`
Gets a container's name at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Container location

### `IndexOfSubstring`
**Get Index of Substring**
Searches for a string in another string and gets the index if found.
- _Info:_ Returns 0 if the substring is not found.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — String to search in
   - `[2]` `TEXT` — String to search for
   - `[3]` `NUMBER` _(optional)_ — Starting index — _Default = 1_
- **Tags:**
   - `Ignore Case` (slot 25): `True`, `False` (default)
   - `Search Type` (slot 26): `First occurrence` (default), `Last occurrence`
- **Returns:** `NUMBER` Value index

### `GetParticleSprd`
**Get Particle Effect Spread**
Gets a particle effect's horizontal or vertical spread.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get spread of
- **Tags:**
   - `Spread` (slot 26): `Horizontal` (default), `Vertical`
- **Returns:** `NUMBER` Particle spread

### `ReflectVector`
**Reflect Vector**
Reflects a vector off a surface defined by another vector.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(optional)_ — Vector to reflect
   - `[2]` `VECTOR` — Surface vector
- **Returns:** `VECTOR` Reflected vector

### `SetBookTitle`
**Set Book Title**
Sets a book's title.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Book
   - `[2]` `TEXT` — Title

### `DecimalRGB`
**Set to Decimal RGB**
Converts a hex code to a packed RGB integer.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Hex code to convert
- **Returns:** `NUMBER` Packed RGB integer
- _Flags:_ advanced

### `ClearItemAttrs`
**Clear Item Attributes**
Clears all attributes from an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
- _Flags:_ advanced

### `GetMaxItemAmount`
**Get Item Maximum Stack Size**
> ⚠ **LEGACY** → use `GetMaxAmount`
Gets an item's maximum stack size.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get maximum stack size of
- **Returns:** `NUMBER` Maximum stack size

### `" SetDirection "`
**Set Location Direction**
Sets a location's rotation (pitch and yaw) to a direction.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `VECTOR` — Direction
- **Returns:** `LOCATION` Modified location

### `" SetItemEnchants "`
**Set Item Enchantments**
Sets an item's enchantment list.
- _Info:_ Refer to the format of the 'Get Item Enchantments' action.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `DICT` — Enchantments — _Key of enchantment ID Value of enchantment level_

### `SetBookText`
**Set Book Text**
Sets a book's text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Book
   - **OR**
   - `[2]` `COMPONENT` _(plural)_ — Pages
   - **OR**
   - `[3]` `COMPONENT` — Page text
   - `[4]` `NUMBER` — Page number

### `RandomValue`
**Set to Random Value** · aliases: `RandomObj`
Sets a variable to a random value from a set.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ANY_TYPE` _(plural)_ — Value set to choose from
- **Returns:** `ANY_TYPE` Randomly chosen value

### `SetItemType`
**Set Item Material**
Sets an item's material.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` — Material

### `GetSoundType`
**Get Sound Type**
> ⚠ **LEGACY** → use `" GetSoundType "` (Sounds are now represented using their key identifiers.)
Gets the given sound's type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` — Sound to get type of
- **Returns:** `TEXT` Sound type

### `GetListValue`
**Get List Value**
Gets a list variable's value at an index.
- _Info:_ List indices start at 1.
- _Info:_ If the index is outside the given list, 0 is returned.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` — List to get value of
   - `[2]` `NUMBER` — Index
- **Returns:** `ANY_TYPE` List value

### `VoronoiNoise`
**Get Voronoi Noise**
> ⚠ **LEGACY** → use `CellularNoise` (Use Get Cellular Noise and set Return Type to Voronoi.)
Gets a Voronoi noise value: A cellular noise in which the value of an entire cell is calculated.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Noise location
   - `[2]` `NUMBER` _(optional)_ — Cell frequency — _Default = 1_
   - `[3]` `NUMBER` _(optional)_ — Cell scatter — _5-15 for cell shapes (Default = 10)_
   - `[4]` `NUMBER` _(optional)_ — Generation seed
- **Tags:**
   - `Cell Edge Type` (slot 26): `Euclidean` (default), `Manhattan`, `Natural`
- **Returns:** `NUMBER` A value between 0 and 1
- _Flags:_ advanced

### `HSBColor`
**Set to HSB Color** · aliases: `HSVColor`
Creates a color based on hue, saturation, and brightness.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - **OR**
   - `[1]` `NUMBER` — Hue (Color circle, 0-360)
   - `[2]` `NUMBER` _(optional)_ — Saturation (0-100) — _Default = 100_
   - `[3]` `NUMBER` _(optional)_ — Brightness (0-100) — _Default = 100_
   - **OR**
   - `[4]` `LIST` — H, S, B values
- **Returns:** `TEXT` Hex code of the color
- _Flags:_ advanced

### `Interpolate`
**Interpolate Number**
Interpolates two numbers based on a time value and an easing type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Time (%)
   - `[2]` `NUMBER` _(optional)_ — First number — _Default = 0_
   - `[3]` `NUMBER` _(optional)_ — Second number — _Default = 1_
- **Tags:**
   - `Easing` (slot 25): `Linear` (default), `Quad`, `Cubic`, `Quart`, `Quint`, `Sine`, `Expo`, `Circ`
   - `Direction` (slot 26): `In` (default), `Out`, `In Out`
- **Returns:** `NUMBER` Result

### `GetSoundVariant`
**Get Sound Variant**
Gets the variant of a sound.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` — Sound to get variant of
- **Returns:** `TEXT` Variant name

### `ClearFormatting`
**Clear Formatting**
Clears all formatting from the given styled text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `COMPONENT` _(optional)_ — Text to change
- **Returns:** `TEXT` Plain string with no formatting

### `InsertListValue`
**Insert List Value** · aliases: `InsertListIndex`
Inserts a value into a list variable at an index, shifting all values at and after it to the right.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — List to change
   - `[1]` `NUMBER` — Index
   - `[2]` `ANY_TYPE` — Value to insert

### `DialogBodyItem`
**Add Item Dialog Body**
Adds an item dialog body element to a dialog.
- _Info:_ Items cannot be both stackable and have durability if they do, they won't show up in the dialog.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `ITEM` — Item to display
   - `[2]` `NUMBER` _(optional)_ — Item width — _Default = 16 (1-256)_
   - `[3]` `NUMBER` _(optional)_ — Item height — _Default = 16 (1-256)_
   - `[4]` `COMPONENT` _(optional)_ — Label
   - `[5]` `NUMBER` _(optional)_ — Label width — _Default = 200 (1-1024)_
- **Tags:**
   - `Show Tooltip` (slot 26): `True` (default), `False`

### `AddDialogButton`
**Add Dialog Button**
Adds a button element to a given dialog that can do various things when clicked.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `COMPONENT` — Label
   - `[2]` `COMPONENT` — Tooltip
   - `[3]` `TEXT` — Value
   - `[4]` `NUMBER` _(optional)_ — Button width — _Default = 200 (1-1024)_
- **Tags:**
   - `Button Type` (slot 26): `ID` (default), `Run Command`, `Open URL`, `Copy to Clipboard`
       - `ID`: Triggers the Player Click Dialog Event with the specified ID.
       - `Run Command`: Runs the given plot command on click.
       - `Open URL`: Opens the given URL on click.
       - `Copy to Clipboard`: Copies the given text to the clipboard on click.

### `PurgeBucket`
**Purge Variable Bucket**
Purges all variables in a given loaded bucket.
- _Info:_ When an empty bucket is saved it will be fully deleted.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Bucket key
   - `[1]` `TEXT` _(optional)_ — Namespace alias — _Default = Default alias_
- _Flags:_ advanced

### `GetParticleDur`
**Get Particle Effect Duration**
Gets a particle effect's duration.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get duration of
- **Returns:** `NUMBER` Particle duration

### `SetSoundVolume`
**Set Sound Volume**
Sets a sound's volume.
- _Info:_ Sound volume decreases based on distance to the sound's origin.
- _Info:_ The maximum distance to hear a sound is equal to 16.0 × volume.
- _Info:_ At volumes below 1.0, the sound's loudness at its origin decreases.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` _(optional)_ — Sound to change
   - `[2]` `NUMBER` — Volume

### `GetBucketVars`
**Get Bucket Variables**
Loads the values of specific variables from a given bucket.
- _Info:_ This will return the saved values of the given variables or the current values if the bucket is loaded on this plot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VARIABLE` — Fetch result
   - `[2]` `TEXT` — Namespace alias
   - `[3]` `TEXT` — Bucket key
   - `[4]` `TEXT` _(plural)_ — Variable names
- **Returns:** `ANY_TYPE` Variable values; `TEXT` Fetch result "success" if the value was successfully loaded. Any of the following if it failed: "unknown_namespace", "no_access", "selected_more_than_one_player", "internal", "throttled"
- _Flags:_ advanced

### `AddVectors`
**Add Vectors**
Sets a variable to the sum of the given vectors.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(plural)_ — Vectors to add
- **Returns:** `VECTOR` Result

### `SanitizeTags`
**Sanitize MiniMessage Tags**
Sanitizes all MiniMessage tags in a string. This is useful for using user input in the Parse MiniMessage Set Variable.
- _Examples:_ "<green>Hello" becomes "\<green>Hello"
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` _(optional)_ — String to sanitize
- **Returns:** `TEXT` Sanitized string

### `SetPitch`
**Set Pitch**
> ⚠ **LEGACY** → use `SetCoord`
Sets the pitch (up/down) rotation of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `NUMBER` — Pitch
- **Returns:** `LOCATION` Modified location

### `GetParticleFade`
**Get Particle Effect Fade Color**
Gets a particle effect's particle fade color.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get fade color of
- **Returns:** `TEXT` Color hexadecimal Example: "#FF0000" (red)

### `RaycastBlock`
**Raycast to Block**
> ⚠ **LEGACY** → use `Raycast`
Shifts a location forward until a block is hit and stores the hit location and face.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Gets the hit block location (or "none")
   - `[1]` `VARIABLE` _(optional)_ — Gets the hit block face (or "none")
   - `[2]` `VARIABLE` _(optional)_ — Gets the exact hit (or end) location
   - `[3]` `LOCATION` — Start location
   - `[4]` `NUMBER` _(optional)_ — Limit of 500 — _Default = 5_
- **Tags:**
   - `Ignore Passable Blocks` (slot 25): `True`, `False` (default)
   - `Fluid Collision` (slot 26): `Ignore fluids` (default), `Detect fluids`, `Source blocks only`
- _Flags:_ advanced

### `GetTooltipStyle`
**Get Item Tooltip Style**
Gets an item's custom tooltip.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get tooltip style of
- **Returns:** `TEXT` Tooltip key, e.g. "namespace:id"

### `SetHeadTexture`
**Set Head Texture** · aliases: `SetHeadOwner`
> ⚠ **LEGACY** → use `" SetHeadTexture "`
Sets a player head's texture using an owning player or custom texture.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Player head
   - `[2]` `TEXT` — Owner name, UUID or texture value

### `PerlinNoise`
**Get Perlin Noise**
> ⚠ **LEGACY** → use `GradientNoise` (Use Get Gradient Noise and set Return Type to Perlin.)
Gets a Perlin noise value: A type of fractal gradient noise.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Noise location
   - `[2]` `NUMBER` _(optional)_ — Frequency (Scale) — _Default = 1_
   - `[3]` `NUMBER` _(optional)_ — Octaves (Perlin layers) — _1-8, default = 1_
   - `[4]` `NUMBER` _(optional)_ — Octave frequency gain — _Default = 1.5_
   - `[5]` `NUMBER` _(optional)_ — Octave amplitude gain — _Default = 0.75_
   - `[6]` `NUMBER` _(optional)_ — Generation seed
- **Tags:**
   - `Fractal Type` (slot 26): `Brownian` (default), `Billow (Dark edges)`, `Rigid (Light edges)`
- **Returns:** `NUMBER` A value between 0 and 1

### `WorleyNoise`
**Get Worley Noise**
> ⚠ **LEGACY** → use `CellularNoise` (Use Get Cellular Noise and set Return Type to Worley.)
Gets a Worley noise value: A cellular noise in which the distance between two cells' nuclei is calculated.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Noise location
   - `[2]` `NUMBER` _(optional)_ — Cell frequency — _Default = 1_
   - `[3]` `NUMBER` _(optional)_ — Cell scatter — _5-15 for cell shapes (Default = 10)_
   - `[4]` `NUMBER` _(optional)_ — Generation seed
- **Tags:**
   - `` (slot 25): `Euclidean` (default), `Manhattan`, `Natural`
   - `` (slot 26): `Primary` (default), `Secondary`, `Additive`, `Subtractive`, `Multiplicative`, `Divisive`
- **Returns:** `NUMBER` A value between 0 and 1
- _Flags:_ advanced

### `GetParticleColor`
**Get Particle Effect Color**
Gets a particle effect's particle color.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get color of
- **Returns:** `TEXT` Color hexadecimal Example: "#FF0000" (red)

### `SetSoundPitch`
**Set Sound Pitch**
Sets a sound's pitch or note
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` _(optional)_ — Sound to change
   - **OR**
   - `[2]` `NUMBER` — Pitch
   - **OR**
   - `[3]` `TEXT` — Note

### `GetCanPlaceOn`
**Get Placeable Blocks**
Gets which blocks an item can be placed on in Adventure Mode.
- _Info:_ Output is in arbitrary order.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item
- **Returns:** `LIST` Contains one Item for each placeable block

### `SetCustomSound`
**Set Custom Sound Key**
> ⚠ **LEGACY** → use `" SetSoundType "` (Sounds are now represented using their key identifiers, making this action obsolete.)
Sets the key of a custom sound.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` _(optional)_ — Sound to change
   - `[2]` `TEXT` _(optional)_ — Sound key

### `SetPotSherds`
**Set Decorated Pot Sherds**
Sets the sherds on a decorated pot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Decorated pot
   - `[2]` `ITEM` _(optional)_ — Back
   - `[3]` `ITEM` _(optional)_ — Left
   - `[4]` `ITEM` _(optional)_ — Right
   - `[5]` `ITEM` _(optional)_ — Front

### `StringLength`
**Get String Length** · aliases: `TextLength`
Gets the number of characters a string has.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — String to measure
- **Returns:** `NUMBER` String length

### `StyledText`
**Set to Styled Text**
Sets a variable to a styled text, or combines multiple values into one styled text.
- _Info:_ All values will be converted to styled text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ANY_TYPE` _(optional, plural)_ — Text to set to
- **Tags:**
   - `Text Value Merging` (slot 26): `Add spaces`, `No spaces` (default)
   - `Inherit Styles` (slot 25): `True` (default), `False`
       - `True`: Text style of the last value will be inherited when merging multiple values.
       - `False`: Each value will have independent text style when merging multiple values.

### `GetMiniMessageExpr`
**Get MiniMessage Expression**
Gets the MiniMessage expression for a styled text value.
- _Info:_ This value will often NOT match the source expression! Meta tags such as "<rainbow>" will be broken down into several adjacent "<color>" tags.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `COMPONENT` — Text to read
- **Returns:** `TEXT` String containing the raw expressions

### `" SetItemLore "`
**Set Item Lore**
Sets an item's lore list.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - **OR**
   - `[2]` `COMPONENT` _(plural)_ — Lore
   - **OR**
   - `[3]` `NONE` — Clears lore)
   - **OR**
   - `[4]` `COMPONENT` _(optional)_ — Lore line text
   - `[5]` `NUMBER` — Line number

### `SetItemEffects`
**Set Item Potion Effects**
Sets the potion effects applied by an item.
- _Works with:_ Potions; Tipped Arrows; Suspicious Stew
- _Info:_ Durations of effects applied by Tipped Arrows and Instant Potions are divided by 8.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `POTION` _(plural)_ — Item effects

### `CreateList`
**Create List**
Creates a list from the given values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ANY_TYPE` _(optional, plural)_ — Value list

### `GetContainerItems`
**Get Container Contents**
Gets a container's contents at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Container location
- **Tags:**
   - `Ignore Empty Slots` (slot 26): `True`, `False` (default)
- **Returns:** `LIST` Contains one Item for each item in the container

### `GetModelDataNums`
**Get Custom Model Data Numbers**
Gets the numerical model values on an item, used in resource packs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get model data from
- **Tags:**
   - `Model Value Type` (slot 26): `Floats` (default), `Flags`, `Colors`
       - `Floats`: Gets the "floats" array of the model data Contents: Decimal numbers
       - `Flags`: Gets the "flags" array of the model data Contents: 0 (false) or 1 (true)
       - `Colors`: Gets the "colors" array of the model data Contents: RGB integers (e.g. 16733525)
- **Returns:** `LIST` List of model numbers of the selected value type

### `ShiftToward`
**Shift Location Toward Location** · aliases: `ShiftTowards`, `ShiftLocTowards`
Shifts a location toward another location by the given distance.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to shift
   - `[2]` `LOCATION` — Target location
   - `[3]` `NUMBER` _(optional)_ — Shift distance — _Default = 1_

### `SetItemHideTooltip`
**Set Tooltip Visible**
Sets whether an item's tooltip is visible or not.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
- **Tags:**
   - `Tooltip` (slot 26): `Enable` (default), `Disable`

### `GetBlockDrops`
**Get Block Drop Items**
Gets the items dropped by a block if mined by a given tool.
- _Info:_ Setting the tool to air will use no tool (player's fist).
- _Info:_ Leaving the tool unset will use the best tool to mine the block.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location to check
   - `[2]` `ITEM` _(optional)_ — Tool to use — _Default = Required Tool_
- **Returns:** `LIST` List of Items dropped

### `GetParticleOpac`
**Get Particle Effect Opacity**
Gets a particle effect's opacity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get opacity of
- **Returns:** `NUMBER` Particle opacity

### `Exponent`
**Set to Exponential**
Raises a number to the power of an exponent.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Number input
   - `[2]` `NUMBER` _(optional)_ — Exponent — _Default = 2 (input²)_
- **Returns:** `NUMBER` Result

### `ShiftAllDirs`
**Shift Location in All Directions**
> ⚠ **LEGACY** → use `ShiftAllDirections`
Shifts a location in multiple directions, based on its rotation (pitch/yaw)
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to shift
   - `[2]` `NUMBER` — Forwards change
   - `[3]` `NUMBER` _(optional)_ — Upwards change
   - `[4]` `NUMBER` _(optional)_ — Sideways change (-L / +R)
- **Tags:**
   - `Ignore Pitch` (slot 26): `True`, `False` (default)

### `GetValueIndex`
**Get List Index of Value**
Searches for a value in a list variable and gets the index if found.
- _Info:_ Returns 0 if the value is not found in the list.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` — List to search in
   - `[2]` `ANY_TYPE` — Value to search
- **Tags:**
   - `Search Order` (slot 26): `Ascending (first index)` (default), `Descending (last index)`
- **Returns:** `NUMBER` Value index

### `AddItemLore`
**Add Item Lore**
Adds lines to an item's lore.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `COMPONENT` _(plural)_ — Lore to add

### `DialogInputRange`
**Add Number Range Dialog Input**
Adds a number range dialog input element to a dialog.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `TEXT` — ID — _Can only use lowercase characters, numbers, and '_'_
   - `[2]` `COMPONENT` — Label
   - `[3]` `NUMBER` — Minimum value
   - `[4]` `NUMBER` — Maximum value
   - `[5]` `NUMBER` _(optional)_ — Initial value
   - `[6]` `NUMBER` _(optional)_ — Step
   - `[7]` `NUMBER` _(optional)_ — Element width — _Default = 200 (1-1024)_
   - `[8]` `TEXT` _(optional)_ — Display format (a translation key or format string) — _Example: "%s has value %s" Default = "options.generic_value"_

### `GetBookText`
**Get Book Text**
> ⚠ **LEGACY** → use `" GetBookText "`
Gets a book's text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Book
   - **OR**
   - `[2]` `NUMBER` _(optional)_ — Page number
   - **OR**
   - `[3]` `NONE` _(optional)_ — Gets list of all pages

### `SetParticleRoll`
**Set Particle Effect Roll**
Sets a particle effect's roll.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `NUMBER` — Particle roll

### `ClampNumber`
**Clamp Number**
Checks if a number is between a minimum and maximum value, and if not, sets it to the nearest.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Number to clamp
   - `[2]` `NUMBER` — Minimum
   - `[3]` `NUMBER` — Maximum
- **Returns:** `NUMBER` Result

### `GetSoundPitch`
**Get Sound Pitch**
Gets a sound's pitch or note.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` — Sound to get pitch or note of
- **Tags:**
   - `Return Value Type` (slot 26): `Pitch (number)` (default), `Note (text)`
- **Returns:** `NUMBER` Sound pitch

### `TranslateColors`
**Translate Legacy Colors**
Converts legacy color codes written in "&" or hex format to functional color codes, or vice versa.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` _(optional)_ — String to translate
- **Tags:**
   - `Translation Type` (slot 26): `From hex to color`, `From & to color` (default), `From color to &`, `Strip color`
       - `From hex to color`: Replaces "#FF0000Example #AAFF55Text" with "Example Text"
       - `From & to color`: Replaces "&9Example &3Text" with "Example Text"
       - `From color to &`: Replaces "Example Text" with "&9Example &3Text"
       - `Strip color`: Replaces "Example Text" with "Example Text"

### `GetAllItemTags`
**Get All Item Custom Tags**
Gets all tags registered on an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get tags of
- **Returns:** `DICT` Key represents the tag name as String, value represents tag value

### `RemoveListValue`
**Remove List Value**
Removes all matching values from a list variable.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — List to change
   - `[1]` `ANY_TYPE` _(plural)_ — Value(s) to remove
- **Tags:**
   - `Items to remove` (slot 26): `All Matches` (default), `First Match`, `Last Match`

### `ValueNoise`
**Get Value Noise**
Gets value noise: A type of noise based on a lattice of random values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Noise location
   - `[2]` `NUMBER` _(optional)_ — Frequency — _Modifies the scale that the noise generates at. Default = 1_
   - `[3]` `NUMBER` _(optional)_ — Marbling — _Every 0.5 adds an additional ring to the noise output. Default = 0.5_
   - `[4]` `NUMBER` _(optional)_ — Octaves — _Adds an additional layer of value noise to the output. 1-16, Default = 1_
   - `[5]` `NUMBER` _(optional)_ — Lacunarity — _Every time a new octave is added, the frequency is multiplied by this value. Default = 1.5_
   - `[6]` `NUMBER` _(optional)_ — Gain — _Every time a new octave is added, the weight of it is multiplied by this value. Default = 0.5_
   - `[7]` `NUMBER` _(optional)_ — Warping — _Distorts the noise output. Default = 0_
   - `[8]` `NUMBER` _(optional)_ — Resonance — _Modifies the frequency of warping. Default = 1_
   - `[9]` `NUMBER` _(optional)_ — Domains — _Adds an additional layer of warping to the output. Default = 1_
   - `[10]` `NUMBER` _(optional)_ — Domain lacunarity — _Every time a new domain is added, the resonance is multiplied by this value. Default = 1.5_
   - `[11]` `NUMBER` _(optional)_ — Domain gain — _Every time a new domain is added, the weight of it is multiplied by this value. Default = 0.5_
   - `[12]` `NUMBER` _(optional)_ — Generation seed
- **Tags:**
   - `Dimensions` (slot 24): `3D` (default), `2D`
       - `3D`: Generates using the X, Y and Z axis.
       - `2D`: Generates using the X and Z axis only.
   - `Domain Fractal` (slot 25): `Progressive` (default), `Independent`
       - `Progressive`: Domains are calculated on top of one another.
       - `Independent`: Domains are calculated in isolation.
   - `Return Type` (slot 26): `Value` (default), `Cubic`
       - `Value`: Provides the average of a lattice (grid) of random values.
       - `Cubic`: Provides the bicubic average of a lattice of random values.  The use of bicubic averaging produces a smoother result than the averaging used in Value Noise.
- **Returns:** `NUMBER` A decimal value between 0 and 1
- _Flags:_ advanced

### `WebResponse`
**Get Web Response**
Gets the response from a web request.  The output dictionary has 3 keys: "status": The HTTP status code. "statusText": The HTTP status message. And "body", or "json" if the response is JSON.
- _Info:_ Custom APIs may include the "X-Hypercube-Response" header in responses for significantly lower rate limits.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to store response
   - `[1]` `TEXT` — URL to request
   - `[2]` `TEXT` _(optional)_ — Content body
- **Tags:**
   - `Request Method` (slot 25): `Post` (default), `Get`, `Put`, `Delete`
   - `Content Type` (slot 26): `text/plain` (default), `application/json`
   - `Code Flow` (slot 24): `Synchronous` (default), `Asynchronous`
       - `Synchronous`: Waits for the response before continuing.
       - `Asynchronous`: Continues running code immediately. Variable will be set when the response is received.
- **Returns:** `DICT` Web Response
- _Flags:_ rank: Overlord, advanced

### `SetCanPlaceOn`
**Set Placeable Blocks**
Sets which blocks an item can be placed on in Adventure Mode.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `BLOCK` _(plural)_ — Placeable blocks

### `GetBookGen`
**Get Book Generation**
Gets a book's generation. Can be "original", "copy of original", "copy of copy", "tattered",
- _Info:_ Returns 0 if the item doesn't have a generation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Book
- **Returns:** `TEXT` Generation

### `GetBlockType`
**Get Block Material**
Gets a block's material at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Block location
- **Tags:**
   - `Return Value Type` (slot 26): `Block ID (oak_log)` (default), `Block name (Oak Log)`, `Item`
- **Returns:** `TEXT` Material ID or name; `None` ; `ITEM` Material as item

### `ParsePitch`
**Parse Pitch**
> ⚠ **LEGACY** → use `GetCoord` (Set Coordinate tag to Pitch. This legacy action rounds the pitch to a whole number.)
Sets a variable to the pitch of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location

### `GetDictValue`
**Get Dictionary Value**
Get a dictionary variable's value for a key.
- _Info:_ If the dictionary does not contain the key, 0 is returned.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `DICT` — Dictionary to pull from
   - `[2]` `TEXT` — Key
- **Returns:** `ANY_TYPE` Value of the corresponding key in the dictionary

### `SetDialogType`
**Set Dialog Type**
Sets the type of Dialog to display. This affects how the dialog's buttons are arranged.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
- **Tags:**
   - `Type` (slot 26): `Multi Action` (default), `Multi Action with Exit Button`, `Confirmation`, `Notice`
       - `Multi Action`: Arranges an unlimited number of buttons in multiple rows where each row can have a limited number of buttons, this can be changed with the Set Variable: Set Dialog Button Columns block.
       - `Multi Action with Exit Button`: Arranges buttons the same way "Multi Action" does, however the first button is instead displayed at the very bottom as an "Exit Button".
       - `Confirmation`: The first button is considered the "Yes" the, second the "No" button, the yes button is displayed in the bottom right, the no button in the bottom right of the dialog.
       - `Notice`: Uses only the first button, which is displayed at the bottom of the dialog

### `GetVectorLength`
**Get Vector Length**
Gets a vector's length.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` — Vector to get length of
- **Tags:**
   - `Length Type` (slot 26): `Length` (default), `Length Squared`
       - `Length`: √(x² + y² + z²)
       - `Length Squared`: x² + y² + z²
- **Returns:** `NUMBER` Vector length

### `DialogInputBool`
**Add Boolean Option Dialog Input**
Adds a boolean option dialog input element to a dialog.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `TEXT` — ID — _Can only use lowercase characters, numbers, and '_'_
   - `[2]` `COMPONENT` — Label
- **Tags:**
   - `Initial Value` (slot 26): `True` (default), `False`

### `JoinStyledText`
**Join Styled Text**
Combines a list of styled texts.
- _Info:_ The joining text will be added between each of the joined texts.
- _Info:_ Each text will have independent style when joined.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` — Styled texts to join
   - `[2]` `COMPONENT` _(optional)_ — Joining styled text
   - `[3]` `COMPONENT` _(optional)_ — Final joining styled text
- **Returns:** `COMPONENT` Joined styled text

### `ContentLength`
**Get Text Content Length**
Gets the number of characters a styled text has, ignoring all formatting tags.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `COMPONENT` — Text to measure
- **Returns:** `NUMBER` Text length

### `StringToBytes`
**Convert String to Bytes**
Converts a string to a list of numbers representing bytes.
- _Info:_ Returns 0 if the result is too large.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — String to convert
- **Tags:**
   - `Signed` (slot 26): `True` (default), `False`
   - `Charset` (slot 25): `UTF-8` (default), `ASCII`
- **Returns:** `LIST` Converted value
- _Flags:_ advanced

### `GetMaxAmount`
**Get Item Maximum Stack Size**
Gets an item's maximum stack size.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get maximum stack size of
- **Returns:** `NUMBER` Maximum stack size
