# Environmental Complications

## Objects

Combat doesn't typically occur on a flat, featureless expanse.
Objects of all sorts, from doors to switches to trees to ziplines are common elements in fights.

Objects can be targeted with your actions, allowing you to Shove them, Strike them, Interact with them and so on.
By default, all objects have 10 life, are immune to ailments and afflictions, cannot be knocked prone and are automatically hit by attacks.
Objects do not take damage from multi-target attacks or tile effects unless they are **fragile**.
Unsurprisingly, **stationary** objects are immune to forced movement.

You can use Interact on many objects to use or alter them in interesting ways.
Use the Scan action on an object to find out what this will do!

## Tile Effects

While in combat, tiles are commonly modified in dangerous or interesting ways: sacred shrines, clinging fog, walls of fire and so on. Collectively, these are referred to as **tile effects.**
Creatures moving by **teleportation** do not enter any intervening tiles, and so ignore the tile effects of the tiles between their origin and destination.

Tile effects (including ground effects) are triggered when:

- a creature begins their turn in the tile effect
- a creature enters the tile effect
- a creature is in a tile effect as it is created

Once a creature has triggered a particular type of tile effect, they cannot trigger it again until the start of their next turn.

Various effects, from glue to oil slicks to pools of acid to thorny underbrush, cover the ground.
This special type of tile effects are called **ground effects**.
As you might expect, creatures that are flying or aloft ignore ground effects.

### Difficult terrain

The most common ground effect is **difficult terrain,** which is difficult to cross, due to obstructions, unsteady footing or so on.
Moving out of a tile that is difficult terrain costs an additional tile of movement.

The effects of difficult terrain do not stack with the effects of being prone.

## Visibility and Cover

There are three levels of **visibility** that describe how well you can see other creatures or objects: clear, obscured and invisible.
These descriptors are determined on the basis of both the character attempting to see and the target that is being seen: covering your eyes with your hands makes everyone else invisible to you, but not to anyone else.

By default, your target is **clear.**
As your target becomes **obscured:**

- you have disadvantage on skill checks made as part of the Scan action targeting them

Once the target is fully **invisible:**

- you have disadvantage on skill checks made as part of the Scan action targeting them
- single-target attacks you make against them have disadvantage

**Light** is the most common and important driver of visibility. There are three levels of lighting: **bright light, dim light,** and **darkness**, which are all tracked on a per-tile basis. Creatures and objects are obscured while in dim light, and invisible while in darkness.

<div class="infobox">

**A Light in the Darkness**

If you have the tools (or patience) to carefully track light levels during combat in your game, a few questions are quickly raised.

Every light sources have a **light radius**, and shed bright light in that radius, and dim light in that radius beyond that.
The light radius of common objects are:

- **1 tile:** a candle
- **2 tiles:** a small fire
- **3 tiles:** a sputtering torch
- **6 tiles:** a blazing torch, bonfire or lantern
- **12 tiles:** a brilliant magical light, gained by the _light_ trinket affix

Players can create objects with up to a 6 tile light radius using **Prepared for Anything.**
Lighting or extinguishing your own torch while in combat takes an Interact action, as does extinguishing a light source found in the environment (typically at melee range).
Creatures carrying a light source can fight normally: _Fonts of Power_ is not a simulationist system.
Extinguishing other creatures' light sources is both powerful and frustrating: it can only be done with an appropriate affix or power.

</div>

**Shroud** effects, such as a cloud of smoke, rain, thick underbrush or fog make objects both within them and seen from within them harder to see.
If a tile is **lightly shrouded**, objects within that shroud and objects seen from within that shroud are objects obscured.
If a tile is **heavily shrouded**, the objects are invisible instead.

Physical objects can also affect visibility and, independently, provide **cover,** shielding creatures behind them from attacks.
There are two kinds of cover, partial cover and full cover. Like visibility, cover is relative to both the actor and the target.

**Partial cover** is provided by things like castle parapets, a pile of boxes or a sturdy tree.
While your target is behind partial cover, attacks that you make have disadvantage. Creatures that are at least one tile larger than the target grant partial cover.

**Full cover** is commonly granted by things like solid walls, closed doors or being inside of a monster's stomach.
While your target is behind full cover, your attacks automatically miss, and you cannot target them with other effects such as when applying boons, creating ground effects or using the Scan or Spot actions.

When an effect does not physically originate from the attacking creature, cover needs to be calculated with respect to the target and the source of the effect. This is an important rule when, for example, a ball of fire explodes in mid-air, or the ground becomes covered in a dangerous effect.

In general, when a creature is behind partial cover, they are obscured to you, and when a creature is behind full cover, they are also invisible to you.
Transparent objects, such as windows or shimmering walls of force are an obvious exception to this.
Unless specifically stated, you do not need to be able to see creatures in order to target them with an attack.

## Height Tiers

Environments commonly contain interesting topological features: hills, boulders, furniture, staircases, terraces and so on.
We can create interesting vantages by modelling these environments as having several tiers of height.

The edge between two **height tiers** can either be **abrupt** or **gradual**, which affect how easy they are to traverse.
Abrupt edges involve awkward, challenging climbs, while gradual edges are found on hills, stairs or ramps.
Gradual edges can only be found between two tiles of adjacent heights, while abrupt edges can cause sudden drops in height.

Flying creatures cannot increase their height tier freely, and are always considered to be at the height tier of the tile that they are above.
Creatures cannot fly in tiles whose ceiling is less than two height tiers above their floor.
Creatures who are larger than zero tiles in size cannot move through tiles whose ceiling is one height tier above their floor unless they are prone.

### Travelling Up and Down

Gradual edges do not require any special rules for movement: both voluntary and forced movement work as normal.

Safely travelling up or down an abrupt edge takes one additional tile of movement per height tier changed.
Creatures that are flying must spend one additional tile of movement for every two tiers of height changed beyond the first.
Changing height tiers does not affect the distance that can be travelled via teleportation directly, but see the standard rules for how ranges are measured across height tiers.

Forced movement cannot move creatures _up_ across abrupt edges, but can move creatures _down_ abrupt edges, typically causing **falling damage**:

1. Falling deals 2d6 base physical damage for each height tier that the creature fell, measured from the start of the movement to the end of it.
2. The creature that fell makes a Agility (Athletics) skill check to attempt to land gracefully.
3. Subtract the result of this skill check from the base damage rolled in step 1 to get the final falling damage dealt.

For example, falling from the 3rd tier in an area, to the bottom (zeroth) tier would incur 6d6 physical damage. If you rolled a 13 on your skill check, the total damage would be 6d6 - 13.

Creatures can also choose to voluntarily walk off cliffs, offering them a quick and painful way to get down.

Creatures which are flying or aloft do not take falling damage.

### Attacking across height tiers

When two creatures are engaged in combat and are on different height tiers, the creature on the higher height tier is said to have the **high ground** while the other creature has the **low ground**.

Holding the high ground offers several advantages:

- Shove attacks made from the high ground have advantage, while Shove attacks made from the low ground have disadvantage
- The range of ranged attacks is asymmetric: for ranged attacks made by the creature on the low ground, the creature on the high ground is treated as being one tile further away for each tier of height advantage. Ranged attacks made by the creature on the high ground are treated as one tile further away for every two tiers of height advantage (rounded down) instead.
- Creatures on the low ground have disadvantage on Grapple attacks.

The range of melee attacks is unaffected for a height advantage of 1; melee attacks cannot be made across a height advantage of 2 or more.

### Height tiers and cover

Height tiers provide different types of cover depending on the relative positioning of the attacker and the defender.

When the combatants are at different height tiers:

- When the creature on the higher tier is on the relevant edge of their tier, no cover is granted.
- When the creature on the higher tier is one tile back from the relevant edge of their tier, both creatures have partial cover.
- When the creature on the higher tier is one tile back from the relevant edge of their tier, both creatures have full cover.

When a higher tier is in-between the two combatants:

- If the defender is prone, they have full cover.
- If the obstacle is one tier higher than the attacker’s position, the defender has partial cover.
- If the obstacle is two or more tiers higher than the attacker’s position, the defender has full cover.
