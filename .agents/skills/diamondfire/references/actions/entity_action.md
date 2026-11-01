# DiamondFire actions — Entity Action (`entity_action`)

_Generated from actiondump (patch 8.13). Action names are exact wire strings; names in double quotes include **leading/trailing spaces** that must be kept._

## Block `entity_action` — Entity Action (213 actions in this file)

Used to do something related to an entity or multiple entities.

### `DispRotationEuler`
**Set Display Rotation from Euler Angles**
Sets the left or right rotation of a display entity from 3 angles on each axis.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Pitch (0-360)
   - `[1]` `NUMBER` — Yaw (0-360)
   - `[2]` `NUMBER` — Roll (0-360)
   - **OR**
   - `[3]` `VECTOR` — Pitch/Yaw/Roll Vector
- **Tags:**
   - `Rotation Type` (slot 26): `Left Rotation` (default), `Right Rotation`
       - `Left Rotation`: Applies before scaling the display entity when rendering.
       - `Right Rotation`: Applies after scaling the display entity when rendering.
- _Flags:_ rank: Mythic, advanced

### `MannequinLayers`
**Set Mannequin Skin Layers**
Sets a mannequin's skin layers.
- _Works with:_ Mannequin
- **Tags:**
   - `Cape Layer` (slot 26): `Visible` (default), `Hidden`
   - `Jacket Layer` (slot 25): `Visible` (default), `Hidden`
   - `Left Sleeve Layer` (slot 24): `Visible` (default), `Hidden`
   - `Right Sleeve Layer` (slot 23): `Visible` (default), `Hidden`
   - `Left Pants Layer` (slot 22): `Visible` (default), `Hidden`
   - `Right Pants Layer` (slot 21): `Visible` (default), `Hidden`
   - `Hat Layer` (slot 20): `Visible` (default), `Hidden`
- _Flags:_ rank: Overlord

### `Shear`
**Shear**
Sets a mob in the sheared state.
- _Works with:_ Bogged; Mooshroom; Sheep; Snow Golem
- _Info:_ Shearing some mobs may turn them into different mob types.

### `SetVelocity`
**Set Velocity**
Sets an entity's movement velocity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VECTOR` — New velocity
- **Tags:**
   - `Add to Current Velocity` (slot 26): `True`, `False` (default)

### `SetGlowSquidDark`
**Set Glow Squid Dark Ticks**
Sets the number of ticks a glow squid will stop glowing for.
- _Works with:_ Glow Squid
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks

### `DispRotAxisAngle`
**Set Display Rotation from Axis-Angle**
Sets the left or right rotation of a display entity from axis-angle rotation.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VECTOR` — Axis vector
   - `[1]` `NUMBER` — Angle (0-360)
- **Tags:**
   - `Rotation Type` (slot 26): `Left Rotation` (default), `Right Rotation`
       - `Left Rotation`: Applies before scaling the display entity when rendering.
       - `Right Rotation`: Applies after scaling the display entity when rendering.
- _Flags:_ rank: Mythic, advanced

### `SetFrogType`
**Set Frog Variant**
> ⚠ **LEGACY** → use `SetTemperature`
Sets a frog's temperature variant.
- _Works with:_ Frog
- **Tags:**
   - `Frog Type` (slot 26): `Temperate` (default), `Warm`, `Cold`
- _Flags:_ rank: Noble

### `Damage`
**Damage**
Damages a mob.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Damage to inflict — _❤ = 2 Health_
   - **OR**
   - `[1]` `TEXT` _(optional)_ — UUID of damager entity
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `SetMobSitting`
**Set Sitting**
Sets whether an entity is sitting.
- _Works with:_ Camel; Cat; Fox; Parrot; Wolf
- _Info:_ Works on both tamed and untamed mobs.
- **Tags:**
   - `Is Sitting` (slot 26): `Enable` (default), `Disable`

### `SendAnimation`
**Send Mob Animation**
Makes a mob perform an animation.
- _Works with:_ Any Mob
- **Tags:**
   - `Animation Type` (slot 26): `Hurt animation` (default), `Crit particles`, `Enchanted hit particles`
- _Flags:_ rank: Noble

### `DisableGlowing`
**Disable Glowing**
> ⚠ **LEGACY** → use `SetGlowing` (Set Glowing tag to Disable.)
Makes an entity no longer glow.

### `SetWardenAnger`
**Set Warden Anger Level**
Sets the anger level of a Warden.
- _Works with:_ Warden
- _Info:_ If the anger level reaches 80, the warden will actively purse the target.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Anger level (0-150)
   - **OR**
   - `[1]` `TEXT` — Entity UUID
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `SetHorsePattern`
**Set Horse Pattern**
Sets a horse's color and pattern.
- _Works with:_ Horse
- **Tags:**
   - `Horse Color` (slot 25): `White`, `Buckskin`, `Flaxen chestnut` (default), `Bay`, `Black`, `Dapple gray`, `Dark bay`, `Don't change`
   - `Horse Markings` (slot 26): `No markings`, `Stockings and blaze` (default), `Paint`, `Snowflake appaloosa`, `Sooty`, `Don't change`
- _Flags:_ rank: Noble

### `Heal`
**Heal**
Restores a mob's health.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Amount to heal — _❤ = 2 Health_
   - **OR**
   - `[1]` `NONE` — Heals to full health)

### `SetPandaSadTicks`
**Set Panda Sad Ticks**
Makes a panda sad for the specified duration.
- _Works with:_ Panda
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks

### `OxidizeTicksLeft`
**Set Copper Golem Oxidization Ticks Left**
Sets the amount of ticks until a copper golem will next oxidize.
- _Works with:_ Copper Golem
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks

### `SetItemOwner`
**Set Item Owner**
> ⚠ **LEGACY** → use `PickUpItem` (Use Player Pick Up Item Event, use Game Action: Cancel Event to cancel picking up depending on the player.)
Sets the owner of a dropped item. Other players will not be able to pick up the item.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Owner UUID

### `SetDyeColor`
**Set Dye Color**
Sets a mob's dye color.
- _Works with:_ Sheep; Shulker; Dog (collar); Cat (collar)
- **Tags:**
   - `Dye` (slot 26): `White` (default), `Orange`, `Magenta`, `Light blue`, `Yellow`, `Lime`, `Pink`, `Gray`, `Light gray`, `Cyan`, `Purple`, `Blue`, `Brown`, `Green`, `Red`, `Black`
- _Flags:_ rank: Noble

### `LaunchUp`
**Launch Up**
Launches an entity up or down.
- _Info:_ A negative launch power will launch the entity down.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Launch power
- **Tags:**
   - `Add to Current Velocity` (slot 26): `True` (default), `False`

### `SetAge`
**Set Animal Age**
Sets an animal's age.
- _Works with:_ Any Breedable
- _Info:_ 0: Target is an adult that can breed. Age stays at 0.
- _Info:_ -1 and below: Target is a baby. Age goes up each tick.
- _Info:_ 1 and above: Target is an adult that can't breed. Age goes down each tick.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Age
- **Tags:**
   - `Age Lock` (slot 26): `Enable`, `Disable`, `Don't change` (default)

### `" SetName "`
**Set Custom Name**
Sets an entity's custom name.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Custom name
   - **OR**
   - `[1]` `NONE` — Removes custom name)
- **Tags:**
   - `Name Tag Visibility` (slot 26): `Always` (default), `Default`, `Never`, `Don't change`
       - `Always`: The custom name is always shown.
       - `Default`: The custom name is shown when you are looking at the entity.
       - `Never`: The custom name is always hidden.

### `NoGravity`
**Disable Gravity**
> ⚠ **LEGACY** → use `SetGravity` (Set Gravity tag to Disabled.)
Disables gravity for the entity.
- _Flags:_ rank: Overlord

### `SetArmsRaised`
**Set Arms Raised**
Sets whether a mob has its arms raised.
- _Works with:_ Any Zombie
- **Tags:**
   - `Arms Raised` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `SetMoveSpeed`
**Set Movement Speed**
> ⚠ **LEGACY** → use `MovementAttribute` (Set walking/flying speed attributes.)
Sets a mob's movement speed.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Movement speed percentage (0% to 1000%)
- **Tags:**
   - `` (slot 26): `Enable` (default), `Disable`
       - `Enable`: Move speed will be relative to the base move speed of the given type of entity.
       - `Disable`: Directly sets the move speed attribute.
- _Flags:_ rank: Noble

### `SetInvulnerable`
**Set Invulnerable**
Sets whether an entity is invulnerable to damage.
- _Info:_ An invulnerable entity does not trigger damage events.
- **Tags:**
   - `Invulnerable` (slot 26): `Enable` (default), `Disable`

### `SetFriction`
**Set Friction**
Changes the type of friction an entity experiences.
- **Tags:**
   - `Friction Type` (slot 26): `Normal`, `No Friction` (default)
       - `Normal`: The entity has normal friction.
       - `No Friction`: The entity isn't slowed down by friction with the ground.

### `ProjColl`
**Enable Projectile Collisions**
> ⚠ **LEGACY** → use `SetCollidable` (Set Collision tag to Enable.)
Allows projectiles to hit the mob.
- _Works with:_ Any Mob

### `ArmorStandTags`
**Set Armor Stand Tags**
> ⚠ **LEGACY** → use `ArmorStandParts`
Changes the settings of an armor stand, such as visibility.
- _Works with:_ Armor stand
- **Tags:**
   - `Is Visible` (slot 20): `True`, `False`, `Don't change` (default)
   - `Is Marker (No Hitbox)` (slot 21): `True`, `False`, `Don't change` (default)
   - `Allow Item Taking / Adding` (slot 22): `True`, `False`, `Don't change` (default)
   - `Has Physics / Updates` (slot 23): `True`, `False`, `Don't change` (default)
   - `Is Small` (slot 24): `True`, `False`, `Don't change` (default)
   - `Has Arms` (slot 25): `True`, `False`, `Don't change` (default)
   - `Has Base Plate` (slot 26): `True`, `False`, `Don't change` (default)
- _Flags:_ rank: Mythic

### `SetPickupDelay`
**Set Pickup Delay**
Sets the number of ticks a dropped item cannot be picked up for.
- _Info:_ If set to 0 upon spawning, the item can be picked up immediately.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Delay

### `MannequinHand`
**Set Mannequin Main Hand**
Sets a mannequin's main hand.
- _Works with:_ Mannequin
- **Tags:**
   - `Main Hand` (slot 26): `Left Hand` (default), `Right Hand`
- _Flags:_ rank: Overlord

### `DropItems`
**Enable Death Drops**
> ⚠ **LEGACY** → use `SetDeathDrops` (Set Has Death Drops tag to Enable.)
After this code block is executed, the mob will drop their equipment and loot when they die.
- _Works with:_ Any Mob

### `MiscAttribute`
**Set Miscellaneous Attribute**
Sets one of the entity's miscellaneous attributes such as scale and burning time.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Scale` (default), `Follow range`, `Zombie spawn reinforcements`, `Oxygen bonus`, `Burning time`, `Camera distance`, `Name tag distance`, `Tempt range`
       - `Scale`: Base: 1.0 Range: 0.0625-16.0
       - `Follow range`: Base: 32.0 Range: 0.0-2048.0
       - `Zombie spawn reinforcements`: Base: 0.0 Range: 0.0-1.0
       - `Oxygen bonus`: Base: 0.0 Range: 0.0-1024.0
       - `Burning time`: Base: 1.0 Range: 0.0-1024.0
       - `Camera distance`: Base: 4.0 Range: 0.0-32.0
       - `Name tag distance`: Base: 64.0 Range: 0.0-512.0
       - `Tempt range`: Base: 10.0 Range: 0.0-2048.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `SetCreeperPower`
**Set Creeper Explosion Power**
Sets a creeper's explosion power. This affects the damage and area of effect.
- _Works with:_ Creeper
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Power (0-25) — _Default = 3 (6 for charged creepers)_

### `SetMarker`
**Set Marker**
Sets whether an armor stand is a marker.
- **Tags:**
   - `Marker` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Mythic

### `RemoveCustomTag`
**Remove Entity Custom Tag**
Removes a custom tag from an entity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Tag name

### `SetAbsorption`
**Set Absorption Health**
Sets an entity's absorption health (golden hearts).
- _Works with:_ Any Mob
- _Info:_ The target does not need to have an absorption effect.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Absorption health — _❤ = 2 Health_

### `CreeperCharged`
**Set Creeper Charged**
Sets whether a creeper has the charged effect.
- _Works with:_ Creeper
- **Tags:**
   - `Charged` (slot 26): `Enable` (default), `Disable`

### `SetFireTicks`
**Set Fire Ticks**
Sets the remaining time an entity is on fire for.
- _Info:_ 0 ticks mean the target is not on fire.
- _Info:_ Ghasts, magma cubes, vexes, withers, wither skulls, tnt, falling blocks, and shulker bullets cannot be set on fire.
- _Info:_ Blazes do not rely on fire ticks to display the fire effect.
- _Info:_ This triggers the Entity Combust event.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks

### `CombatAttribute`
**Set Combat Attribute**
Sets one of the entity's combat-related attributes such as attack damage and attack speed.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Attack damage` (default), `Attack knockback`
       - `Attack damage`: Base: 2.0 Range: 0.0-2048.0
       - `Attack knockback`: Base: 0.0 Range: 0.0-5.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `SetName`
**Set Custom Name**
> ⚠ **LEGACY** → use `" SetName "`
Sets an entity's custom name.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Custom name
   - **OR**
   - `[1]` `NONE` — Removes custom name
- **Tags:**
   - `Hide Name Tag` (slot 26): `True`, `False` (default), `Don't change`

### `Jump`
**Jump**
Causes a mob to jump.
- _Works with:_ Any Mob

### `MannequinDesc`
**Set Mannequin Description**
Sets a mannequin's description.
- _Works with:_ Mannequin
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(optional)_ — Description — _Only shown if name is visible_
- _Flags:_ rank: Overlord

### `BDisplayBlock`
**Set Block Display Block**
Sets the displayed block of a block display.
- _Works with:_ Block Display
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Displayed block
   - `[1]` `BLOCK_TAG` _(optional, plural)_ — Block data — _Example: "facing=up", "half=top"_
- _Flags:_ rank: Mythic

### `SetFireworkTicks`
**Set Firework Detonation Ticks**
Sets the amount of ticks until a firework detonates.
- _Works with:_ Fireworks
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks to detonate

### `SetFreezeTicks`
**Set Freeze Ticks**
Sets an entity's current freeze ticks.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks (0-140)
- **Tags:**
   - `Ticking Locked` (slot 26): `Enable`, `Disable` (default)
       - `Enable`: Entity will stay frozen.
       - `Disable`: Entity will slowly unfreeze over time.

### `TDisplaySeeThru`
**Set Text Display See-through**
Sets whether a text display is visible through walls or not.
- _Works with:_ Text Display
- **Tags:**
   - `See-through` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Mythic

### `SetGliding`
**Set Gliding**
Sets whether an entity is gliding.
- _Works with:_ Any Mob
- _Info:_ The entity must be wearing an elytra.
- **Tags:**
   - `Gliding` (slot 26): `Enable` (default), `Disable`

### `SetRotation`
**Set Rotation**
Changes an entity's pitch and yaw without teleporting it.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Pitch (-90 to 90)
   - `[1]` `NUMBER` — Yaw (-180 to 180)

### `VillagerHeadAnim`
**Send Villager Head Shake Animation**
Makes a villager perform a head shake animation.
- _Works with:_ Villager

### `SetPandaRolling`
**Set Panda Rolling**
Sets whether a panda is rolling or not.
- _Works with:_ Panda
- **Tags:**
   - `Roll Type` (slot 26): `Roll` (default), `Stop Rolling`
       - `Roll`: Makes the panda do a single roll. Does nothing if already rolling.
       - `Stop Rolling`: Makes the panda stop rolling if it's already rolling.

### `SetFishPattern`
**Set Tropical Fish Pattern**
Sets a tropical fish's color and pattern.
- _Works with:_ Tropical Fish
- **Tags:**
   - `Pattern Color` (slot 24): `White` (default), `Orange`, `Magenta`, `Light blue`, `Yellow`, `Lime`, `Pink`, `Gray`, `Light gray`, `Cyan`, `Purple`, `Blue`, `Brown`, `Green`, `Red`, `Black`, `Don't change`
   - `Body Color` (slot 25): `White` (default), `Orange`, `Magenta`, `Light blue`, `Yellow`, `Lime`, `Pink`, `Gray`, `Light gray`, `Cyan`, `Purple`, `Blue`, `Brown`, `Green`, `Red`, `Black`, `Don't change`
   - `Pattern` (slot 26): `Kob` (default), `Sunstreak`, `Snooper`, `Dasher`, `Brinely`, `Spotty`, `Flopper`, `Stripey`, `Glitter`, `Blockfish`, `Betty`, `Clayfish`, `Don't change`
- _Flags:_ rank: Noble

### `SetTemperature`
**Set Temperature Type**
Sets a mob's temperature variant.
- _Works with:_ Chicken; Cow; Frog; Pig; Zombie Nautilus
- **Tags:**
   - `Temperature Type` (slot 26): `Cold`, `Temperate` (default), `Warm`
- _Flags:_ rank: Noble

### `SetWolfType`
**Set Wolf Type**
Sets a wolf's variant.
- _Works with:_ Wolf
- **Tags:**
   - `Wolf Type` (slot 26): `Ashen` (default), `Black`, `Chestnut`, `Pale`, `Rusty`, `Snowy`, `Spotted`, `Striped`, `Woods`
- _Flags:_ rank: Noble

### `DispInterpolation`
**Set Display Interpolation**
Sets the interpolation properties of a display entity.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Interpolation duration in ticks — _Default = 0_
   - `[1]` `NUMBER` _(optional)_ — Interpolation delay in ticks — _Default = 0_
- _Flags:_ rank: Mythic

### `" SetEquipment "`
**Set Equipment Item**
Sets the item in one of the equipment slots (including horse items) of an entity.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional)_ — Item to set
- **Tags:**
   - `Equipment Slot` (slot 26): `Main hand` (default), `Off hand`, `Head`, `Chest`, `Legs`, `Feet`, `Body`, `Saddle`
       - `Main hand`: Sets the hand item of a mob with a hand slot.
       - `Off hand`: Sets the off hand item of a mob with two hand slots.
       - `Head`: Sets the head armor item of a mob that can wear hats or armor.
       - `Chest`: Sets the chest armor item of a mob that can wear armor.
       - `Legs`: Sets the legs armor item of a mob that can wear armor.
       - `Feet`: Sets the feet armor item of a mob that can wear armor.
       - `Body`: Sets the body slot of a mob.
       - `Saddle`: Sets the saddle item of any rideable mob.

### `SetHandItem`
**Set Hand Item**
> ⚠ **LEGACY** → use `" SetEquipment "` (Set Equipment Slot tag to Main hand or Off hand.)
Sets the item in a mob's main hand or off hand.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional)_ — Item to set
- **Tags:**
   - `Hand Slot` (slot 26): `Main Hand` (default), `Off Hand`

### `SetEndermanBlock`
**Set Enderman Held Block**
Set an enderman's held block.
- _Works with:_ Enderman
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Block to hold
- _Flags:_ rank: Noble

### `MannequinMovable`
**Set Mannequin Movable**
Sets whether a mannequin is movable.
- _Works with:_ Mannequin
- **Tags:**
   - `Movable` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Overlord

### `LaunchToward`
**Launch Toward Location**
Launches an entity toward or away from a location.
- _Info:_ A negative launch power will launch the entity away from the location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Launch destination
   - `[1]` `NUMBER` _(optional)_ — Launch power
- **Tags:**
   - `Add to Current Velocity` (slot 25): `True` (default), `False`
   - `Ignore Distance` (slot 26): `True`, `False` (default)

### `SetArmor`
**Set Armor Items** · aliases: `" SetArmor "`
Sets a mob's armor items. Place the armor in slots 1-4 of the chest, with 1 being the helmet and 4 being the boots.
- _Works with:_ Any Mob
- _Info:_ Any block or item can render on the target's head if placed in the 'helmet' slot.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(plural)_ — Armor to set

### `GetCustomTag`
**Get Entity Custom Tag**
Gets the value of a custom entity tag.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
   - `[1]` `TEXT` — Tag name
- **Returns:** `ANY_TYPE` Tag value

### `InteractionSize`
**Set Interaction Size**
Sets the hitbox size of an interaction entity.
- _Works with:_ Interaction Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Width — _Default = 1_
   - `[1]` `NUMBER` _(optional)_ — Height — _Default = 1_

### `FaceLocation`
**Face Location**
Rotates an entity to look toward a location without teleporting them.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Location to face
- **Tags:**
   - `Face Direction` (slot 26): `Toward location` (default), `Away from location`

### `SetCatType`
**Set Cat Type**
Sets a cat's skin type.
- _Works with:_ Cat
- **Tags:**
   - `Skin Type` (slot 26): `Tabby` (default), `Tuxedo`, `Red`, `Siamese`, `British Shorthair`, `Calico`, `Persian`, `Ragdoll`, `White`, `Jellie`, `Black`
- _Flags:_ rank: Noble

### `SetArrowDamage`
**Set Base Arrow Damage**
Sets the base damage dealt by an arrow or trident.
- _Works with:_ Arrows, Tridents
- _Info:_ Base Arrow Damage: 2
- _Info:_ Base Thrown Trident Damage: 8
- _Info:_ Total Arrow Damage: ||velocity|| x base
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Base Damage

### `DisplayScale`
**Set Display Scale**
Sets the scale of a display entity.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — X scale
   - `[1]` `NUMBER` — Y scale
   - `[2]` `NUMBER` — Z scale
   - **OR**
   - `[3]` `VECTOR` — Scale vector
- _Flags:_ rank: Mythic, advanced

### `TDisplayAlign`
**Set Text Display Text Alignment**
Sets the text alignment of a text display.
- _Works with:_ Text Display
- **Tags:**
   - `Text Alignment` (slot 26): `Center` (default), `Left`, `Right`
- _Flags:_ rank: Mythic

### `LaunchProj`
**Launch Projectile**
Launches a projectile from a mob.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `PROJECTILE` — Projectile to launch
   - `[1]` `LOCATION` _(optional)_ — Launch point
   - `[2]` `COMPONENT` _(optional)_ — Projectile name
   - `[3]` `NUMBER` _(optional)_ — Speed
   - `[4]` `NUMBER` _(optional)_ — Inaccuracy — _Controls how much random motion is applied on launch Default = 1_

### `EnableAI`
**Enable AI**
> ⚠ **LEGACY** → use `SetAI` (Set AI tag to Sentient.)
Enables the AI of the mob.
- _Works with:_ Any Mob
- _Flags:_ rank: Noble

### `DisplayBillboard`
**Set Display Billboard**
Sets how a display entity is rotated with a player's view.
- _Works with:_ Any Display Entity
- **Tags:**
   - `Billboard Type` (slot 26): `Fixed` (default), `Vertical`, `Horizontal`, `Center`
       - `Fixed`: Display entity does not rotate.
       - `Vertical`: Display entity faces player around vertical axis.
       - `Horizontal`: Display entity faces player around horizontal axis.
       - `Center`: Display entity faces player in all axes.
- _Flags:_ rank: Mythic

### `NoProjColl`
**Disable Projectile Collisions**
> ⚠ **LEGACY** → use `SetCollidable` (Set Collision tag to Disable.)
Prevents projectiles from hitting the mob.
- _Works with:_ Any Mob

### `Tame`
**Tame**
Tames and sets the owner of a tameable mob.
- _Works with:_ Wolf; Cat; Parrot; Llama; Any Horse
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Owner UUID
   - **OR**
   - `[1]` `NONE` — Untames mob

### `SetGoatScreaming`
**Set Goat Screaming**
Sets whether a goat screams or not.
- _Works with:_ Goat
- **Tags:**
   - `Screams` (slot 26): `Enable` (default), `Disable`

### `SetBeeStinger`
**Set Bee Has Stinger**
Sets whether a bee has its stinger.
- _Works with:_ Bee
- **Tags:**
   - `Has Stinger` (slot 26): `Enable` (default), `Disable`

### `RestockTrades`
**Restock Villager Trades**
Restocks all of a villager's trades.
- _Works with:_ Villager; Wandering Trader

### `MobDisguise`
**Disguise as Mob**
Disguises an entity as a mob.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `SPAWN_EGG` — Mob to disguise as
   - `[1]` `COMPONENT` _(optional)_ — Display name
- _Flags:_ rank: Overlord

### `SetMinecartBlock`
**Set Minecart Block**
Sets the block shown inside a minecart. This does not affect its functionality.
- _Works with:_ Any Minecart
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Block to show
   - `[1]` `NUMBER` _(optional)_ — Block offset
- _Flags:_ rank: Noble

### `FoxSleeping`
**Set Fox Sleeping**
Causes a fox to start or stop sleeping.
- _Works with:_ Fox
- **Tags:**
   - `Sleeping` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

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

### `SetEquipment`
**Set Equipment Item**
Sets the item in one of the equipment slots (including horse items) of an entity.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional)_ — Item to set
- **Tags:**
   - `Equipment Slot` (slot 26): `Main hand` (default), `Off hand`, `Head`, `Body`, `Legs`, `Feet`, `Saddle`, `Horse armor`, `Decor`, `Harness`
       - `Main hand`: Sets the hand item of a mob with a hand slot.
       - `Off hand`: Sets the off hand item of a mob with two hand slots.
       - `Head`: Sets the head armor item of a mob that can wear hats or armor.
       - `Body`: Sets the chest armor item of a mob that can wear armor.
       - `Legs`: Sets the legs armor item of a mob that can wear armor.
       - `Feet`: Sets the feet armor item of a mob that can wear armor.
       - `Saddle`: Sets the saddle item of a horse.
       - `Horse armor`: Sets the armor item of a horse.
       - `Decor`: Sets the decor item of a llama.
       - `Harness`: Sets the harness item of a happy ghast.

### `SetSilenced`
**Set Silenced**
Sets whether an entity will produce sound effects.
- **Tags:**
   - `Silenced` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `SetBeeNectar`
**Set Bee Has Nectar**
Sets if a bee has nectar on its body.
- _Works with:_ Bee
- **Tags:**
   - `Has Nectar` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `AttachLead`
**Attach Lead**
Attaches a lead to the target, held by an entity or lead knot.
- _Works with:_ Mobs; Boats
- _Info:_ Max. attach range is 10 blocks.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Lead holder UUID
   - **OR**
   - `[1]` `LOCATION` — Lead knot location — _Spawns a Leash Knot entity. Requires a fence block._
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `RemovePotion`
**Remove Potion Effect** · aliases: `RemoveEffect`
Removes one or more potion effects from an entity.
- _Works with:_ Any Mob
- _Info:_ Only the potion's type needs to match; amplifier and duration are disregarded.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `POTION` _(plural)_ — Effect(s) to remove

### `ShearSheep`
**Shear Sheep**
Causes a sheep to be sheared.
- _Works with:_ Sheep

### `ArmorStandSlots`
**Set Armor Stand Slot Interactions**
Sets the possible interactions, such as adding or removing items, of an armor stand's slot(s).
- _Works with:_ Armor Stand
- **Tags:**
   - `Interactions` (slot 25): `Take, swap or place item` (default), `Take or swap item`, `Take item`, `Place item`, `None`
       - `Take, swap or place item`: Players are able to: • Remove an item from this slot • Swap their held item with this slot • Place their held item in this slot
       - `Take or swap item`: Players are able to: • Remove an item from this slot • Swap their held item with this slot
       - `Take item`: Players are able to: • Remove an item from this slot
       - `Place item`: Players are able to: • Place their held item in this slot (if the slot is empty)
       - `None`: Players are unable to interact with this slot.
   - `Equipment Slot` (slot 26): `All` (default), `Main hand`, `Off hand`, `Head`, `Chest`, `Legs`, `Feet`
- _Flags:_ rank: Mythic

### `SetAllayDancing`
**Set Allay Dancing**
Sets whether an allay is dancing or not.
- _Works with:_ Allay
- **Tags:**
   - `Dancing` (slot 26): `Enable` (default), `Disable`

### `SetRabbitType`
**Set Rabbit Type**
Sets a rabbit's skin type.
- _Works with:_ Rabbit
- **Tags:**
   - `Skin Type` (slot 26): `Brown` (default), `White`, `Black`, `Black and White`, `Gold`, `Salt and Pepper`, `Killer`
- _Flags:_ rank: Noble

### `SetSize`
**Set Size**
Sets the size of an entity. This may also affect its health and strength.
- _Works with:_ Slime; Magma Cube; Sulfur Cube; Phantom; Pufferfish
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Size

### `ShowName`
**Show Name Tag**
> ⚠ **LEGACY** → use `" SetNameVisible "` (Set Name Tag Visibility tag to Always.)
Shows the name tag of an entity.

### `SetAngry`
**Set Angry** · aliases: `SetAngerTicks`
Sets whether a mob is angry at players.
- _Works with:_ Bee; Wolf; Zombified Piglin
- **Tags:**
   - `Angry` (slot 26): `Enable` (default), `Disable`

### `Undisguise`
**Undisguise**
Removes an entity's disguise.
- _Flags:_ rank: Overlord

### `SetDeathDrops`
**Set Death Drops Enabled**
Sets whether a mob drops their items when dead.
- _Works with:_ Any Mob
- **Tags:**
   - `Has Death Drops` (slot 26): `Enable` (default), `Disable`

### `SetPersistent`
**Set Persistent**
Sets whether an item or a falling block will never despawn.
- _Works with:_ Dropped Item; Falling Block
- **Tags:**
   - `Persistent` (slot 26): `Enable` (default), `Disable`

### `ProjectileItem`
**Set Projectile Display Item**
Sets the item a projectile displays as.
- _Works with:_ Snowball; Egg; Small Fireball; Ghast Fireball; Ender Pearl; Experience Bottle; Eye of Ender
- _Info:_ Does not work with air.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Display item
- _Flags:_ rank: Noble

### `SetNameColor`
**Set Name Color**
Sets the color an entity's name tag appears in.
- **Tags:**
   - `Name Color` (slot 26): `Black` (default), `Dark blue`, `Dark green`, `Dark aqua`, `Dark red`, `Dark purple`, `Gold`, `Gray`, `Dark gray`, `Blue`, `Green`, `Aqua`, `Red`, `Light purple`, `Yellow`, `White`, `None`
- _Flags:_ rank: Noble

### `SetCarryingChest`
**Set Carrying Chest**
Sets whether a mob carries a chest, which allows its inventory to be accessed.
- _Works with:_ Donkey; Mule; Llama
- **Tags:**
   - `Carrying Chest` (slot 26): `Enable` (default), `Disable`

### `SetParrotColor`
**Set Parrot Color**
Sets a parrot's color.
- _Works with:_ Parrot
- **Tags:**
   - `Parrot Color` (slot 26): `Red` (default), `Blue`, `Green`, `Cyan`, `Gray`
- _Flags:_ rank: Noble

### `DispTranslation`
**Set Display Translation**
Sets the translation values of a display entity.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — X translation
   - `[1]` `NUMBER` — Y translation
   - `[2]` `NUMBER` — Z translation
   - **OR**
   - `[3]` `VECTOR` — Translation vector
- _Flags:_ rank: Mythic, advanced

### `Remove`
**Remove**
Deletes an entity.

### `TDispBackground`
**Set Text Display Background**
Sets the background color and opacity of a text display.
- _Works with:_ Text Display
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Color hexadecimal — _Example: "#FF0000" (red)_
   - `[1]` `NUMBER` — Opacity in percentage
   - **OR**
   - `[2]` `NONE` — Uses the default background color)
- _Flags:_ rank: Mythic

### `DisplayCullingSize`
**Set Display Culling Size**
Sets the culling width and height of a display entity.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Width — _Default = 0_
   - `[1]` `NUMBER` _(optional)_ — Height — _Default = 0_
- _Flags:_ rank: Mythic

### `HideName`
**Hide Name Tag**
> ⚠ **LEGACY** → use `" SetNameVisible "` (Set Name Tag Visibility tag to Never.)
Hides the name tag of an entity.

### `SetSheepSheared`
**Set Sheep Sheared**
> ⚠ **DEPRECATED:** This action is planned to be removed in a future update. Please use Entity Action: Shear (Appearance) instead.
Sets whether a sheep has its wool.
- _Works with:_ Sheep
- **Tags:**
   - `Sheared` (slot 26): `Enable` (default), `Disable`

### `SetAxolotlColor`
**Set Axolotl Pattern**
Sets an axolotl's color.
- _Works with:_ Axolotl
- **Tags:**
   - `Axolotl Color` (slot 26): `Pink` (default), `Brown`, `Yellow`, `Cyan`, `Blue`
- _Flags:_ rank: Noble

### `GetAllEntityTags`
**Get All Entity Custom Tags**
Gets all tags registered on an entity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VARIABLE` — Variable to set
- **Returns:** `DICT` Key represents the tag name as String, value represents tag value

### `AttackEntity`
**Attack Entity**
Causes an entity to attack another entity.
- _Works with:_ Any Mob; Arrows; Tridents
- _Info:_ When used with a projectile, it is as if it landed on the target.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Entity UUID
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `SetAI`
**Set AI**
Sets whether an entity is sentient and/or affected by physics.
- _Works with:_ Any Mob
- **Tags:**
   - `AI` (slot 26): `Sentient`, `Insentient`, `None` (default)
       - `Sentient`: The entity acts normally.
       - `Insentient`: The entity does not act by itself, but is affected by physics.
       - `None`: The entity does not act by itself, and is unaffected by physics.

### `SetWaxed`
**Set Copper Golem Waxed**
Sets whether a copper golem is waxed.
- _Works with:_ Copper Golem
- **Tags:**
   - `Waxed` (slot 26): `True` (default), `False`

### `KBAttribute`
**Set Knockback Attribute**
Sets one of the entity's knockback-related attributes such as knockback resistance.
- _Works with:_ Any Mob
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
Sets one of the entity's movement-related attributes, such as walking speed and jump height.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Value
- **Tags:**
   - `Attribute` (slot 25): `Walking speed` (default), `Flying speed`, `Jump strength`, `Step height`, `Movement efficiency`, `Water movement efficiency`, `Air drag modifier`, `Bounciness`, `Friction modifier`
       - `Walking speed`: Base: 0.2 Range: 0.0-1024.0
       - `Flying speed`: Base: 0.4 Range: 0.0-1024.0
       - `Jump strength`: Base: 0.42 Range: 0.0-32.0
       - `Step height`: Base: 0.6 Range: 0.0-10.0
       - `Movement efficiency`: Base: 0.0 Range: 0.0-1.0
       - `Water movement efficiency`: Base: 0.0 Range: 0.0-1.0
       - `Air drag modifier`: Base: 1.0 Range: 0.0-2048.0
       - `Bounciness`: Base: 0.0 Range: 0.0-1.0
       - `Friction modifier`: Base: 1.0 Range: 0.0-2048.0
   - `Value Type` (slot 26): `Direct` (default), `Percentage (Base)`, `Percentage (Relative)`
       - `Percentage (Base)`: A percentage of the base value shown above. 100% is equal to the base value.
       - `Percentage (Relative)`: A percentage of what the attribute is currently set to. For example, 200% doubles the current value.

### `SetRiptiding`
**Set Riptiding**
Sets whether an entity is riptiding.
- _Works with:_ Any Mob
- **Tags:**
   - `Riptiding` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Mythic

### `SetArrowNoClip`
**Set Arrow NoClip**
Sets whether an arrow will pass through blocks and through entities.
- _Works with:_ Arrows; Tridents
- **Tags:**
   - `Has NoClip` (slot 26): `Enable` (default), `Disable`

### `FallingAttribute`
**Set Falling Attribute**
Sets one of the entity's falling-related attributes, such as gravity and fall damage multiplier.
- _Works with:_ Any Mob
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

### `SetProjSource`
**Set Projectile Shooter**
Sets the projectile source of a projectile (or removes it).
- _Works with:_ Any Projectile; Area Effect Cloud
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Shooter UUID
   - **OR**
   - `[1]` `NONE` — Removes shooter
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `SetFoxLeaping`
**Set Fox Leaping**
Sets whether a fox appears to be leaping.
- _Works with:_ Fox
- _Info:_ Does not affect movement.
- **Tags:**
   - `Leaping` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `SetPandaGene`
**Set Panda Gene**
Sets the gene of a panda. This affects their behavior and appearance.
- _Works with:_ Panda
- **Tags:**
   - `Set Gene` (slot 25): `Main gene`, `Hidden gene`, `Both` (default)
   - `Gene Type` (slot 26): `Aggressive` (default), `Lazy`, `Weak`, `Worried`, `Playful`, `Normal`, `Brown`

### `SetOxidization`
**Set Copper Golem Oxidization**
Sets the oxidization level of a copper golem.
- _Works with:_ Copper Golem
- **Tags:**
   - `Oxidization` (slot 26): `Unaffected` (default), `Exposed`, `Weathered`, `Oxidized`

### `SetMaxHealth`
**Set Maximum Health**
Sets an entity's maximum health.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Maximum health
- **Tags:**
   - `Heal Mob to Max Health` (slot 26): `True`, `False` (default)

### `SetFishingTime`
**Set Fishing Wait Time**
Sets the time until a fish starts to approach a fishing hook.
- _Works with:_ Fishing Hook
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Wait time (ticks)

### `ClrVillagerTrades`
**Clear Villager Trades**
Removes all trades from a villager.
- _Works with:_ Villager; Wandering Trader

### `EndCrystalBeam`
**Set End Crystal Beam**
Sets the location an end crystal points its beam at.
- _Info:_ To remove the beam, do not specify a location.
- _Info:_ The target location is rounded to the nearest block.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` _(optional)_ — Target

### `FrogEat`
**Eat Target**
Makes a frog try to eat the specified mob or player.
- _Works with:_ Frog
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Target UUID
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `SetSalmonType`
**Set Salmon Type**
Sets a salmon's variant.
- _Works with:_ Salmon
- **Tags:**
   - `Salmon Type` (slot 26): `Small`, `Medium` (default), `Large`
- _Flags:_ rank: Noble

### `DisplayBrightness`
**Set Display Brightness**
Sets the brightness of a display entity.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Block light level (0-15)
   - `[1]` `NUMBER` — Sky light level (0-15)
   - **OR**
   - `[2]` `NONE` — Uses the light level at entity's location)
- _Flags:_ rank: Mythic

### `IgniteMob`
**Ignite Mob** · aliases: `IgniteCreeper`
Ignites a mob, causing it to explode after a fuse period.
- _Works with:_ Creeper; Sulfur Cube

### `SetProfession`
**Set Villager Profession**
Sets a villager's profession.
- _Works with:_ Villager; Zombie Villager
- _Info:_ If the villager has no experience yet, it is given 1 experience to prevent unemployment.
- **Tags:**
   - `Retain Trades` (slot 25): `True`, `False` (default)
   - `Profession` (slot 26): `Unemployed`, `Armorer` (default), `Butcher`, `Cartographer`, `Cleric`, `Farmer`, `Fisherman`, `Fletcher`, `Leatherworker`, `Librarian`, `Mason`, `Nitwit`, `Shepherd`, `Toolsmith`, `Weaponsmith`

### `SetTradeUses`
**Set Villager Trade Uses**
Sets the amount of times a trade can be made before the villager has to restock.
- _Works with:_ Villager; Wandering Trader
- _Info:_ Remaining trade amount is capped at the maximum amount of trades.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Trade index
   - `[1]` `NUMBER` _(optional)_ — Remaining uses — _Default = Infinite_
   - `[2]` `NUMBER` _(optional)_ — Maximum uses — _Default = No change_

### `ClearPotions`
**Clear Potion Effects** · aliases: `ClearEffects`
Removes all active potion effects from an entity.
- _Works with:_ Any Mob

### `ArmorStandParts`
**Set Armor Stand Parts**
Sets whether an armor stand has arms and a base plate.
- _Works with:_ Armor Stand
- **Tags:**
   - `Arms` (slot 25): `Enable` (default), `Disable`, `Don't change`
   - `Base Plate` (slot 26): `Enable` (default), `Disable`, `Don't change`
- _Flags:_ rank: Mythic

### `" SetNameVisible "`
**Set Name Visible**
Sets whether an entity's custom name is always displayed above them.
- **Tags:**
   - `Name Tag Visibility` (slot 26): `Always` (default), `Default`, `Never`
       - `Always`: The custom name is always shown.
       - `Default`: The custom name is shown when you are looking at the entity.
       - `Never`: The custom name is always hidden.

### `SetTarget`
**Set Target**
Instructs a mob's AI to target a specific mob or player.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Target UUID
   - **OR**
   - `[1]` `NONE` — Stop targeting
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.
- _Flags:_ rank: Overlord

### `TDisplayShadow`
**Set Text Display Text Shadow**
Sets whether the text in a text display has shadow or not.
- _Works with:_ Text Display
- **Tags:**
   - `Text Shadow` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Mythic

### `SetNameVisible`
**Set Name Visible**
> ⚠ **LEGACY** → use `" SetNameVisible "`
Sets whether an entity's custom name is always displayed above them.
- _Info:_ A mob's custom name is always visible when the cursor is placed on them.
- **Tags:**
   - `Name Tag Visible` (slot 26): `Enable` (default), `Disable`

### `SetInvulTicks`
**Set Invulnerability Ticks**
Sets the currently remaining ticks until an entity can next be hurt.
- _Info:_ This value is set to 10 upon taking damage.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks

### `MannequinSkin`
**Set Mannequin Skin**
Sets a mannequin's skin avatar.
- _Works with:_ Mannequin
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Mannequin player head
   - **OR**
   - `[1]` `TEXT` — Mannequin player name or UUID
- _Flags:_ rank: Overlord

### `SetShulkerPeek`
**Set Shulker Peek Percent**
Sets how far a shulker should peek up to.
- _Works with:_ Shulker
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Peek Percentage
- **Tags:**
   - `Is Silent` (slot 26): `Enable` (default), `Disable`

### `" SetPose "`
**Set Pose**
Changes the pose of an entity. This affects their animations and/or hitbox, depending on the pose and entity type.
- **Tags:**
   - `Pose` (slot 26): `Standing` (default), `Sleeping`, `Swimming`, `Sneaking`
- _Flags:_ rank: Mythic

### `SetRearing`
**Set Rearing**
Sets whether a horse is standing on its hind legs.
- _Works with:_ Any Horse
- **Tags:**
   - `Rearing` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `SetCloudRadius`
**Set Potion Cloud Radius**
Sets an area of effect cloud's radius and shrinking speed.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Radius
   - `[1]` `NUMBER` _(optional)_ — Shrinking speed (blocks per second)

### `SetGravity`
**Set Gravity**
Sets whether an entity is affected by gravity.
- **Tags:**
   - `Gravity` (slot 26): `Enable`, `Disable` (default)

### `Mimic`
**Mimic Entity**
Disguises an entity as another currently existing entity or player.
- _Info:_ The disguise will match the target exactly, including things like color or equipment.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — UUID of target to disguise as
- **Tags:**
   - `Remove Original Entity` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Overlord

### `DispTPDuration`
**Set Display Teleport Duration**
Sets how long a display entity takes to visually move to its destination when it teleports.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Teleport duration in ticks — _Default = 0_
- _Flags:_ rank: Mythic

### `SetWitherInvul`
**Set Invulnerability Ticks**
Sets the remaining ticks of invulnerability a wither has.
- _Works with:_ Wither
- _Info:_ If AI is enabled, this value decreases over time. At 0, an explosion occurs.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Ticks

### `Silence`
**Silence**
> ⚠ **LEGACY** → use `SetSilenced` (Set Silenced tag to Enable.)
Prevents the entity from making any sounds.
- _Flags:_ rank: Noble

### `SetArrowPierce`
**Set Arrow Pierce**
Sets how many targets an arrow can pierce through. A pierce of 1 can hit up to 2 entities.
- _Works with:_ Arrows
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Targets to pierce

### `DisplayShadow`
**Set Display Shadow**
Sets the shadow properties of a display entity.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Shadow radius in blocks — _Default = 1_
   - `[1]` `NUMBER` _(optional)_ — Shadow opacity in percentage — _Default = 100_
- _Flags:_ rank: Mythic

### `InteractResponse`
**Set Interaction Responsive**
Sets whether an interaction entity has response when interacting with it.
- _Works with:_ Interaction Entity
- **Tags:**
   - `Responsive` (slot 26): `Enable` (default), `Disable`

### `UseItem`
**Use Item**
Forces a mob to use held items such as bow or spyglass.
- _Works with:_ Any Mob
- **Tags:**
   - `Hand` (slot 25): `Main Hand` (default), `Off Hand`
   - `Use Item` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `RideEntity`
**Ride Entity**
Mounts an entity on top of another entity or player.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Target UUID
   - **OR**
   - `[1]` `NONE` — Dismounts entity)
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.
- _Flags:_ rank: Noble

### `DisplayMatrix`
**Set Display Transformation Matrix**
Sets the affine transformation matrix of a display entity.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LIST` — 16 numbers describing a row-major matrix
- _Flags:_ rank: Mythic, advanced

### `NoDrops`
**Disable Death Drops**
> ⚠ **LEGACY** → use `SetDeathDrops` (Set Has Death Drops tag to Disable.)
After this code block is executed, the mob will no longer drop their equipment and loot when they die.
- _Works with:_ Any Mob

### `SetWolfSoundType`
**Set Wolf Sound Type**
Sets a wolf's sound variant.
- _Works with:_ Wolf
- **Tags:**
   - `Wolf Sound Type` (slot 26): `Angry` (default), `Big`, `Classic`, `Cute`, `Grumpy`, `Puglin`, `Sad`
- _Flags:_ rank: Noble

### `SnifferState`
**Set Sniffer State**
Forces a sniffer to perform a specific action.
- _Works with:_ Sniffer
- **Tags:**
   - `Behavior` (slot 26): `Idle` (default), `Feeling Happy`, `Scenting`, `Sniffing`, `Searching`, `Digging`

### `EnableGlowing`
**Enable Glowing**
> ⚠ **LEGACY** → use `SetGlowing` (Set Glowing tag to Enable.)
Makes an entity glow.

### `Teleport`
**Teleport**
Teleports an entity to a specified location.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — New position
- **Tags:**
   - `Keep Current Rotation` (slot 26): `True`, `False` (default)

### `DisplayGlowColor`
**Set Display Glow Color**
Sets the glowing color of a display entity.
- _Works with:_ Item Display; Block Display
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Color hexadecimal — _Example: "#FF0000" (red)_
   - **OR**
   - `[1]` `NONE` — Resets glow color)
- _Flags:_ rank: Mythic

### `SetVisualFire`
**Set Visual Fire**
Sets whether an entity should appear on fire.
- _Info:_ The affected entity's fire ticks won't change and won't take any damage.
- **Tags:**
   - `On Fire` (slot 26): `True` (default), `False`
- _Flags:_ rank: Noble

### `DisguiseShiftVert`
**Shift Disguise Vertically**
Shifts the disguise of an entity up or down relative to the entity itself.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Y-Offset
- _Flags:_ rank: Overlord

### `SetAge/Size`
**Set Age / Size**
> ⚠ **LEGACY** → use `SetAge` (For slimes/phantoms use Set Size. For zombies use Set Baby.)
Sets the age or size of a mob.
- _Works with:_ Any Mob
- _Info:_ Ages below 0 make the target a baby. For animals, age increases constantly
- _Info:_ For slimes, magma cubes and phantoms, size is set instead.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Mob age or size

### `AttachFirework`
**Attach Firework**
Attaches a firework rocket to another mob or player.
- _Works with:_ Fireworks
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` _(plural)_ — Target UUID
   - **OR**
   - `[1]` `NONE` — Removes the attachment
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `L SetArmor`
**Set Equipment Items**
> ⚠ **LEGACY** → use `SetArmor`
Sets the armor and held items of a mob.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` _(optional, plural)_ — Equipment items list (6) — _Order: main hand, helmet, chestplate, leggings, boots, off hand_
   - **OR**
   - `[1]` `ITEM` _(optional)_ — Main hand
   - `[2]` `ITEM` _(optional)_ — Helmet
   - `[3]` `ITEM` _(optional)_ — Chestplate
   - `[4]` `ITEM` _(optional)_ — Leggings
   - `[5]` `ITEM` _(optional)_ — Boots
   - `[6]` `ITEM` _(optional)_ — Off hand

### `SetSaddle`
**Set Wearing Saddle**
Sets whether a mob wears a saddle.
- _Works with:_ Any Horse; Pig; Llama; Happy Ghast; Nautilus; Camel; Ravager; Strider
- _Info:_ For a custom saddle item, use 'Set Equipment Item'.
- **Tags:**
   - `Saddle` (slot 26): `Enable` (default), `Disable`

### `SetBulletTarget`
**Set Shulker Bullet Target**
Causes a shulker bullet to start targeting the provided entity.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Target UUID
   - **OR**
   - `[1]` `NONE` — No Target
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `TDisplayLineWidth`
**Set Text Display Line Width**
Sets the maximum line width of a text display.
- _Works with:_ Text Display
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Line width — _Default = 200_
- _Flags:_ rank: Mythic

### `RemVillagerTrade`
**Remove Villager Trade**
Removes a trade from a villager
- _Works with:_ Villager; Wandering Trader
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Trade index

### `SetDragonPhase`
**Set Dragon Phase**
Sets the behavior phase of an Ender Dragon.
- _Works with:_ Ender Dragon
- **Tags:**
   - `Phase` (slot 26): `Flying` (default), `Hovering`, `Breath attack`, `Dying`
       - `Flying`: The dragon will fly towards the world origin.
       - `Hovering`: The dragon will not move.
       - `Breath attack`: The dragon will perform a breath attack while hovering.
       - `Dying`: The dragon will disappear after a 10 second death animation.

### `SetLlamaColor`
**Set Llama Color**
Sets a llama's fur color.
- _Works with:_ Llama
- **Tags:**
   - `Llama Color` (slot 26): `Brown` (default), `Creamy`, `White`, `Gray`
- _Flags:_ rank: Noble

### `SetVillagerBiome`
**Set Villager Biome**
Sets the biome type of a villager. This affects their appearance only.
- _Works with:_ Villager; Zombie villager
- **Tags:**
   - `Biome` (slot 26): `Desert` (default), `Jungle`, `Plains`, `Savanna`, `Snow`, `Swamp`, `Taiga`
- _Flags:_ rank: Noble

### `SetCreeperFuse`
**Set Creeper Fuse**
Sets the starting amount of ticks it takes for a creeper to explode.
- _Works with:_ Creeper
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Fuse ticks

### `SetBaby`
**Set to Baby/Adult**
Sets whether an entity is a baby (permanently).
- _Works with:_ Any Breedable; Any Zombie; Piglin; Armor Stand
- **Tags:**
   - `Baby` (slot 26): `Enable` (default), `Disable`

### `MooshroomType`
**Set Mooshroom Type**
Sets a mooshroom's skin type.
- _Works with:_ Mooshroom
- **Tags:**
   - `Mooshroom Variant` (slot 26): `Red` (default), `Brown`
- _Flags:_ rank: Noble

### `SetInvisible`
**Set Invisible**
Sets whether an entity is invisible.
- _Works with:_ Any Mob; Item Frame; Armor Stand; Interaction Entity
- **Tags:**
   - `Invisible` (slot 26): `Enable` (default), `Disable`

### `SheepEat`
**Eat Grass**
Causes a sheep to eat grass.
- _Works with:_ Sheep

### `SetCatResting`
**Set Cat Resting**
Sets whether a cat appears to be lying down.
- _Works with:_ Cat
- _Info:_ Does not affect movement.
- **Tags:**
   - `Resting` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `GivePotion`
**Give Potion Effect** · aliases: `GiveEffect`
Gives one or more potion effects to an entity.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `POTION` _(plural)_ — Effect(s) to give
- **Tags:**
   - `Overwrite Effect` (slot 25): `True` (default), `False`
       - `True`: Effects of the same type will always be replaced.
       - `False`: Weaker effects of the same type will be hidden.
   - `Effect Particles` (slot 26): `Regular` (default), `Ambient`, `None`
       - `Ambient`: Ambient particles are translucent.

### `SetGoatHorns`
**Set Goat Horns**
Sets which goat horns are shown or hidden.
- _Works with:_ Goat
- **Tags:**
   - `Left Horn` (slot 25): `Show`, `Hide`, `No Change` (default)
   - `Right Horn` (slot 26): `Show`, `Hide`, `No Change` (default)

### `SetGlowing`
**Set Glowing**
Sets whether this entity has a glowing outline that can be seen through blocks.
- **Tags:**
   - `Glowing` (slot 26): `Enable` (default), `Disable`

### `SetPandaOnBack`
**Set Panda On Back**
Sets whether a panda is laying on its back or not.
- _Works with:_ Panda
- **Tags:**
   - `On Its Back` (slot 26): `Enable` (default), `Disable`

### `IDisplayModelType`
**Set Item Display Model Type**
Sets the model type of an item display.
- _Works with:_ Item Display
- **Tags:**
   - `Model Type` (slot 26): `None` (default), `First Person Left Hand`, `First Person Right Hand`, `Third Person Left Hand`, `Third Person Right Hand`, `Head`, `GUI`, `Ground`, `Fixed`
- _Flags:_ rank: Mythic

### `SetHealth`
**Set Current Health**
Sets an entity's current health.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Current health — _❤ = 2 Health_

### `BlockDisguise`
**Disguise as Block**
Disguises an entity as a block.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `BLOCK` — Block to disguise as
   - `[1]` `COMPONENT` _(optional)_ — Display name
- _Flags:_ rank: Overlord

### `AddVillagerTrade`
**Add Villager Trade**
Adds a trade to a villager.
- _Works with:_ Villager; Wandering Trader
- _Info:_ Villagers need a profession to sell things.
- _Info:_ Trades only check for the material and amount of the ingredients.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Result item
   - `[1]` `ITEM` — First ingredient
   - `[2]` `ITEM` _(optional)_ — Second ingredient
   - `[3]` `NUMBER` _(optional)_ — Maximum uses — _Negative numbers are treated as infinite Default = Infinite_

### `SetCollidable`
**Set Collidable**
Sets whether a mob is able to collide with other entities.
- _Works with:_ Any Mob
- _Info:_ To disable collisions between mobs and players, both must have their collisions disabled.
- **Tags:**
   - `Collision` (slot 26): `Enable`, `Disable` (default)

### `ArmorStandPose`
**Set Armor Stand Pose** · aliases: `SetPose`
Sets the rotation of an armor stand part.
- _Works with:_ Armor Stand
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `VECTOR` — Direction
   - **OR**
   - `[1]` `NUMBER` _(optional)_ — X Rotation (0-360) — _Default = 0_
   - `[2]` `NUMBER` _(optional)_ — Y Rotation (0-360) — _Default = 0_
   - `[3]` `NUMBER` _(optional)_ — Z Rotation (0-360) — _Default = 0_
- **Tags:**
   - `Armor Stand Part` (slot 26): `Head` (default), `Body`, `Left Arm`, `Right Arm`, `Left Leg`, `Right Leg`
- _Flags:_ rank: Mythic

### `LaunchFwd`
**Launch Forward**
Launches an entity forward or backward.
- _Info:_ A negative launch power will launch the entity backward.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Launch power
- **Tags:**
   - `Add to Current Velocity` (slot 25): `True` (default), `False`
   - `Launch Axis` (slot 26): `Pitch and Yaw` (default), `Yaw Only`

### `SetFallDistance`
**Set Fall Distance**
Sets an entity's fall distance, affecting fall damage upon landing.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Fall distance (blocks)

### `SetArrowCritical`
**Set Arrow Critical**
Sets whether an arrow is critical. A critical arrow deals extra damage.
- _Works with:_ Arrows; Tridents
- **Tags:**
   - `Critical` (slot 26): `Enable` (default), `Disable`

### `MoveToLoc`
**Move to Location**
Instructs a mob's AI to always pathfind to a certain location at a certain speed.
- _Works with:_ Any Mob
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Target location
   - **OR**
   - `[1]` `NONE` — Stop moving
   - `[2]` `NUMBER` — Walk speed
- _Flags:_ rank: Overlord

### `TDisplayOpacity`
**Set Text Display Text Opacity**
Sets the text opacity of a text display.
- _Works with:_ Text Display
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — Text opacity — _Default = 100_
- _Flags:_ rank: Mythic

### `IDisplayItem`
**Set Item Display Item**
Sets the displayed item of an item display.
- _Works with:_ Item Display
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — Displayed item
- _Flags:_ rank: Mythic

### `AttackAnimation`
**Send Mob Attack Animation**
Makes a mob perform an attack animation.
- _Works with:_ Any Mob
- **Tags:**
   - `Animation Arm` (slot 26): `Swing main arm` (default), `Swing off arm`
- _Flags:_ rank: Noble

### `SnowmanPumpkin`
**Set Snow Golem Pumpkin**
Sets whether a snow golem is wearing a pumpkin.
- _Works with:_ Snow Golem
- **Tags:**
   - `Pumpkin` (slot 26): `Enable`, `Disable` (default)
- _Flags:_ rank: Noble

### `SetCustomTag`
**Set Entity Custom Tag**
Sets the value of or creates a custom tag value.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Tag name
   - **OR**
   - `[1]` `NUMBER` — Tag value
   - **OR**
   - `[2]` `TEXT` — Tag value

### `Gravity`
**Enable Gravity**
> ⚠ **LEGACY** → use `SetGravity` (Set Gravity tag to Enable.)
Enables gravity for the entity.
- _Flags:_ rank: Overlord

### `DisplayViewRange`
**Set Display View Range**
Sets the view range of a display entity.
- _Works with:_ Any Display Entity
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` _(optional)_ — View range in blocks — _Default = 64_
- _Flags:_ rank: Mythic

### `NoAI`
**Disable AI**
> ⚠ **LEGACY** → use `SetAI` (Set AI tag to None.)
Disables the AI of the mob.
- _Works with:_ Any Mob
- _Flags:_ rank: Noble

### `PlayerDisguise`
**Disguise as Player**
Disguises an entity as a player.
- _Info:_ To get a skin use /item head <player name> OR /item mshead
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` — Player name to disguise as
   - `[1]` `ITEM` _(optional)_ — Display skin
- _Flags:_ rank: Overlord

### `SetItem`
**Set Entity Item**
Sets the item of an item entity.
- _Works with:_ Dropped Item
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `ITEM` — New item

### `SetVillagerTrade`
**Set Villager Trade**
Sets the villager trade at an index.
- _Works with:_ Villager; Wandering Trader
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Trade index
   - `[1]` `ITEM` — Result item
   - `[2]` `ITEM` — First Ingredient
   - `[3]` `ITEM` _(optional)_ — Second Ingredient
   - `[4]` `NUMBER` _(optional)_ — Maximum uses — _Default = Infinite_

### `Explode`
**Explode** · aliases: `ExplodeCreeper`
Causes an entity to explode.
- _Works with:_ Creeper; Primed TNT; Firework; Thrown Potion

### `SetDigging`
**Set Warden Digging**
Makes a warden emerge or dig into the ground.
- _Works with:_ Warden
- **Tags:**
   - `Digging Type` (slot 26): `Emerge` (default), `Dig Down`
       - `Dig Down`: The warden will not despawn after digging down.

### `MoveTo`
**Move to Location**
> ⚠ **LEGACY** → use `MoveToLoc`
Instructs a mob's AI to always pathfind to a certain location at a certain speed.
- _Works with:_ Any Mob
- _Info:_ 'Move To' only works if the target is close enough to the target so that its AI can pathfind to it.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `LOCATION` — Target location
   - `[1]` `NUMBER` _(optional)_ — Walk speed
- _Flags:_ rank: Overlord

### `SetArrowHitSound`
**Set Arrow Hit Sound**
Sets the sound an arrow plays whenever it lands.
- _Works with:_ Arrows, Tridents
- _Info:_ The pitch and volume of the sound are ignored.
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `SOUND` — Sound to play

### `SetVexCharging`
**Set Vex Charging**
Sets whether a vex is charging or not.
- _Works with:_ Vex
- **Tags:**
   - `Charging` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `SetVillagerExp`
**Set Villager Experience**
Sets a villager's experience points, which affects their level.
- _Works with:_ Villager
- _Info:_ Level requirements: ■ 0: Novice ■ 10: Apprentice ■ 70: Journeyman ■ 150: Expert ■ 250: Master
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Experience

### `SetCelebrating`
**Set Celebrating**
Causes a mob to start or stop celebrating.
- _Works with:_ Any Raider; Piglin
- **Tags:**
   - `Celebrate` (slot 26): `Enable` (default), `Disable`
- _Flags:_ rank: Noble

### `TDisplayText`
**Set Text Display Text**
Sets the displayed text of a text display.
- _Works with:_ Text Display
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `COMPONENT` _(plural)_ — Displayed text
- **Tags:**
   - `Text Value Merging` (slot 26): `Add spaces`, `No spaces` (default)
   - `Inherit Styles` (slot 25): `True` (default), `False`
       - `True`: Text style of the last value will be inherited when merging multiple values.
       - `False`: Each value will have independent text style when merging multiple values.
- _Flags:_ rank: Mythic

### `SetHorseJump`
**Set Horse Jump Strength**
Sets a horse's jump strength.
- _Works with:_ Horse; Donkey; Mule; Llama
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `NUMBER` — Strength — _0.0 (can't jump) to 2.0_

### `Unsilence`
**Unsilence**
> ⚠ **LEGACY** → use `SetSilenced` (Set Silenced tag to Disable.)
Allows the entity to make sounds.
- _Flags:_ rank: Noble

### `HealthAttribute`
**Set Health Attribute**
Sets one of the entity's health-related attributes such as max health and armor defense points.
- _Works with:_ Any Mob
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

### `Ram`
**Ram Target**
Makes a goat ram the specified mob or player.
- _Works with:_ Goat
- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):
   - `[0]` `TEXT` — Target UUID
- **Tags:**
   - `Ignore Formatting` (slot 26): `True` (default), `False`
       - `True`: Color codes and style tags are ignored when matching an entity by its name.
       - `False`: All color codes and style tags in the entity's name must match exactly.

### `SetFoxType`
**Set Fox Type**
Sets a fox's fur type.
- _Works with:_ Fox
- **Tags:**
   - `Fox Type` (slot 26): `Red` (default), `Snow`
- _Flags:_ rank: Noble
