# Core Actions

## Movement

When you spend movement:

- You may not pass through tiles occupied by enemy creatures unless they are prone.
- You may not end your movement in a tile occupied by another creature unless they or you take up zero tiles.

### Move (1 AP)

Gain two tiles of movement.
Then, spend any number of tiles of movement.

### Lie Down (0 AP)

Become prone.

### Stand Up (1 AP)

Lose the prone status and gain one tile of movement.
Then, spend any number of tiles of movement.

### Take Flight (1 AP)

Gain the flying status.

### Land (1 AP)

Lose the flying status and gain one tile of movement.
Then, spend any number of tiles of movement.

### Leap (1 AP)

Choose a tile within a range equal to the movement you have to spend.
Note that if the source and target are on different height tiers, ranges are measured differently, as described in the rules on height tiers.

Make a Prowess (Athletics) skill check:

- if the result is less than 10, reduce the movement you have to spend by two
- if the result is less than 20, reduce the movement you have to spend by one

Then, spend movement up to the range of the targeted tile:

- if you have enough movement, you move to the targeted square, avoiding any ground effects between you and that tile. Keep any extra movement.
- if you are one short, you spend all movement to move to the targeted square, avoiding any ground effects between you and that tile but are knocked prone.
- if you are more than one short, spend all movement you land a number of tiles away from your starting position equal to the tiles of movement spent.

If the tile is occupied by a creature whose tile you cannot enter (such as most enemies), immediately make a melee Shove attack against the creature in that tile. If you hit, you must either knock them prone or push them out of your way if able.
If you miss, they automatically hit you with a Shove attack and must either knock you prone or push you out of the way.

## Offense

### Grapple (2 AP)

Make a melee special attack against Agility against another creature within a one tile range. If you hit, they gain the [grappled](../../conditions/statuses.md?id=grappled) status, and you gain the [grappling](../../conditions/statuses.md?id=grappling) status.

You have disadvantage on Grapple attacks made against creatures smaller than a quarter of your size (in tiles). You cannot make Grapple attacks against creatures that are more than four times your size. You cannot make Grapple attacks while you have the grappled or grappling status.

You have disadvantage on melee Grapple attacks against creatures who have the high ground. You have advantage on Grapple attacks against creatures who are prone.

### Reverse Grapple (2 AP)

Gain the benefits of the Break Grapple action to free yourself from a grapple. If you succeed, you may shift once, then immediately make a Grapple attack against the creature that was grappling you.

### Strike (2 AP)

Strike at your target with your currently equipped set of arms. Make a basic attack against a creature or object within range of your current arms.

By default, the damage dealt by basic attacks is 1d8 + Prowess.

### Shove (1 AP)

Make a melee special attack against Prowess against a creature within your zone of control. If you hit, you may choose to either knock them prone or push them into an empty adjacent tile of your choice.

Melee Shove attacks have advantage if you have the high ground, and disadvantage if they have the high ground.

## Defense

### Defend (1 AP)

Gain 6 + Presence absorption

### Ward (1 AP)

The first time before the start of your next turn that you would gain any stacks of an ailment or affliction, ignore all ailments and afflictions that would be applied by that effect.

If you use this action multiple times in a single turn, you prevent a second (or third or..) set of ailments or afflictions from being applied to you.

### Block (1 AP, reaction)

Whenever you would be hit by an attack (after seeing the initial results of the attack), you may use the Block reaction. If you do, make a Prowess (Athletics) skill check. If your skill check result is at least 10, you **block** all attacks made against you until the end of the current turn. Halve the efficacy of blocked attacks made against you.

If your skill check result is at least 20, gain one AP.

### Dodge (1 AP, reaction)

Whenever you would be hit a basic attack, Grapple attack or Shove attack (after seeing the initial results of the attack), you may use the Dodge reaction. If you do, make an Agility (Athletics) skill check. If your skill check result is greater than or equal to the attack roll that hit you, **dodge** the attack. Attacks that you dodge are treated as if the attacker missed.

### Break Grapple (1 AP)

Attempt to break another creature’s hold on a target, ending the grappled status on yourself or another creature within 1 tile of you.

To do so, make an opposed Prowess (Athletics) skill check against the grappling creature. If you succeed, the grapple ends.

### Treat (1 AP)

Attempt to treat a creature within 1 tile’s injuries and ailments, curing a single ailment or mitigating a single affliction of your choice.

Select a ailment or affliction. If you succeed on a difficulty 10 Focus (Medicine) skill check, remove all stacks of the chosen ailment or half the stacks of the chosen affliction from them.

## Utility

### Activate (1 AP)

Activate a powerful source of magic.

Choose one:

- use a consumable magic item. These can be found while adventuring, or created using the Devise downtime activity.
- activate a powerful magical vantage in arena, as specified in its description.

Essence crystals can always be used as consumables, restoring 5 essence on use.

Concentrated magic is dangerous stuff!
Each combat, each time you use this action after the first you risk **essence overload**. You may use this action once per combat without any risk.
Make a Focus (Endurance) skill check.
Gain 5 stacks of exhaustion, reduced by 1 for every 5 points of your skill check result.

### Essence Tap (1 AP)

Regain half your life and essence, rounded up.

Then, make a Focus (Endurance) skill check. Gain 3 stacks of exhaustion, reduced by 1 for every 10 points of your skill check result.

### Hide (1 AP)

If your tile is a **hiding spot**, make a difficulty 10 Agility (Stealth) skill check. If you succeed, gain the [hidden](../../conditions/statuses.md#hidden) status.

A tile is a hiding spot if at least two of the following conditions are true:

- it is lightly shrouded
- it is adjacent to any opaque cover
- it is in dim light
- if you are prone

Or, if at least one of the following conditions are true:

- it is heavily shrouded
- it is in darkness
- all enemy creatures are either:
  - out of the line of sight
  - more than 12 tiles away
  - suffering from the blinded ailment
- you are invisible

You lose the hidden status when:

- another creature successfully reveals you with the Spot action
- you end your turn in a tile that is not an eligible hiding spot (see the Hide action)
- you use an action targeting a non-allied creature

### Interact (1 AP)

Interact with an object in your environment. You might pull a lever, quickly slash a rope, kick down a door, throw a torch or so on.

Objects in their environment have their own rules for what using Interact on them does.
This can be discovered by using Scan.

### Recover (1 AP)

Make a Focus (Endurance) skill check. Restore 1 essence for every 10 points of the skill check result.

### Scan (1 AP)

Make an Expertise (X) skill check where X is one of (Anima, Arcana, Fontcraft, Tinkering), based on the monster.

Gain 1 + floor(roll / 10) questions:

- Learn the attributes, basic defense, special defenses, tier and turns per round of a creature of your choice.
- Learn the percentage of life, percentage of essence and the number of stacks of exhaustion that another creature is at.
- Learn either the arms, armor, or trinket affixes of a creature of your choice. If you choose to learn their trinket affixes, you also learn the skills that they are proficient in.
- Learn the details of a single power of your choice. You have advantage on this skill check if this power was used since your last turn or a creature is currently concentrating on the power.
- Learn the tactics and motivation of a creature of your choice.
- Learn the details of a vantage of your choice. If you do not specify a particular vantage, learn one of the worldspeaker's choice instead. The skill used depends on the details of the vantage, and is specified in its description.

### Spot (1 AP)

Choose a hidden creature within a 6 tile range.
Make a Focus (Perception) skill check, opposed by that creature's Agility (Stealth).
If you win, the hidden status on that creature ends.

You have disadvantage on this skill check if you are more than 3 tiles away from them, and have advantage if you are within a 1 tile range.

### Swap (1 AP)

Change the set of arms that you currently have equipped. This also allows you to completely stow your arms (and have no arms equipped), or draw them when you do not have arms equipped.

### Let Go (0 AP)

Stop grappling a target you have grappled.

### Jostle (1 AP)

Choose a creature you are grappling, then either:

- move them up to two tiles.
- force them to stand up, then move them up to one tile.

If they leave your zone of control, you immediately stop grappling them.
