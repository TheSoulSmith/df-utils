# DiamondFire actions — Set Variable (`set_var`), part 1

_Generated from actiondump (patch 8.13). Action names are exact wire strings; names in double quotes include **leading/trailing spaces** that must be kept._

## Block `set_var` — Set Variable (179 actions in this file)

Used to set the value of a variable.

### `LoadedBuckets`
**Get Loaded Buckets**
Gets all buckets currently loaded on this plot from a given namespace
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` _(optional)_ — Namespace alias — _Default = Default alias_
- **Returns:** `TEXT` The keys of all buckets from that namespace on this plot
- _Flags:_ advanced

### `String`
**Set to String** · aliases: `Text`
Sets a variable to a string, or combines multiple values into one string.
- _Info:_ All values will be converted to string.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ANY_TYPE` _(optional, plural)_ — String to set to
- **Tags:**
   - `Text Value Merging` (slot 26): `Add spaces`, `No spaces` (default)

### `AllRegexGroups`
**Get All Regex Groups**
Gets a list of all capture groups in a Regex match.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Input string
   - `[2]` `TEXT` — Regex pattern
- **Returns:** `LIST` List of all capture groups
- _Flags:_ advanced

### `GetParticleMat`
**Get Particle Effect Material**
Gets a particle effect's particle display material.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get material of
- **Returns:** `TEXT` Material ID

### `SetParticleSprd`
**Set Particle Effect Spread**
Sets a particle effect's horizontal and vertical spread.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `NUMBER` — Horizontal spread
   - `[3]` `NUMBER` — Vertical spread

### `RemoveItemAttrs`
**Remove Item Attributes**
Removes all attributes from an item with the given attribute type and slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
- **Tags:**
   - `Slot` (slot 25): `All` (default), `Main hand`, `Off hand`, `Head`, `Body`, `Legs`, `Feet`
   - `Attribute` (slot 26): `All` (default), `Air drag modifier`, `Armor`, `Armor toughness`, `Attack damage`, `Attack knockback`, `Attack speed`, `Bounciness`, `Burning time`, `Camera distance`, `Explosion knockback resistance`, `Fall damage multiplier`, `Flying speed`, `Follow range`, `Friction modifier`, `Gravity`, `Jump strength`, `Knockback resistance`, `Luck`, `Maximum absorption health`, `Maximum health`, `Movement efficiency`, `Name tag distance`, `Walking speed`, `Oxygen bonus`, `Safe fall distance`, `Scale`, `Step height`, `Water movement efficiency`, `Tempt range`, `Block break speed`, `Block interaction range`, `Entity interaction range`, `Mining efficiency`, `Sneaking speed`, `Submerged mining speed`, `Sweeping damage ratio`, `Zombie spawn reinforcements`
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
- _Flags:_ advanced

### `AppendValue`
**Append Value to List**
Adds a value to the end of a list variable.
- _Info:_ Appending a list will result in the list itself being appended to the list (a nested list).
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — List to append to
   - `[1]` `ANY_TYPE` _(plural)_ — Value(s) to append

### `%`
**Set to Remainder (%)**
Sets a variable to the remainder after dividing two numbers with a whole quotient.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Dividend
   - `[2]` `NUMBER` — Divisor
- **Tags:**
   - `Remainder Mode` (slot 26): `Remainder` (default), `Modulo`
       - `Remainder`: Takes the remainder of the numbers. Remainder keeps the sign of the numerator: -1 rem 10 = -1
       - `Modulo`: Takes the modulus of the numbers. Modulus keeps the sign of the denominator: -1 mod 10 = 9
- **Returns:** `NUMBER` Result

### `GetItemAttribute`
**Get Item Attribute**
Get an attribute's multiplier for a specific slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item
- **Tags:**
   - `Attribute` (slot 25): `Air drag modifier` (default), `Armor`, `Armor toughness`, `Attack damage`, `Attack knockback`, `Attack speed`, `Bounciness`, `Burning time`, `Camera distance`, `Explosion knockback resistance`, `Fall damage multiplier`, `Flying speed`, `Follow range`, `Friction modifier`, `Gravity`, `Jump strength`, `Knockback resistance`, `Luck`, `Maximum absorption health`, `Maximum health`, `Movement efficiency`, `Name tag distance`, `Walking speed`, `Oxygen bonus`, `Safe fall distance`, `Scale`, `Step height`, `Water movement efficiency`, `Tempt range`, `Block break speed`, `Block interaction range`, `Entity interaction range`, `Mining efficiency`, `Sneaking speed`, `Submerged mining speed`, `Sweeping damage ratio`, `Zombie spawn reinforcements`
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
   - `Active Equipment Slot` (slot 26): `Any` (default), `Main hand`, `Off hand`, `Head`, `Body`, `Legs`, `Feet`
- **Returns:** `NUMBER` Multiplier of the specified item attribute
- _Flags:_ advanced

### `ClearDict`
**Clear Dictionary**
Removes all entries from a dictionary.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dictionary to clear

### `+`
**Add Numbers (+)**
Sets a variable to the sum of the given numbers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(plural)_ — Numbers to add
- **Returns:** `NUMBER` Result

### `-`
**Subtract Numbers (-)**
Sets a variable to the difference between the given numbers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(plural)_ — Numbers to subtract
- **Returns:** `NUMBER` Result

### `" GetItemName "`
**Get Item Name**
Gets an item's name.
- _Info:_ If the item is unnamed, a <translate> component is used.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get name of
- **Returns:** `COMPONENT` Item name

### `GetItemRarity`
**Get Item Rarity**
Gets an item's rarity.
- _Info:_ Item rarity determines the item's default name color.
- _Info:_ An item's rarity is determined by its enchantments and type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item
- **Returns:** `TEXT` One of the following: ■ "common" ■ "uncommon" ■ "rare" ■ "epic"

### `MultiplyVector`
**Multiply Vector**
Multiplies a vector's length by a number.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(optional)_ — Vector to multiply
   - `[2]` `NUMBER` — Multiplier
- **Returns:** `VECTOR` Result

### `/`
**Divide Numbers (÷)**
Sets a variable to the quotient of the given numbers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(plural)_ — Numbers to divide
- **Tags:**
   - `Division Mode` (slot 26): `Default` (default), `Floor result`
       - `Floor result`: Rounds down the result.
- **Returns:** `NUMBER` Result

### `DestructureList`
**Destructure List**
Splits a list into separate variables.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LIST` — List to destructure
   - `[1]` `VARIABLE` _(plural)_ — Variable(s) to set
- **Returns:** `ANY_TYPE` Value(s) in the list

### `DialogColumns`
**Set Dialog Button Columns**
Sets the amount of button columns.
- _Info:_ This only works with the Dialog Multi Action types
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `NUMBER` — Columns — _(1-1024)_

### `GetSignText`
**Get Sign Text**
> ⚠ **LEGACY** → use `" GetSignText "`
Gets a sign's text at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location
- **Tags:**
   - `Sign Line` (slot 26): `1` (default), `2`, `3`, `4`, `All lines`

### `GetLecternPage`
**Get Lectern Page**
Gets the displayed page number of a Lectern.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Lectern location
- **Returns:** `NUMBER` Page number

### `ShiftOnAxis`
**Shift Location on Axis** · aliases: `ShiftAxis`
Shifts the X, Y, or Z coordinate of a location on its axis.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to shift
   - `[2]` `NUMBER` — Shift distance
- **Tags:**
   - `Coordinate` (slot 26): `X` (default), `Y`, `Z`

### `VectorBetween`
**Set to Vector Between Locations**
Sets a variable to the vector between two locations.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Start location
   - `[2]` `LOCATION` — End location

### `SetBreakSound`
**Set Item Break Sound**
Sets which sound an item makes when it breaks.
- _Info:_ Pitch and volume are ignored.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - **OR**
   - `[2]` `SOUND` — Break sound
   - **OR**
   - `[3]` `NONE` — Sets to default break sound)

### `GetVectorComp`
**Get Vector Component**
Gets a vector's X, Y, or Z component.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` — Vector to get component of
- **Tags:**
   - `Component` (slot 26): `X` (default), `Y`, `Z`
- **Returns:** `NUMBER` Value of the specified component

### `=`
**Set to Value (=)**
Sets a variable to a value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ANY_TYPE` — Value

### `RmText`
**Remove Text**
> ⚠ **LEGACY** → use `RemoveString`
Removes all instances of the given text from a text variable.
- _Examples:_ RmText("homchom", "hom") = "c"
- _Info:_ Numbers will be converted to text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to remove text from
   - `[1]` `TEXT` _(plural)_ — Text to remove
- **Tags:**
   - `Regular Expressions` (slot 26): `Enable`, `Disable` (default)

### `Base64Decode`
**Decode from Base64**
Decodes Base64-encoded bytes as a list of numbers.
- _Info:_ Returns 0 if the result is too large.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `BYTE` _(plural)_ — Base64 byte(s) to decode
- **Tags:**
   - `Signed` (slot 26): `True` (default), `False`
- **Returns:** `LIST` Bytes as a list of numbers
- _Flags:_ advanced

### `AlignLoc`
**Align Location**
Aligns a location to the center or corner of the block it is in.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to align
- **Tags:**
   - `Alignment Mode` (slot 26): `Block center` (default), `Lower block corner`
   - `Coordinates` (slot 25): `All coordinates` (default), `X and Z`, `Only Y`
   - `Rotation` (slot 24): `Keep rotation` (default), `Remove rotation`
- **Returns:** `LOCATION` Aligned location

### `" SetParticleType "`
**Set Particle Effect Type**
Sets a particle effect's type.
- _Info:_ Keys taken are the same as the ones used in /particle get.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `TEXT` — Particle key (e.g. "minecraft:dust")

### `GetCrossbowProj`
**Get Crossbow Projectiles**
Gets a crossbow's loaded projectile list.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Crossbow
- **Returns:** `LIST` Contains one Item for each loaded projectile

### `" GetParticleType "`
**Get Particle Effect Type**
Gets a particle effect's type.
- _Info:_ Keys returned are the same as the ones used in /particle get.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get type of
- **Returns:** `TEXT` Particle type

### `Raycast`
**Raycast from Location**
Raycasts from a location to the first intersection.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Ray origin
   - `[2]` `NUMBER` — Ray distance
- **Tags:**
   - `Entity Collision` (slot 25): `True`, `False` (default)
       - `True`: Collides with players and entities.
   - `Block Collision` (slot 26): `All blocks` (default), `Non-fluid blocks`, `Solid blocks`, `None`
       - `All blocks`: Passes through air only.
       - `Non-fluid blocks`: Passes through fluids (water and lava).
       - `Solid blocks`: Passes through non-solid blocks such as tall grass, and through fluids.
       - `None`: Passes through all blocks.
- **Returns:** `LOCATION` End location

### `RotateAroundVec`
**Rotate Vector Around Vector**
Rotates a vector around another vector by an angle.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(optional)_ — Vector to rotate
   - `[2]` `VECTOR` — Axis vector
   - `[3]` `NUMBER` — Angle
- **Tags:**
   - `Angle Units` (slot 26): `Degrees` (default), `Radians`
- **Returns:** `VECTOR` Rotated vector

### `SetItemFood`
**Set Food Properties**
> ⚠ **LEGACY** → use `SetConsumable`
Adds food properties to an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `NUMBER` — Nutrition
   - `[3]` `NUMBER` — Saturation
   - `[4]` `NUMBER` _(optional)_ — Eating duration (seconds) — _Default = 1.6_
- **Tags:**
   - `Can Always Eat` (slot 26): `True`, `False` (default)

### `GetParticleMotion`
**Get Particle Effect Motion**
Gets a particle effect's particle motion.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get motion of
- **Returns:** `VECTOR` Particle motion

### `SetTooltipStyle`
**Set Item Tooltip Style**
Sets an item's custom tooltip.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` _(optional)_ — Tooltip key — _A namespaced key ("minecraft:" by default)_

### `SetParticleMotion`
**Set Particle Effect Motion**
Sets a particle effect's particle motion and motion variation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `VECTOR` _(optional)_ — Particle motion
   - `[3]` `NUMBER` _(optional)_ — Motion variation (%)

### `Average`
**Set to Average Number**
Sets a variable to the average of the given numbers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(plural)_ — Numbers to average
- **Returns:** `NUMBER` Result

### `WrapNumber`
**Wrap Number**
> ⚠ **LEGACY** → use `WrapNum`
Checks if a number is between two bounds and if not, wraps it around the farthest bound.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Number to wrap
   - `[2]` `NUMBER` — Lower bound
   - `[3]` `NUMBER` — Upper bound

### `ClampLoc`
**Clamp Location**
Clamps a location to a region defined by 2 corners.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to clamp
   - `[2]` `LOCATION` — Corner 1
   - `[3]` `LOCATION` — Corner 2
- **Tags:**
   - `Coordinates` (slot 26): `All coordinates` (default), `X and Z`
- **Returns:** `LOCATION` Clamped location

### `SetY`
**Set Y Coordinate**
> ⚠ **LEGACY** → use `SetCoord` (Set Coordinate tag to Y.)
Sets the Y coordinate of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `NUMBER` — Y coordinate
- **Returns:** `LOCATION` Modified location

### `SetMapTexture`
**Set Map Texture**
Sets a map item's texture to the image at the given URL.
- _Info:_ Cannot be used more than 10 times without delay. An additional usage is given every 5 seconds.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` — Image URL
- _Flags:_ rank: Emperor, advanced

### `SetX`
**Set X Coordinate**
> ⚠ **LEGACY** → use `SetCoord` (Set Coordinate tag to X.)
Sets the X coordinate of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `NUMBER` — X coordinate
- **Returns:** `LOCATION` Modified location

### `SortDict`
**Sort Dictionary**
Sorts a dictionary by its keys or values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `DICT` _(optional)_ — Dictionary to sort
- **Tags:**
   - `Sorting Type` (slot 25): `Sort by Key` (default), `Sort by Value`
   - `Sorting Order` (slot 26): `Ascending` (default), `Descending`

### `GetLecternBook`
**Get Lectern Book**
Gets the book on the Lectern at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Lectern location
- **Returns:** `ITEM` Lectern book

### `GetCustomSound`
**Get Custom Sound Key**
> ⚠ **LEGACY** → use `" GetSoundType "` (Sounds are now represented using their key identifiers, making this action obsolete.)
Gets the key of a custom sound.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` — Sound to get key of
- **Returns:** `TEXT` Sound key

### `x`
**Multiply Numbers (×)**
Sets a variable to the product of the given numbers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(plural)_ — Numbers to multiply
- **Returns:** `NUMBER` Result

### `SaveBucket`
**Save Variable Bucket**
Saves a given bucket.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Save result
   - `[1]` `TEXT` — Bucket key
   - `[2]` `TEXT` _(optional)_ — Namespace alias — _Default = Default alias_
- **Returns:** `TEXT` Save result "success" if the bucket was successfully saved. Any of the following if it failed: "active_lock", "bucket_not_loaded", "bucket_too_much_data", "namespace_too_much_data", "selected_more_than_one_player", "internal", "throttled"
- _Flags:_ advanced

### `ParseYaw`
**Parse Yaw**
> ⚠ **LEGACY** → use `GetCoord` (Set Coordinate tag to Yaw. This legacy action rounds the yaw to a whole number.)
Sets a variable to the yaw of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location

### `DotProduct`
**Set to Dot Product**
Sets a variable to the dot product of two vectors.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` — Vector 1
   - `[2]` `VECTOR` — Vector 2
- **Returns:** `NUMBER` Dot product

### `SetZ`
**Set Z Coordinate**
> ⚠ **LEGACY** → use `SetCoord` (Set Coordinate tag to Z.)
Sets the Z coordinate of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `NUMBER` — Z coordinate
- **Returns:** `LOCATION` Modified location

### `SetArmorTrim`
**Set Armor Trim**
Sets the trim of an armor item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
- **Tags:**
   - `Trim Pattern` (slot 25): `None` (default), `Bolt`, `Coast`, `Dune`, `Eye`, `Flow`, `Rib`, `Sentry`, `Snout`, `Spire`, `Tide`, `Vex`, `Ward`, `Wayfinder`, `Shaper`, `Silence`, `Raiser`, `Host`, `Wild`
       - `None`: Removes the armor trim from the item.
   - `Trim Material` (slot 26): `Amethyst` (default), `Copper`, `Diamond`, `Emerald`, `Gold`, `Iron`, `Lapis Lazuli`, `Netherite`, `Quartz`, `Resin`, `Redstone`

### `PopListValue`
**Pop List Value**
Gets a list variable's value at an index and removes it.
- _Info:_ List indices start at 1.
- _Info:_ If the index is outside the given list, 0 is returned.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` — List to get value of
   - `[2]` `NUMBER` _(optional)_ — Index — _Default = Last index_
- **Returns:** `ANY_TYPE` List value

### `SetParticleOpac`
**Set Particle Effect Opacity**
Sets a particle effect's opacity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `NUMBER` — Particle opacity

### `GetPotionType`
**Get Potion Effect Type**
> ⚠ **LEGACY** → use `" GetPotionType "` (Potions are now represented using their key identifiers.)
Gets a potion effect's type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `POTION` — Potion to get type of
- **Returns:** `TEXT` Effect type

### `RepeatString`
**Repeat String** · aliases: `DuplicateText`, `RepeatText`
Repeats a string the given number of times.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — String to repeat
   - `[2]` `NUMBER` — Times to repeat
- **Returns:** `TEXT` Repeated string

### `GetItemLore`
**Get Item Lore**
> ⚠ **LEGACY** → use `" GetItemLore "`
Gets an item's lore list.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get lore from
- **Returns:** `LIST` Contains one legacy styled text line for each lore line

### `JoinString`
**Join String** · aliases: `JoinText`
Combines a list of strings.
- _Info:_ The joining string will be added between each of the joined strings.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` — Strings to join
   - `[2]` `TEXT` _(optional)_ — Joining string
   - `[3]` `TEXT` _(optional)_ — Final joining string
- **Returns:** `TEXT` Joined string

### `ReverseList`
**Reverse List**
Reverses the order of a list variable's values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` _(optional)_ — List to reverse
- **Returns:** `LIST` Reversed list

### `FaceLocation`
**Face Location**
Sets a location's rotation to face another location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `LOCATION` — Target location
- **Tags:**
   - `Face Direction` (slot 26): `Toward location` (default), `Away from location`
- **Returns:** `LOCATION` Facing location

### `" GetSoundType "`
**Get Sound Type**
Gets the given sound's type.
- _Info:_ Keys returned are the same as the ones used in /sound.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` — Sound to get type of
- **Tags:**
   - `Return Type` (slot 26): `Sound key` (default), `Sound name`
- **Returns:** `TEXT` Sound type

### `GetItemLoreLine`
**Get Item Lore Line**
> ⚠ **LEGACY** → use `GetLoreLine`
Gets a single line from an item's lore.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get lore from
   - `[2]` `NUMBER` — Lore line to get
- **Returns:** `TEXT` Lore text

### `" SetSoundType "`
**Set Sound Type**
Sets a sound's type.
- _Info:_ Keys taken are the same as the ones used in /sound.
- _Info:_ Custom sounds can be set by providing a custom key
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` _(optional)_ — Sound to change
   - `[2]` `TEXT` — Sound key (e.g. "block.note_block.pling")

### `SplitString`
**Split String** · aliases: `SplitText`
Splits a string into a list of strings.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — String to split
   - `[2]` `TEXT` _(optional)_ — Splitter string — _Default = " "_
- **Tags:**
   - `Strip Spaces` (slot 26): `Enable` (default), `Disable`
       - `Enable`: Leading and trailing spaces are removed from the result.
       - `Disable`: Leading and trailing spaces are kept.
- **Returns:** `LIST` Contains one String for each split part

### `NormalRandom`
**Set to Normally Distributed Random Number**
Sets a variable to a normally distributed random number. Values closer to μ are more likely to be chosen.
- _Info:_ Also known as a Gaussian distribution.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — μ (Mean midpoint)
   - `[2]` `NUMBER` — σ (Standard deviation)
- **Tags:**
   - `Distribution` (slot 26): `Normal` (default), `Folded normal`
       - `Normal`: Values can be above or below μ.
       - `Folded normal`: Values can only be above μ.
- **Returns:** `NUMBER` Random number
- _Flags:_ advanced

### `SetPotionType`
**Set Potion Effect Type**
> ⚠ **LEGACY** → use `" SetPotionType "` (Potions are now represented using their key identifiers.)
Sets a potion effect's type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `POTION` _(optional)_ — Potion to change
   - `[2]` `TEXT` — Type

### `AlignVector`
**Align Vector**
Aligns a vector to the nearest axis.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(optional)_ — Vector to align
- **Returns:** `VECTOR` Aligned vector

### `SetMaxAmount`
**Set Item Maximum Stack Size**
Set an item's maximum stack size.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `NUMBER` _(optional)_ — Maximum stack size (1-99)

### `SegmentList`
**Segment List**
Segments a list into sub-lists of a specified size.
- _Examples:_ [1, 2, 3, 4, 5] with segment size 2 becomes [[1, 2], [3, 4], [5]]
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` _(optional)_ — List to segment
   - `[2]` `NUMBER` — Segment size
- **Returns:** `LIST` Segmented list

### `RGBColor`
**Set to RGB Color**
Creates a color hex based on red, green, and blue channels.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - **OR**
   - `[1]` `NUMBER` — Red (0-255)
   - `[2]` `NUMBER` — Green (0-255)
   - `[3]` `NUMBER` — Blue (0-255)
   - **OR**
   - `[4]` `LIST` — R, G, B values
- **Returns:** `TEXT` Hex code of the color
- _Flags:_ advanced

### `HSLColor`
**Set to HSL Color**
Creates a color based on hue, saturation, and lightness.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - **OR**
   - `[1]` `NUMBER` — Hue (Color circle, 0-360)
   - `[2]` `NUMBER` _(optional)_ — Saturation (0-100) — _Default = 100_
   - `[3]` `NUMBER` _(optional)_ — Lightness (0-100) — _Default = 50_
   - **OR**
   - `[4]` `LIST` — H, S, L values
- **Returns:** `TEXT` Hex code of the color
- _Flags:_ advanced

### `Logarithm`
**Set to Logarithm**
Finds the logarithm of a number. A logarithm is the power the base must be raised to to get the given input.
- _Info:_ If the result of the logarithm is undefined, 0 will be returned.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Number input
   - `[2]` `NUMBER` — Base
- **Returns:** `NUMBER` Result

### `ParseMiniMessageExpr`
**Parse MiniMessage Expression**
> ⚠ **LEGACY** → use `ParseMiniMessage`
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

### `GetItemAmount`
**Get Item Stack Size**
Gets an item's stack size.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get stack size of
- **Returns:** `NUMBER` Stack size

### `GetItemName`
**Get Item Name**
> ⚠ **LEGACY** → use `" GetItemName "`
Gets an item's name.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get name of
- **Returns:** `TEXT` Item name

### `GetItemDura`
**Get Item Durability**
Gets an item's current or maximum durability.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item
- **Tags:**
   - `Durability Type` (slot 26): `Get Damage` (default), `Get Damage Percentage`, `Get Remaining`, `Get Remaining Percentage`, `Get Maximum`
- **Returns:** `NUMBER` Item durability

### `ShiftInDirection`
**Shift Location in Direction**
Shifts a location forward, upward, or sideways based on its rotation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to shift
   - `[2]` `NUMBER` _(optional)_ — Shift distance — _Default = 1_
- **Tags:**
   - `Direction` (slot 26): `Forward` (default), `Upward`, `Sideways`

### `RotationVector`
**Set to Vector from Rotation**
Sets a variable to a vector facing a given rotation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Pitch
   - `[2]` `NUMBER` — Yaw
   - `[3]` `NUMBER` _(optional)_ — Length — _Default = 1_
- **Returns:** `VECTOR` Vector facing the specified rotation

### `JsonToValue`
**Convert JSON to Value** · aliases: `JsonFromValue`
Converts JSON to a value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — JSON to convert
- **Returns:** `NUMBER` Converted value; `None` ; `TEXT` Converted value; `None` ; `LIST` Converted value; `None` ; `DICT` Converted value
- _Flags:_ advanced

### `WrapNum`
**Wrap Number**
Checks if a number is inside the bounds and if not, wraps it around the farthest bound.
- _Examples:_ WrapNum(10, 2, 8) = 4 WrapNum(4, 1, 4) = 1 WrapNum(-2, 0, 7) = 5
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Number to wrap
   - `[2]` `NUMBER` — Lower bound (inclusive)
   - `[3]` `NUMBER` — Upper bound (exclusive)
- **Returns:** `NUMBER` Result

### `ReplaceString`
**Replace String** · aliases: `ReplaceText`
Searches for part of a string and replaces it.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — String to change
   - `[2]` `TEXT` — String part to replace
   - `[3]` `TEXT` — Replacement
- **Tags:**
   - `Regular Expressions` (slot 26): `Enable`, `Disable` (default)
   - `Replacement Type` (slot 25): `First occurrence`, `All occurrences` (default)
- **Returns:** `TEXT` Modified string

### `SetItemGlowing`
**Set Item Glowing**
Sets whether an item is glowing regardless of its enchantments.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
- **Tags:**
   - `Glowing` (slot 26): `Enable` (default), `Disable`, `Default`

### `" SetItemName "`
**Set Item Name**
Sets an item's name.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `COMPONENT` _(plural)_ — Name

### `SetLodestoneLoc`
**Set Compass Lodestone Location**
Sets a compass's lodestone location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `LOCATION` — Lodestone location
- **Tags:**
   - `Require Lodestone at Location` (slot 26): `True`, `False` (default)

### `FlattenList`
**Flatten List**
Sets a variable to a list with its sub-lists spread out into individual elements.
- _Examples:_ [[1, 2], 5, [3, 4]] becomes [1, 2, 5, 3, 4]
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` _(optional)_ — List to flatten
- **Returns:** `LIST` Flattened list

### `SetItemRarity`
**Set Item Rarity**
Sets an item's rarity, affecting its default name color.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
- **Tags:**
   - `Rarity` (slot 26): `Default for item type` (default), `Common`, `Uncommon`, `Rare`, `Epic`

### `GetBucketVar`
**Get Bucket Variable**
Loads the value of a specific variable from a given bucket.
- _Info:_ This will return the saved value of the given variable or the current value if the bucket is loaded on this plot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VARIABLE` — Fetch result
   - `[2]` `TEXT` — Bucket key
   - `[3]` `TEXT` — Variable name
   - `[4]` `TEXT` _(optional)_ — Namespace alias — _Default = Default alias_
- **Returns:** `ANY_TYPE` Variable value; `TEXT` Fetch result "success" if the values were successfully loaded. Any of the following if it failed: "unknown_namespace", "no_access", "selected_more_than_one_player", "internal", "throttled"
- _Flags:_ advanced

### `SetBookSigned`
**Set Book Signed**
Sets whether or not a book is signed.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Book
- **Tags:**
   - `Sign` (slot 26): `Enable` (default), `Disable`

### `GetBundleItems`
**Get Bundle Contents**
Gets the contents of a bundle.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Bundle
- **Returns:** `LIST` Contains one Item for each item in the bundle

### `GetParticleAmount`
**Get Particle Effect Amount**
Gets a particle effect's particle amount.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get amount of
- **Returns:** `NUMBER` Particle amount

### `RandomVector`
**Set to Random Vector**
Creates a random vector that lies on a sphere. Generated vectors will be uniformly distributed on its surface.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Vector length — _Default = 1_
- **Returns:** `VECTOR` A random vector

### `SubtractVectors`
**Subtract Vectors**
Sets a variable to the difference between the given vectors.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(plural)_ — Vectors to subtract
- **Returns:** `VECTOR` Result

### `AllRegexMatches`
**Get All Regex Matches**
Gets a list of every Regex match in the given string.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Input string
   - `[2]` `TEXT` — Regex pattern
- **Tags:**
   - `Return Type` (slot 26): `Full Match` (default), `All Groups`, `All Named Groups`
       - `Full Match`: Return a list of all matches
       - `All Groups`: Return a list of lists where each sub-list contains all groups of the relevant Regex match
       - `All Named Groups`: Return a list of dictionaries where each dictionary contains all named groups of the relevant Regex match
- **Returns:** `LIST` List of all matches
- _Flags:_ advanced

### `SwapVectorComp`
**Swap Vector Components**
Swaps two of a vector's X, Y, or Z components.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(optional)_ — Vector to swap components of
- **Tags:**
   - `Components to Swap` (slot 26): `X ⇄ Y` (default), `X ⇄ Z`, `Y ⇄ Z`
       - `X ⇄ Y`: Swaps the X and Y components.
       - `X ⇄ Z`: Swaps the X and Z components.
       - `Y ⇄ Z`: Swaps the Y and Z components.
- **Returns:** `VECTOR` The vector with swapped components

### `SetParticleColor`
**Set Particle Effect Color**
Sets a particle effect's particle color and color variation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `TEXT` — Color hexadecimal — _Example: "#FF0000" (red)_
   - `[3]` `NUMBER` _(optional)_ — Color variation (%)

### `" GetBookText "`
**Get Book Text**
Gets a book's text.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Book
   - **OR**
   - `[2]` `NUMBER` — Page number
   - **OR**
   - `[3]` `NONE` — Gets list of all pages)
- **Returns:** `COMPONENT` Text of the specified page; `None` ; `LIST` Contains one Styled Text for each page of the book

### `Vector`
**Set to Vector**
Sets a variable to a vector.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - **OR**
   - `[1]` `NUMBER` — X component
   - `[2]` `NUMBER` — Y component
   - `[3]` `NUMBER` — Z component
   - **OR**
   - `[4]` `TEXT` — Direction name — _One of the following: "north", "east", "south", "west", "up", "down"_
- **Returns:** `VECTOR` Vector with the specified components or direction

### `Distance`
**Set to Distance**
Sets a variable to the distance between two locations.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location 1
   - `[2]` `LOCATION` — Location 2
- **Tags:**
   - `Distance Type` (slot 26): `Distance 2D (X/Z)`, `Distance 3D (X/Y/Z)` (default), `Altitude (Y)`
- **Returns:** `NUMBER` Distance

### `DialogCanEscape`
**Set Dialog Can Close With Escape**
Sets whether a dialog can be closed by pressing the escape key.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to modify
- **Tags:**
   - `Can Close With Escape` (slot 26): `True` (default), `False`

### `Root`
**Set to Root** · aliases: `SquareRoot`
Takes the root of a number.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Number input
   - `[2]` `NUMBER` _(optional)_ — Root index — _Default = 2 (²√input)_
- **Returns:** `NUMBER` Result

### `SetParticleAmount`
**Set Particle Effect Amount**
Sets a particle effect's particle amount.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `NUMBER` — Particle amount

### `DialogInputText`
**Add Text Box Dialog Input**
Adds a text box dialog input element to a dialog.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `TEXT` — ID — _Can only use lowercase characters, numbers, and '_'_
   - `[2]` `COMPONENT` — Label
   - `[3]` `TEXT` _(optional)_ — Initial content — _Default = ""_
   - `[4]` `NUMBER` _(optional)_ — Element width — _Default = 200 (1-1024)_
   - `[5]` `NUMBER` _(optional)_ — Maximum characters — _Default = 32_
   - `[6]` `NUMBER` _(optional)_ — Maximum lines
   - `[7]` `NUMBER` _(optional)_ — Element Height
- **Tags:**
   - `Show Label` (slot 26): `True` (default), `False`

### `ArcTangent2`
**Set to Arc Tangent of 2 Numbers**
Sets a variable to the arc tangent of 2 numbers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Y
   - `[2]` `NUMBER` — X
- **Tags:**
   - `Output Type` (slot 26): `Degrees` (default), `Radians`
- **Returns:** `NUMBER` Result
- _Flags:_ advanced

### `GetParticleType`
**Get Particle Effect Type**
> ⚠ **LEGACY** → use `" GetParticleType "` (Particles are now represented using their key identifiers.)
Gets a particle effect's type.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` — Effect to get type of
- **Returns:** `TEXT` Particle type

### `GetAllBlockData`
**Get All Block Data**
Gets the block state tags at a location.
- _Info:_ Returns "0" if the block has no tags.
- _Info:_ Always returns a string value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Block location
- **Tags:**
   - `Hide Default` (slot 26): `True` (default), `False`
- **Returns:** `TEXT` Block data separated with commas (,)

### `SetCrossbowProj`
**Set Crossbow Projectiles**
Sets a crossbow's loaded projectiles.
- _Works with:_ Arrows; Fireworks
- _Info:_ A crossbow can hold any number of projectiles.
- _Info:_ The projectiles will shoot out in a straight line.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Crossbow
   - `[2]` `ITEM` _(optional, plural)_ — Projectiles

### `MaxNumber`
**Set to Maximum Number** · aliases: `MaxValue`, `Max`
Sets a variable to the highest number in a set.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(plural)_ — Number set to compare
- **Returns:** `NUMBER` Result

### `GetDictKeys`
**Get Dictionary Keys**
Gets the list of keys in this dictionary.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `DICT` — Dictionary to pull from
- **Returns:** `LIST` Contains one String entry for each key of the dictionary

### `TrimStyledText`
**Trim Styled Text Content**
Trims the content of styled text, leaving all formatting in place.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `COMPONENT` _(optional)_ — Text to trim
   - `[2]` `NUMBER` — Start character position
   - `[3]` `NUMBER` _(optional)_ — End character position — _Default = Text length_
- **Returns:** `COMPONENT` The text trimmed from the original text

### `SetParticleMat`
**Set Particle Effect Material**
Sets a particle effect's particle display material.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `ITEM` — Particle material

### `GetCoord`
**Get Location Coordinate**
Gets a location's X, Y, Z, pitch, or yaw coordinate.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location to get coordinate of
- **Tags:**
   - `Coordinate Type` (slot 25): `Plot coordinate` (default), `World coordinate`
   - `Coordinate` (slot 26): `X` (default), `Y`, `Z`, `Pitch`, `Yaw`
- **Returns:** `NUMBER` Coordinate of the specified axis

### `GetModelDataStrs`
**Get Custom Model Data Strings**
Gets the model string values on an item, used in resource packs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get model data from
- **Returns:** `LIST` List of model strings

### `RandomLoc`
**Set to Random Location**
Sets the variable to a random location between two locations.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Location 1
   - `[2]` `LOCATION` — Location 2
- **Returns:** `LOCATION` Random location

### `GetLodestoneLoc`
**Get Compass Lodestone Location**
Gets a compass's lodestone location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Compass to get lodestone location of
- **Returns:** `LOCATION` Block location

### `" GetItemAttribute "`
**Get Item Attribute**
Get an attribute's multiplier for a specific slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item
- **Tags:**
   - `Attribute` (slot 25): `Air drag modifier` (default), `Armor`, `Armor toughness`, `Attack damage`, `Attack knockback`, `Attack speed`, `Bounciness`, `Burning time`, `Camera distance`, `Explosion knockback resistance`, `Fall damage multiplier`, `Flying speed`, `Follow range`, `Friction modifier`, `Gravity`, `Jump strength`, `Knockback resistance`, `Luck`, `Maximum absorption health`, `Maximum health`, `Movement efficiency`, `Name tag distance`, `Walking speed`, `Oxygen bonus`, `Safe fall distance`, `Scale`, `Step height`, `Water movement efficiency`, `Tempt range`, `Block break speed`, `Block interaction range`, `Entity interaction range`, `Mining efficiency`, `Sneaking speed`, `Submerged mining speed`, `Sweeping damage ratio`, `Zombie spawn reinforcements`
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
   - `Active Equipment Slot` (slot 26): `Any` (default), `Main hand`, `Off hand`, `Either hand`, `Head`, `Chest`, `Legs`, `Feet`, `Body`, `Saddle`
- **Returns:** `NUMBER` Multiplier of the specified item attribute
- _Flags:_ advanced

### `SetItemLeftover`
**Set Item Use Leftover**
Sets an item's leftover item that gets replaced on use, e.g. drinking stew gives a bowl.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `ITEM` _(optional)_ — Remainder item

### `GetHeadOwner`
**Get Head Owner**
Gets a player head's owner name or UUID.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Head to get owner of
- **Tags:**
   - `Text Value` (slot 26): `Owner Name` (default), `Owner UUID`
- **Returns:** `TEXT` Head owner

### `GetItemEnchants`
**Get Item Enchantments**
> ⚠ **LEGACY** → use `" GetItemEnchants "`
Gets an item's enchantment list.
- _Info:_ Returns a list of lists, each with two values: the enchantment name and level.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get enchantments of

### `AppendDict`
**Append Dictionary**
Adds all entries from one dictionary into the other.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dictionary to add to
   - `[1]` `DICT` — Dictionary to append

### `SetItemModel`
**Set Item Model**
Sets an item's model.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` _(optional)_ — Model key — _A namespaced key ("minecraft:" by default) Example: "stone"_

### `GetColorChannels`
**Get Color Channels**
Gets a color's RGB/HSB/HSL number values as a list.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Color hexadecimal — _Example: "#FF0000" (red)_
- **Tags:**
   - `Color Channels` (slot 26): `RGB` (default), `HSB`, `HSL`
       - `RGB`: 1. Red (0 - 255) 2. Green (0 - 255) 3. Blue (0 - 255)
       - `HSB`: 1. Hue (0.0 - 360.0) 2. Saturation (0.0 - 100.0) 3. Brightness (0.0 - 100.0)
       - `HSL`: 1. Hue (0.0 - 360.0) 2. Saturation (0.0 - 100.0) 3. Lightness (0.0 - 100.0)
- **Returns:** `LIST` Contains one Number for each color channel
- _Flags:_ advanced

### `SetListValue`
**Set List Value**
Sets a list variable's value at an index.
- _Info:_ List indices start at 1.
- _Info:_ Setting an index beyond the bounds of the list will clamp the index to the list bounds.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — List to change
   - `[1]` `NUMBER` — Index
   - `[2]` `ANY_TYPE` — Value to set

### `" SetItemFlags "`
**Set Item Visibility Flags**
> ⚠ **LEGACY** → use `HiddenComponents` (Item Flags were removed from Minecraft in 1.20.5. Hide individual components, or use 'Hide all except' and provide the 'lore' component.)
Sets which components of an item are visible, similar to /hideflags.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
- **Tags:**
   - `Armor Trim` (slot 18): `Hide`, `Show`, `No Change` (default)
   - `Color` (slot 19): `Hide`, `Show`, `No Change` (default)
   - `Enchantments` (slot 20): `Hide`, `Show`, `No Change` (default)
   - `Attributes` (slot 21): `Hide`, `Show`, `No Change` (default)
   - `Unbreakable` (slot 22): `Hide`, `Show`, `No Change` (default)
   - `Can Destroy` (slot 23): `Hide`, `Show`, `No Change` (default)
   - `Can Place On` (slot 24): `Hide`, `Show`, `No Change` (default)
   - `Potion Effects` (slot 25): `Hide`, `Show`, `No Change` (default)
   - `Others` (slot 26): `Hide`, `Show`, `No Change` (default)

### `ClampVector`
**Clamp Vector**
Clamps a vector to a region defined by 2 vectors
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(optional)_ — Vector to clamp
   - `[2]` `VECTOR` — Lower bound vector
   - `[3]` `VECTOR` — Upper bound vector
- **Returns:** `VECTOR` Clamped vector

### `ValueToJson`
**Convert Value to JSON**
Converts a value to JSON.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - **OR**
   - `[1]` `NUMBER` — Value to convert
   - **OR**
   - `[2]` `TEXT` — Value to convert
   - **OR**
   - `[3]` `LIST` — Value to convert
   - **OR**
   - `[4]` `DICT` — Value to convert
- **Returns:** `TEXT` JSON string
- _Flags:_ advanced

### `BounceNum`
**Bounce Number**
Checks if a number is inside the bounds and if not, bounces it towards the other bound.
- _Examples:_ BounceNum(10, 2, 8) = 6 BounceNum(4, 1, 4) = 4 BounceNum(-9, 0, 7) = 5
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` _(optional)_ — Number to bounce
   - `[2]` `NUMBER` — Lower bound (inclusive)
   - `[3]` `NUMBER` — Upper bound (exclusive)
- **Returns:** `NUMBER` Result

### `Tangent`
**Set to Tangent**
Sets a variable to the trigonometric tangent function of a number.
- _Info:_ Use the variant tag to swap between tan, atan and tanh functions.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Number input
- **Tags:**
   - `Tangent Variant` (slot 25): `Tangent` (default), `Inverse tangent (arctangent)`, `Hyperbolic tangent`
   - `Input` (slot 26): `Degrees` (default), `Radians`
- **Returns:** `NUMBER` Result
- _Flags:_ advanced

### `SetDirection`
**Set Location Direction** · aliases: `FaceDirection`
> ⚠ **LEGACY** → use `" SetDirection "`
Rotates a location to face a direction.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - **OR**
   - `[2]` `TEXT` — Direction — _"north", "south", "east", "west", "up", "down"_
   - **OR**
   - `[3]` `VECTOR` — Direction
   - **OR**
- **Tags:**
   - `Face Direction` (slot 26): `Towards direction` (default), `Towards opposite direction`

### `+=`
**Increment Number (+=)**
Increments a number variable by 1 or more other numbers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to increment
   - `[1]` `NUMBER` _(optional, plural)_ — Number(s) to increment by — _Default = 1_
- **Returns:** `NUMBER` Result

### `" GetPotionType "`
**Get Potion Effect Type**
Gets a potion effect's type.
- _Info:_ Keys returned are the same as the ones used in /potion get.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `POTION` — Potion to get type of
- **Returns:** `TEXT` Effect type

### `GetItemColor`
**Get Item Color**
Gets a colorable item's color.
- _Works with:_ Leather Armor; Potions; Tipped Arrows; Filled Maps; Firework Stars
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get color of
- **Returns:** `TEXT` Color hexadecimal Example: "#FF0000" (red)
- _Flags:_ advanced

### `SetCoord`
**Set Location Coordinate**
Sets a location's X, Y, Z, pitch, or yaw coordinate.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `NUMBER` — Coordinate
- **Tags:**
   - `Coordinate Type` (slot 25): `Plot coordinate` (default), `World coordinate`
   - `Coordinate` (slot 26): `X` (default), `Y`, `Z`, `Pitch`, `Yaw`

### `SetItemTool`
**Set Tool Properties**
Adds tool properties to an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `NUMBER` _(optional)_ — Default mining speed — _Default = 1_
   - `[3]` `NUMBER` _(optional)_ — Damage per block — _Default = 1_

### `" GetItemEnchants "`
**Get Item Enchantments**
Gets an item's enchantments.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get enchantments from
- **Returns:** `DICT` Key represents the enchantment name as String, value represents the enchantment level as Number

### `GzipCompress`
**Compress to Gzip**
Compresses bytes as a list of numbers to Gzip.
- _Info:_ Returns 0 if the result is too large.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `BYTE` _(plural)_ — Byte(s) to compress
- **Returns:** `LIST` Bytes as a list of numbers
- _Flags:_ advanced

### `SetBookAuthor`
**Set Book Author**
Sets a book's author.
- _Works with:_ Signed Books
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Book
   - `[2]` `TEXT` — Author

### `SetItemColor`
**Set Item Color** · aliases: `SetLeatherColor`
Sets a colorable item's color.
- _Works with:_ Leather Armor; Potions; Tipped Arrows; Filled Maps; Firework Stars
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - **OR**
   - `[2]` `TEXT` — Color hexadecimal — _Example: "#FF0000" (red)_
   - **OR**
   - `[3]` `NONE` — Removes color)
- _Flags:_ advanced

### `" RoundNumber "`
**Round Number**
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

### `SortList`
**Sort List**
Sorts a list variable's values.
- _Works with:_ Numbers, Text, Lists
- _Info:_ Sub-lists are sorted by their first index.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` _(optional)_ — List to sort
- **Tags:**
   - `Sort Order` (slot 26): `Ascending` (default), `Descending`
- **Returns:** `LIST` Sorted list

### `RemoveDictEntry`
**Remove Dictionary Entry**
Removes the dictionary entry with the given key.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dictionary to change
   - `[1]` `TEXT` — Key to remove
   - `[2]` `ANY_TYPE` _(optional, plural)_ — Expected value(s) — _When given, the entry will only be removed if the current value matches._

### `FormatTime`
**Format Timestamp**
Converts a numerical timestamp to a human-readable time/date text using a date format.
- _Info:_ Use Game Value: Timestamp to generate timestamps.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Time to format
   - `[2]` `TEXT` _(optional)_ — Custom Format — _Used only if Tag: Format is set to Custom_
- **Tags:**
   - `Format` (slot 26): `Custom`, `2020/08/17 17:20:54` (default), `2020/08/17`, `Mon, August 17`, `Monday`, `17:20:54`, `5:20 PM`, `17h20m54s`, `54.229 seconds`
       - `Custom`: Use the SimpleDateFormat online documentation.
       - `2020/08/17 17:20:54`: YYYY/MM/dd HH:mm:s
       - `2020/08/17`: YYYY/MM/dd
       - `Mon, August 17`: EEE, MMMM d
       - `Monday`: EEEE
       - `17:20:54`: HH:mm:ss
       - `5:20 PM`: h:mm a
       - `17h20m54s`: H'h'm'm's's'
       - `54.229 seconds`: s.S 'seconds'
- **Returns:** `TEXT` Formatted timestamp

### `SetItemFlags`
**Set Item Visibility Flags**
> ⚠ **LEGACY** → use `HiddenComponents` (Item Flags were removed from Minecraft in 1.20.5. Hide individual components, or use 'Hide all except' and provide the 'lore' component.)
Sets which components of an item are visible, similar to /hideflags.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
- **Tags:**
   - `Hide Armor Trim` (slot 19): `True`, `False`, `No Change` (default)
   - `Hide Color` (slot 20): `True`, `False`, `No Change` (default)
   - `Hide Enchantments` (slot 21): `True`, `False`, `No Change` (default)
   - `Hide Attributes` (slot 22): `True`, `False`, `No Change` (default)
   - `Hide Unbreakable` (slot 23): `True`, `False`, `No Change` (default)
   - `Hide Can Destroy` (slot 24): `True`, `False`, `No Change` (default)
   - `Hide Can Place On` (slot 25): `True`, `False`, `No Change` (default)
   - `Hide Potion Effects` (slot 26): `True`, `False`, `No Change` (default)

### `SetParticleDur`
**Set Particle Effect Duration**
Sets a particle effect's duration.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `NUMBER` — Particle duration in ticks

### `GetItemEffects`
**Get Item Potion Effects**
Gets the potion effects applied by an item.
- _Works with:_ Potions; Tipped Arrows; Suspicious Stew
- _Info:_ Durations of effects applied by Tipped Arrows and Instant Potions are multiplied by 8.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get effects from
- **Returns:** `LIST` Contains one Potion Effect for each effect in the item

### `" RemoveItemAttrs "`
**Remove Item Attributes**
Removes all attributes from an item with the given attribute type and slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item
- **Tags:**
   - `Slot` (slot 25): `All` (default), `Any`, `Main hand`, `Off hand`, `Either hand`, `Head`, `Chest`, `Legs`, `Feet`, `Body`, `Saddle`
   - `Attribute` (slot 26): `All` (default), `Air drag modifier`, `Armor`, `Armor toughness`, `Attack damage`, `Attack knockback`, `Attack speed`, `Bounciness`, `Burning time`, `Camera distance`, `Explosion knockback resistance`, `Fall damage multiplier`, `Flying speed`, `Follow range`, `Friction modifier`, `Gravity`, `Jump strength`, `Knockback resistance`, `Luck`, `Maximum absorption health`, `Maximum health`, `Movement efficiency`, `Name tag distance`, `Walking speed`, `Oxygen bonus`, `Safe fall distance`, `Scale`, `Step height`, `Water movement efficiency`, `Tempt range`, `Block break speed`, `Block interaction range`, `Entity interaction range`, `Mining efficiency`, `Sneaking speed`, `Submerged mining speed`, `Sweeping damage ratio`, `Zombie spawn reinforcements`
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
- _Flags:_ advanced

### `SetYaw`
**Set Yaw**
> ⚠ **LEGACY** → use `SetCoord` (Set Coordinate tag to Yaw.)
Sets the yaw (left/right) rotation of a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to change
   - `[2]` `NUMBER` — Yaw
- **Returns:** `LOCATION` Modified location

### `-=`
**Decrement Number (-=)**
Decrements a number variable by 1 or more other numbers.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to decrement
   - `[1]` `NUMBER` _(optional, plural)_ — Number(s) to decrement by — _Default = 1_
- **Returns:** `NUMBER` Result

### `GetItemTag`
**Get Item Custom Tag**
Gets the value of a custom item tag.
- _Info:_ Returns 0 if the tag is not present.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get tag of
   - `[2]` `TEXT` — Tag name
- **Returns:** `ANY_TYPE` Tag value

### `AppendList`
**Append List to List**
Adds a list to the end of another list variable.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — List to append to
   - `[1]` `LIST` _(plural)_ — List(s) to append

### `GetBookAuthor`
**Get Book Author**
Gets a book's author.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Book
- **Returns:** `TEXT` Author

### `TrimList`
**Trim List**
Trims a list, starting and ending at the given indices.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` _(optional)_ — List to trim
   - `[2]` `NUMBER` — Start index
   - `[3]` `NUMBER` _(optional)_ — End index — _Default = List length_
- **Returns:** `LIST` Trimmed list

### `GradientNoise`
**Get Gradient Noise**
Gets gradient noise: A type of noise based on a lattice of random gradients.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Noise location
   - `[2]` `NUMBER` _(optional)_ — Frequency — _Modifies the scale that the noise generates at. Default = 1_
   - `[3]` `NUMBER` _(optional)_ — Marbling — _Every 0.5 adds an additional ring to the noise output. Default = 0.5_
   - `[4]` `NUMBER` _(optional)_ — Octaves — _Adds an additional layer of gradient noise to the output. 1-16, Default = 1_
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
   - `Return Type` (slot 26): `Simplex` (default), `Perlin`
       - `Simplex`: Provides the average of a honeycomb of gradient vectors.  Using a honeycomb instead of a lattice like that used in Perlin Noise, removes directional artifacts and increases efficiency.
       - `Perlin`: Provides the average of a lattice of gradient vectors.  Using vectors rather than values like those seen in Value Noise, provides more detail at the cost of lower efficiency.
- **Returns:** `NUMBER` A decimal value between 0 and 1
- _Flags:_ advanced

### `GetBlockSound`
**Get Block Sound**
Gets the sound for a block sound group.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `BLOCK` — Block to get sound from
- **Tags:**
   - `Sound Type` (slot 26): `Break Sound` (default), `Place Sound`, `Hit Sound`, `Fall Sound`, `Step Sound`
       - `Break Sound`: Sound played when the block is broken.
       - `Place Sound`: Sound played when the block is placed.
       - `Hit Sound`: Sound played when the block is hit.
       - `Fall Sound`: Sound played when something falls on the block.
       - `Step Sound`: Sound played when something steps on the block.
- **Returns:** `SOUND` Block sound group sound

### `NamedRegexGrps`
**Get All Named Regex Groups**
Gets a dictionary of all named capture groups in a Regex match.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Input string
   - `[2]` `TEXT` — Regex pattern
- **Returns:** `DICT` Dictionary of all named capture groups
- _Flags:_ advanced

### `ClearEnchants`
**Clear Item Enchantments**
Removes enchantments from an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change

### `Cosine`
**Set to Cosine**
Sets a variable to the trigonometric cosine function of a number.
- _Info:_ Use the variant tag to swap between cos, acos and cosh functions.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `NUMBER` — Number input
- **Tags:**
   - `Cosine Variant` (slot 25): `Cosine` (default), `Inverse cosine (arccosine)`, `Hyperbolic cosine`
   - `Input` (slot 26): `Degrees` (default), `Radians`
- **Returns:** `NUMBER` Result
- _Flags:_ advanced

### `GetBreakSound`
**Get Item Break Sound**
Gets which sound an item makes when it breaks.
- _Info:_ Pitch and volume are set to 1.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get break sound from
- **Returns:** `SOUND` Break sound

### `GetItemByMCTag`
**Get Items by Minecraft Tag**
Gets a list of items that belongs to the specified item tag.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Item tag ID — _Example: "#enchantable/sword"_
- **Tags:**
   - `Return Value Tyep` (slot 26): `Material ID (golden_apple)` (default), `Material Name (Golden Apple)`, `Item`
- **Returns:** `LIST` Materials
- _Flags:_ advanced

### `SetParticleFade`
**Set Particle Effect Fade Color**
Sets a particle effect's particle fade color.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `PARTICLE` _(optional)_ — Effect to change
   - `[2]` `TEXT` — Color hexadecimal — _Example: "#FF0000" (red)_

### `SetVectorComp`
**Set Vector Component**
Sets a vector's X, Y, or Z component.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `VECTOR` _(optional)_ — Vector to change
   - `[2]` `NUMBER` — Component
- **Tags:**
   - `Component` (slot 26): `X` (default), `Y`, `Z`

### `ParseNumber`
**Parse Number from String**
Converts a string to a number.
- _Info:_ If the string is not a valid number, the result is 0.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` _(optional)_ — String to convert
- **Returns:** `NUMBER` Parsed number

### `SetItemWeapon`
**Set Weapon Properties**
Adds weapon properties to an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `NUMBER` _(optional)_ — Durability reduction per attack — _Default = 1_
   - `[3]` `NUMBER` _(optional)_ — Seconds to disable blocking for — _Default = 0_

### `RemItemEnchant`
**Remove Item Enchantment**
Removes an enchantment from an item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` — Enchantment name

### `GetConsumable`
**Get Consumable Property** · aliases: `GetItemFood`
Gets an item's consumable property.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get properties of
- **Tags:**
   - `Consumable Property` (slot 26): `Nutrition` (default), `Saturation`, `Use Duration`, `Animation`, `Sound`
- **Returns:** `NUMBER` Property value; `None` ; `TEXT` Property value; `None` ; `SOUND` Property value

### `SetSoundVariant`
**Set Sound Variant**
Sets the variant of a sound.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `SOUND` _(optional)_ — Sound to change
   - `[2]` `TEXT` _(optional)_ — Variant ID (e.g. "break1")

### `ShiftLocation`
**Shift Location**
> ⚠ **LEGACY** → use `ShiftAllDirections`
Shifts a location forward, upward, or sideways based on its rotation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to shift
   - `[2]` `NUMBER` — Shift distance
- **Tags:**
   - `Shift Direction` (slot 26): `(+) Upwards / (-) Downwards`, `(+) Forwards / (-) Backwards` (default), `(+) Right / (-) Left`

### `RandomizeList`
**Randomize List**
Randomizes the order of a list variable's values.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LIST` _(optional)_ — List to randomize
- **Returns:** `LIST` Randomized list

### `HiddenComponents`
**Set Item Hidden Components** · aliases: `HideComponents`
Sets an item's components that should show up in its lore.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `TEXT` _(optional, plural)_ — Item component(s) to hide — _Item component keys, Example: "attribute_modifiers"_
- **Tags:**
   - `Hiding Mode` (slot 26): `Hide`, `Show`, `Hide all` (default), `Show all`, `Hide all except`, `Show all except`

### `Round`
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

### `GetItemModel`
**Get Item Model**
Gets an item's model.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get model of
- **Returns:** `TEXT` Model key, e.g. "minecraft:stone"

### `GetBlockGrowth`
**Get Block Growth**
Gets a block's current growth at a location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Block location
- **Tags:**
   - `Growth Unit` (slot 26): `Growth stage number` (default), `Growth percentage`
- **Returns:** `NUMBER` Growth level

### `BytesToString`
**Convert Bytes to String**
Converts a list of numbers representing bytes to a string.
- _Info:_ Returns 0 if the result is too large.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `BYTE` _(plural)_ — Byte(s) to convert
- **Tags:**
   - `Signed` (slot 26): `True` (default), `False`
   - `Charset` (slot 25): `UTF-8` (default), `ASCII`
- **Returns:** `TEXT` Converted value
- _Flags:_ advanced

### `ShiftAllDirections`
**Shift Location in All Directions**
Shifts a location in multiple directions based on its rotation.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` _(optional)_ — Location to shift
   - `[2]` `NUMBER` _(optional)_ — Forwards change
   - `[3]` `NUMBER` _(optional)_ — Upwards change
   - `[4]` `NUMBER` _(optional)_ — Sideways change (-L / +R)

### `DialogTitle`
**Set Dialog Title**
Sets the title of a dialog.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Dialog to change
   - `[1]` `COMPONENT` — Title

### `SetAllItemTags`
**Set All Item Custom Tags**
Sets custom tags on an item based on the input dictionary.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `DICT` — Tags

### `ContainerLock`
**Get Container Lock**
Gets a container's lock key at a location.
- _Info:_ Returns "none" if the container is not locked.
- _Info:_ Returns 0 if the container is not lockable.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Container location
- **Returns:** `TEXT` Lock key; `None` ; `NUMBER` 0 (Not lockable)

### `GetItemWeapon`
**Get Weapon Property**
Gets an item's weapon property.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` — Item to get property of
- **Tags:**
   - `Weapon Property` (slot 26): `Durability reduction per attack` (default), `Seconds to disable blocking for`
- **Returns:** `NUMBER` Property value

### `GetBlockPower`
**Get Block Power**
Gets a block's redstone power level.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `LOCATION` — Block location
- **Returns:** `NUMBER` Power level

### `SetModelData`
**Set Custom Model Data**
> ⚠ **LEGACY** → use `SetModelDataNums`
Sets the model value used in resource packs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `ITEM` _(optional)_ — Item to change
   - `[2]` `NUMBER` — Model value
