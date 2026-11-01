# DiamondFire actions — Events (`event`, `entity_event`, `game_event`)

_Generated from actiondump (patch 8.13). Action names are exact wire strings; names in double quotes include **leading/trailing spaces** that must be kept._

## Block `event` — Player Event (81 actions in this file)

Used to execute code when something is done by (or happens to) a player.

### `ClickContainerSlot`
**Player Click Container Slot Event**
Executes code when a player clicks a slot in a container.
- Cancellable: True; auto-cancelled: False

### `CloseInv`
**Player Close Inventory Event**
Executes code when a player closes an inventory.
- Cancellable: False; auto-cancelled: False
- _Flags:_ rank: Emperor

### `StartFly`
**Player Start Flight Event**
Executes code when a player starts flying.
- Cancellable: True; auto-cancelled: False

### `AirChange`
**Player Air Change Event**
Executes code when a player's air level changes.
- Cancellable: True; auto-cancelled: False

### `BreakBlock`
**Player Break Block Event**
Executes code when a player breaks a block.
- Cancellable: True; auto-cancelled: False

### `StartSprint`
**Player Start Sprint Event**
Executes code when a player starts sprinting.
- _Info:_ Cancelling this event will only result in the sprint particles being hidden.
- _Info:_ Client side particles are not hidden!
- Cancellable: True; auto-cancelled: False

### `MobKillPlayer`
**Mob Kill Player Event**
Executes code when a mob kills a player.
- Cancellable: True; auto-cancelled: False

### `Teleport`
**Player Teleport Event**
Executes code when a player is teleported.
- Cancellable: True; auto-cancelled: False

### `VillagerTrade`
**Player Trade Event**
Executes code when a player trades with a villager.
- _Info:_ When a trade is cancelled, the trades left still goes down for the player. This is fixed when the trade menu is reopened.
- Cancellable: True; auto-cancelled: False

### `ShootBow`
**Player Shoot Bow Event**
Executes code when a player fires an arrow with a bow.
- Cancellable: True; auto-cancelled: False

### `ChangeSign`
**Player Change Sign Event**
Executes code when a player changes a sign.
- Cancellable: True; auto-cancelled: False

### `StopFly`
**Player Stop Flight Event**
Executes code when a player stops flying.
- Cancellable: True; auto-cancelled: False

### `StartSpecPlayer`
**Player Start Spectating Player Event**
Executes code when a player starts spectating a player.
- Cancellable: False; auto-cancelled: True

### `TameEntity`
**Player Tame Mob Event**
Executes code when a player tames a mob.
- Cancellable: True; auto-cancelled: False

### `LeftClick`
**Player Left Click Event**
Executes code when a player left clicks.
- _Info:_ Does not execute on left clicks where a player deals damage.
- Cancellable: True; auto-cancelled: False

### `PlayerTakeDmg`
**Player Take Damage Event**
Executes code when a player takes damage.
- Cancellable: True; auto-cancelled: False

### `ProjHit`
**Player Projectile Hit Event**
Executes code when a projectile launched by a player hits a block, an entity, or another player.
- Cancellable: False; auto-cancelled: False

### `StopSpecPlayer`
**Player Stop Spectating Player Event**
Executes code when a player stops spectating a player.
- Cancellable: False; auto-cancelled: True

### `StopUsingItem`
**Player Stop Using Item Event**
Executes code when a player stops using an item.
- _Examples:_ ⏵ Player cancels using a trident ⏵ Player stops looking through a spyglass ⏵ Player stops eating a food item
- Cancellable: False; auto-cancelled: False

### `KillPlayer`
**Player Kill Player Event**
Executes code when a player kills another player.
- Cancellable: True; auto-cancelled: False

### `VehicleJump`
**Player Vehicle Jump Event**
Executes code when a player presses the jump key while riding a vehicle or other entity.
- Cancellable: False; auto-cancelled: False

### `ClickInvSlot`
**Player Click Inventory Slot Event** · aliases: `ClickOwnInv`
Executes code when a player clicks a slot inside their inventory.
- Cancellable: True; auto-cancelled: False

### `Respawn`
**Player Respawn Event**
Executes code when a player respawns.
- Cancellable: False; auto-cancelled: False

### `StartBreaking`
**Player Start Breaking Block Event**
Executes code when a player begins breaking a block.
- Cancellable: False; auto-cancelled: False

### `SwapHands`
**Player Swap Hands Event**
Executes code when a player swaps an item or items between their main hand and off hand.
- Cancellable: True; auto-cancelled: False

### `PackLoad`
**Resource Pack Load**
Executes code when a player finishes loading a plot resource pack.
- Cancellable: False; auto-cancelled: False
- _Flags:_ rank: Overlord

### `DamageEntity`
**Player Damage Entity Event**
Executes code when a player damages an entity.
- Cancellable: True; auto-cancelled: False

### `Sneak`
**Player Sneak Event**
Executes code when a player sneaks.
- _Info:_ Cancelling this event will not update what the sneaking player sees!
- Cancellable: True; auto-cancelled: False

### `PlayerHeal`
**Player Heal Event**
Executes code when a player regains health from any source.
- Cancellable: True; auto-cancelled: False

### `ClickPlayer`
**Player Right Click Player Event**
Executes code when a player right clicks another player.
- Cancellable: False; auto-cancelled: False

### `Consume`
**Player Consume Item Event**
Executes code when a player eats or drinks an item.
- Cancellable: True; auto-cancelled: False

### `StartGlide`
**Player Start Glide Event**
Executes code when a player starts gliding.
- Cancellable: True; auto-cancelled: False

### `MovementKey`
**Player Movement Key Change Event**
Executes code when a player presses or releases movement keys.
- Cancellable: False; auto-cancelled: False

### `Death`
**Player Death Event**
Executes code when a player dies, not as a result of another player or entity.
- Cancellable: True; auto-cancelled: False

### `PlaceBlock`
**Player Place Block Event**
Executes code when a player places a block.
- Cancellable: True; auto-cancelled: False

### `StopBreaking`
**Player Stop Breaking Block Event**
Executes code when a player stops breaking a block.
- Cancellable: False; auto-cancelled: False

### `StopSpecEntity`
**Player Stop Spectating Entity Event**
Executes code when a player stops spectating an entity.
- Cancellable: False; auto-cancelled: True

### `Walk`
**Player Walk Event**
Executes code while a player is walking.
- Cancellable: True; auto-cancelled: False

### `PickUpItem`
**Player Pick Up Item Event** · aliases: `PickupItem`
Executes code when a player picks up an item.
- Cancellable: True; auto-cancelled: False

### `Dismount`
**Player Dismount Event**
Executes code when a player dismounts a vehicle or other entity.
- Cancellable: True; auto-cancelled: False

### `CloudImbuePlayer`
**Potion Cloud Imbue Player Event**
Executes code when an area of effect cloud applies its potion effect(s) to a player.
- _Info:_ Area of effect clouds that do not apply any potion effects will not trigger this event.
- Cancellable: True; auto-cancelled: False

### `StartSpecEntity`
**Player Start Spectating Entity Event**
Executes code when a player starts spectating an entity.
- Cancellable: False; auto-cancelled: True

### `Leave`
**Player Leave Game Event** · aliases: `Quit`
Executes code when a player leaves the plot.
- Cancellable: False; auto-cancelled: False

### `DropItem`
**Player Drop Item Event**
Executes code when a player drops an item.
- Cancellable: True; auto-cancelled: False

### `LeftClickPlayer`
**Player Left Click Player Event**
Executes code when a player left clicks another player.
- Cancellable: True; auto-cancelled: False

### `ChangeSlot`
**Player Change Slot Event**
Executes code when a player changes their hotbar slot.
- Cancellable: True; auto-cancelled: False

### `ClickEntity`
**Player Right Click Entity Event**
Executes code when a player right clicks an entity.
- Cancellable: True; auto-cancelled: False

### `PickEntity`
**Player Pick Entity Event**
Executes code when a player uses their Pick Block key bind on an entity.
- Cancellable: True; auto-cancelled: False

### `HorseJump`
**Player Horse Jump Event**
Executes code when a player causes a horse to jump.
- Cancellable: False; auto-cancelled: False

### `ShootProjectile`
**Player Shoot Projectile Event**
Executes code when a player throws a projectile such as snowballs or eggs.
- Cancellable: True; auto-cancelled: False

### `Move`
**Player Move Event**
Executes code when a player moves.
- Cancellable: True; auto-cancelled: False

### `Unsneak`
**Player Unsneak Event**
Executes code when a player stops sneaking.
- _Info:_ Cancelling this event will not update what the sneaking player sees!
- Cancellable: True; auto-cancelled: False

### `TakeKnockback`
**Player Take Knockback Event**
Executes code when a player takes knockback.
- Cancellable: True; auto-cancelled: False

### `PlayerCombust`
**Player Combust Event**
Executes code when a player catches on fire.
- Cancellable: True; auto-cancelled: False

### `Chat`
**Player Chat Event**
Executes code when a player sends a chat message.
- Cancellable: True; auto-cancelled: False
- _Flags:_ rank: Mythic

### `Fish`
**Player Fish Event** · aliases: `FishEvent`
Executes code when a player fishes an entity, player, or nothing.
- Cancellable: True; auto-cancelled: False

### `FallDamage`
- Cancellable: False; auto-cancelled: False

### `BreakItem`
**Player Break Item Event**
Executes code when a player breaks an item.
- Cancellable: False; auto-cancelled: False

### `EntityKnockback`
**Entity Knockback Player Event**
Executes code when an entity applies knockback to a player.
- Cancellable: True; auto-cancelled: False

### `LoopEvent`
> ⚠ **LEGACY** → use `` (Loops have been removed, start a process when a player joins and use a Repeat: Forever to loop, set the delay with Control: Wait)
- Cancellable: False; auto-cancelled: False

### `PlayerResurrect`
**Player Resurrect Event**
Executes code when a player resurrects with a totem of undying.
- Cancellable: True; auto-cancelled: False

### `RightClick`
**Player Right Click Event**
Executes code when a player right clicks while looking at a block or holding an item.
- _Info:_ Executes multiple times if a player holds down right click.
- Cancellable: True; auto-cancelled: False

### `Purchase`
**Purchase Product**
Executes code when a player purchases a plot product.

### `ClickMenuSlot`
**Player Click Menu Slot Event** · aliases: `ClickItem`
Executes code when a player clicks a slot in an inventory menu.
- Cancellable: False; auto-cancelled: True
- _Flags:_ rank: Emperor

### `Riptide`
**Player Riptide Event**
Executes code when a player throws a riptide trident.
- Cancellable: False; auto-cancelled: False

### `StopGlide`
**Player Stop Glide Event**
Executes code when a player stops gliding.
- Cancellable: True; auto-cancelled: False

### `ClickDialog`
**Player Click Dialog Button**
Executes code when a player clicks a custom ID dialog button.
- Cancellable: False; auto-cancelled: False

### `KillMob`
**Player Kill Mob Event**
Executes code when a player kills a mob.
- Cancellable: True; auto-cancelled: False

### `Join`
**Player Join Game Event**
Executes code when a player joins the plot.
- Cancellable: False; auto-cancelled: False

### `EntityDmgPlayer`
**Entity Damage Player Event**
Executes code when an entity damages a player.
- Cancellable: True; auto-cancelled: False

### `StopSprint`
**Player Stop Sprinting Event**
Executes code when a player stops sprinting.
- _Info:_ Cancelling this event will only result in the sprint particles being shown.
- _Info:_ Client side particles are not shown!
- Cancellable: True; auto-cancelled: False

### `Jump`
**Player Jump Event**
Executes code when a player jumps.
- Cancellable: True; auto-cancelled: False

### `LoadCrossbow`
**Player Load Crossbow Event**
Executes code when a player loads a crossbow.
- Cancellable: True; auto-cancelled: False

### `ProjDmgPlayer`
**Projectile Damage Player Event**
Executes code when a projectile damages a player.
- Cancellable: True; auto-cancelled: False

### `Command`
**Player Command Event**
Executes code when a player types a command on the plot.
- _Examples:_ "@command"
- Cancellable: False; auto-cancelled: False
- _Flags:_ rank: Emperor

### `LeftClickEntity`
**Player Left Click Entity Event**
Executes code when a player left clicks an entity.
- Cancellable: True; auto-cancelled: False

### `Exhaustion`
**Player Exhaustion Event**
Executes code when a player gains exhaustion.
- Cancellable: True; auto-cancelled: False

### `PackDecline`
**Resource Pack Decline**
Executes code when a player declines a plot resource pack prompt.
- Cancellable: False; auto-cancelled: False
- _Flags:_ rank: Overlord

### `EditBook`
**Player Edit Book Event**
Executes code when a player edits or signs a book.
- Cancellable: True; auto-cancelled: False

### `PickBlock`
**Player Pick Block Event**
Executes code when a player uses their Pick Block key bind on a block.
- Cancellable: True; auto-cancelled: False

### `PlayerDmgPlayer`
**Player Damage Player Event**
Executes code when a player damages another player.
- Cancellable: True; auto-cancelled: False

## Block `entity_event` — Entity Event (22 actions in this file)

Used to execute code when an entity does something or when something happens to an entity.

### `EntityKillEntity`
**Entity Kill Entity Event**
Executes code when an entity kills another entity.
- Cancellable: True; auto-cancelled: False

### `EntityCombust`
**Entity Combust Event**
Executes code when an entity catches on fire.
- Cancellable: True; auto-cancelled: False

### `BlockFall`
**Block Fall Event**
Executes code when a block affected by gravity turns into a falling block.
- Cancellable: False; auto-cancelled: True

### `NaturallySpawn`
**Entity Spawn Naturally**
Executes code when an entity spawns into the world naturally.
- _Info:_ Use Game Action: Set Mob Spawning to enable natural spawning.
- Cancellable: False; auto-cancelled: True
- _Flags:_ world-plot only

### `ProjKillEntity`
**Projectile Kill Entity Event**
Executes code when a projectile kills an entity.
- Cancellable: True; auto-cancelled: False

### `AirChange`
**Entity Air Change Event**
Executes code when an entity's air level changes.
- Cancellable: True; auto-cancelled: False

### `EntityDmgEntity`
**Entity Damage Entity Event**
Executes code when an entity damages another entity.
- Cancellable: True; auto-cancelled: False

### `FallingBlockLand`
**Falling Block Land Event**
Executes code when a falling block lands on the ground.
- Cancellable: True; auto-cancelled: False

### `EntityResurrect`
**Entity Resurrect Event**
Executes code when an entity resurrects with a totem of undying.
- Cancellable: True; auto-cancelled: False

### `ItemMerge`
**Item Merge Event**
Executes code when dropped items try to merge into a stack.
- Cancellable: True; auto-cancelled: False

### `EntityHeal`
**Entity Heal Event**
Executes code when an entity regains health.
- Cancellable: True; auto-cancelled: False

### `Teleport`
**Entity Teleport Event**
Executes code when an entity is teleported.
- Cancellable: True; auto-cancelled: False

### `ShootBow`
**Entity Shoot Bow Event**
Executes code when an entity shoots a bow.
- Cancellable: True; auto-cancelled: False

### `TakeKnockback`
**Entity Take Knockback Event**
Executes code when an entity takes knockback.
- Cancellable: True; auto-cancelled: False

### `KnockbackEntity`
**Entity Knockback Entity Event**
Executes code when an entity applies knockback to an entity.
- Cancellable: True; auto-cancelled: False

### `EntityDmg`
**Entity Take Damage Event**
Executes code when an entity takes damage.
- Cancellable: True; auto-cancelled: False

### `ProjDmgEntity`
**Projectile Damage Entity Event**
Executes code when a projectile damages an entity.
- Cancellable: True; auto-cancelled: False

### `EntityExplode`
**Entity Explode Event**
Executes code when an entity explodes.
- Cancellable: True; auto-cancelled: False

### `EntityDeath`
**Entity Death Event**
Executes code when an entity dies by natural causes.
- Cancellable: True; auto-cancelled: False

### `VehicleDamage`
**Vehicle Take Damage Event**
Executes code when a vehicle entity (minecart or boat) is damaged.
- Cancellable: True; auto-cancelled: False

### `Transform`
**Entity Transform Event**
Executes code when an entity turns into another or group of others for any reason.
- Cancellable: True; auto-cancelled: False

### `RegrowWool`
**Sheep Regrow Wool Event**
Executes code when a sheep regrows its wool.
- Cancellable: True; auto-cancelled: False

## Block `game_event` — Game Event (35 actions in this file)

Used to execute code when something happens in the world.

### `CauldronChange`
**Cauldron Change Event**
Executes code when a cauldron's level or contents changes.
- _Works with:_ Cauldron
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `FurnaceBurn`
**Furnace Burn Event**
Executes code when a furnace consumes fuel.
- _Works with:_ Furnace blocks
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `LeavesDecay`
**Leaves Decay Event**
Executes code when leaves decay.
- _Works with:_ Leaves blocks
- Cancellable: False; auto-cancelled: True
- _Flags:_ world-plot only

### `BlockIgnite`
**Block Ignite Event**
Executes code when a block ignites.
- _Examples:_ ⏵ Fire Charge used on a block ⏵ Flint and Steel used on a block
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `CrafterCraft`
**Crafter Craft Event**
Executes code when a crafter crafts an item.
- _Works with:_ Crafter
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `ChunkLoad`
**Chunk Load Event**
Executes code when a chunk is loaded.
- _Info:_ The event block location will be set to the bottom corner of the chunk.
- Cancellable: False; auto-cancelled: False
- _Flags:_ world-plot only

### `BlockGrow`
**Block Grow Event**
Executes code when a block grows naturally.
- _Examples:_ ⏵ Wheat growing ⏵ Turtle Eggs hatching ⏵ Pumpkin growing
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `BeaconActivated`
**Beacon Activated Event**
Executes code when a beacon is activated.
- _Works with:_ Beacon
- Cancellable: False; auto-cancelled: False
- _Flags:_ world-plot only

### `BlockExplode`
**Block Explode Event**
Executes code when a block explodes.
- _Works with:_ Bed; Respawn Anchor
- Cancellable: False; auto-cancelled: True
- _Flags:_ world-plot only

### `BlockDispense`
**Block Dispense Event**
Executes code when a block dispenses an item.
- _Works with:_ Dispenser
- Cancellable: False; auto-cancelled: True
- _Flags:_ world-plot only

### `PistonRetract`
**Piston Retract Event**
Executes code when a piston retracts.
- _Works with:_ Piston
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `VaultDisplayItem`
**Vault Display Item Event**
Executes code when a vault displays an item.
- _Works with:_ Vault
- Cancellable: False; auto-cancelled: False
- _Flags:_ world-plot only

### `FluidLevelChange`
**Fluid Level Change**
Executes code when a liquid's level changes.
- _Works with:_ Water; Lava
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `BlockFertilize`
**Block Fertilize Event**
Executes code when a block is fertilized.
- _Works with:_ Plants; Grass Block
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `SpongeAbsorb`
**Sponge Absorb Event**
Executes code when a sponge absorbs water.
- _Works with:_ Sponge
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `PistonExtend`
**Piston Extend Event**
Executes code when a piston extends.
- _Works with:_ Piston
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `SculkBloom`
**Sculk Bloom Event**
Executes code when a sculk catalyst blooms.
- _Examples:_ ⏵ Entity killed near a sculk catalyst.
- Cancellable: False; auto-cancelled: True
- _Flags:_ world-plot only

### `BlockFade`
**Block Fade Event**
Executes code when a block fades.
- _Examples:_ ⏵ Snow melting due to a nearby light source ⏵ Fire burning out after a time ⏵ Coral fading to dead coral ⏵ Turtle Egg bursting when hatching
- Cancellable: False; auto-cancelled: True
- _Flags:_ world-plot only

### `VaultChangeState`
**Vault Change State Event**
Executes code when a vault changes its state.
- _Works with:_ Vault
- Cancellable: False; auto-cancelled: True
- _Flags:_ world-plot only

### `CampfireStart`
**Campfire Start Event**
Executes code when a campfire starts cooking.
- _Works with:_ Campfire
- Cancellable: False; auto-cancelled: False
- _Flags:_ world-plot only

### `PlotShutdown`
**Plot Shutdown Event**
Executes code when a plot no longer has players and is shutting down.
- Cancellable: False; auto-cancelled: False

### `ChunkUnload`
**Chunk Unload Event**
Executes code when a chunk is unloaded.
- _Info:_ The event block location will be set to the bottom corner of the chunk.
- Cancellable: False; auto-cancelled: False
- _Flags:_ world-plot only

### `BlockSpread`
**Block Spread Event**
Executes code when a block spreads.
- _Examples:_ ⏵ Mushroom spreading ⏵ Fire spreading
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `NotePlay`
**Note Block Play Event**
Executes code when a note block plays a note.
- _Works with:_ Note Block
- Cancellable: False; auto-cancelled: False
- _Flags:_ world-plot only

### `Redstone`
**Redstone Power Event**
Executes code when a redstone current changes.
- _Examples:_ ⏵ Redstone dust changes power ⏵ Door is activated ⏵ Lever is switched on ⏵ Lightning rod struck by lightning
- Cancellable: False; auto-cancelled: False
- _Flags:_ world-plot only

### `BlockMove`
**Block Move Event**
Executes code when a block moves.
- _Works with:_ Water; Lava; Dragon Egg
- Cancellable: False; auto-cancelled: True
- _Flags:_ world-plot only

### `MoistureChange`
**Moisture Change Event**
Executes code when soil moisture changes.
- _Works with:_ Farmland
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `BellRing`
**Bell Ring Event**
Executes code when a bell is rung.
- _Works with:_ Bell
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `LagSlayRecover`
**Plot LagSlayer Recover Event**
Executes code when a plot recovers from a LagSlayer halt.
- Cancellable: False; auto-cancelled: False

### `BlockBurn`
**Block Burn Event**
Executes code when a block is destroyed by fire.
- _Works with:_ Flammable blocks
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `PlotStartup`
**Plot Startup Event**
Executes code when a plot is first started.
- Cancellable: False; auto-cancelled: False

### `TNTPrime`
**TNT Prime Event**
Executes code when TNT is ignited.
- _Works with:_ TNT
- Cancellable: False; auto-cancelled: True
- _Flags:_ world-plot only

### `BlockForm`
**Block Form Event**
Executes code when a block is formed, typically through natural means.
- _Examples:_ ⏵ Obsidian forming from contact of lava with water ⏵ Concrete forming from concrete powder mixing with water ⏵ Snow forming due to a snow storm ⏵ Ice forming in a snowy biome
- Cancellable: True; auto-cancelled: False
- _Flags:_ world-plot only

### `Brew`
**Brew Event**
Executes code when brewing completes.
- _Works with:_ Brewing Stand
- Cancellable: False; auto-cancelled: False
- _Flags:_ world-plot only

### `BlockCook`
**Block Cook Event**
Executes code when a block cooks an item.
- _Works with:_ Furnace blocks; Campfire
- Cancellable: False; auto-cancelled: False
- _Flags:_ world-plot only
