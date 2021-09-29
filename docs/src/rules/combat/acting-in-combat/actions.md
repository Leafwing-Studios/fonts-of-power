# Core Actions

## Movement

### Move (1 AP)

Move up to two tiles, or one tile over difficult terrain.

**You may not pass through tiles occupied by enemy creatures** unless they are prone.

**You may not end your movement in a tile occupied by another creature** unless they or you take up zero tiles.

### Stand Up (1 AP)

Lose the prone status and move up to one tile.

### Take Flight (1 AP)

Gain the flying status.

### Land (1 AP)

Lose the flying status and move up to one tile.

### Jostle (1 AP)

Choose a creature you are grappling, then either:

- move them up to two tiles.
- force them to stand up, then move them up to one tile.

If they leave your zone of control, you immediately stop grappling them.

## Offense

### Grapple (2 AP)

Make a melee special attack against Prowess against another creature within a one tile range. If you hit, they gain the [grappled](../../conditions/statuses.md?id=grappled) status, and you gain the [grappling](../../conditions/statuses.md?id=grappling) status.

You have disadvantage on Grapple attacks made against creatures smaller than a quarter of your size (in tiles). You cannot make Grapple attacks against creatures that are more than four times your size. You cannot make Grapple attacks while you have the grappled or grappling status.

### Reverse Grapple (2 AP)

Gain the benefits of the Break Grapple action to free yourself from a grapple. If you succeed, you may shift once, then immediately make a Grapple attack against the creature that was grappling you.

### Strike (2 AP)

Strike at your target with your currently equipped set of arms. Make a basic attack against a creature or object within range of your current arms.

By default, the damage dealt by basic attacks is 1d8 + Prowess.

### Shove (1 AP)

Make a melee special attack against Prowess against a creature within your zone of control. If you hit, you may choose to either knock them prone or push them into an empty adjacent tile of your choice.

### Attack of Opportunity (Variable cost, reaction)

Strike suddenly, taking advantage of an opening in your opponent’s defenses. You may spend your reaction to make an attack of opportunity whenever an enemy in your zone of control **provokes** an attack of opportunity. By default, creatures provoke an attack of opportunity from enemies whose zone of control they are in whenever they:

- Lose life (except due to one of your actions)
- Spend movement to both enter and leave a hostile creature's zone of control in a single turn. This occurs regardless of the order.

Unlike other reactions, **you may not use your reaction to make an attack of opportunity during your turn.** If you would like to follow up on your own attacks, pick up the [Ferocious](character-options/talents/combat-talents?id=ferocious) combat talent.

You may use any attack to make an attack of opportunity, so long as you meet the attack’s other requirements, such as being within range.

Attacks of opportunity always occur before the triggering action is complete. For example, if an enemy attempts to run past you, you could make a Grapple attack against them, holding them in place just before they leave your zone of control.

Attacks which hit more than one creature can be used as attacks of opportunity so long as they meet the other requirements. However, you have disadvantage on any attack rolls made in this way against creatures other than the one who provoked the attack of opportunity.

## Defense

### Defend (1 AP)

Gain 6 + Presence absorption

### Ward (1 AP)

The first time before the start of your next turn that you would gain any stacks of an ailment or affliction, ignore all ailments and afflictions that would be applied by that effect.

### Block (1 AP, reaction)

Whenever you would be hit by an attack (after seeing the initial results of the attack), you may use the Block reaction. If you do, make a Prowess (Endurance) skill check. If your skill check result is at least 10, you **block** all attacks made against you until the end of the current turn. Halve the efficacy of blocked attacks made against you.

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

Use a consumable, or temporarily attune to a magical item.

Choose one:

- Gain the benefits of a consumable magic item whose gear point total is less than or equal to your proficiency bonus. These are created using the Devise downtime activity.
- Consume an essence crystals directly to restore 5 times your proficiency bonus essence.
- Temporarily attune (or unattune) to a magical item until the end of your next rest, ignoring your attunement limit. Like usual, the enchantment gear point total of these items must be less than or equal to your proficiency bonus.

### Essence Tap (1 AP)


// TODO: Tune
Gain 1 essence.

### Interact (1 AP)

Interact with an object in your environment. You might pull a lever, quickly slash a rope, kick down a door, throw a torch or so on.

### Scan (1 AP)

Make an Expertise (X) skill check where X is one of (Anima, Arcana, Fontcraft, Tinkering), based on the monster.

Gain 1 + floor(roll / 10) questions:

- Learn the attributes, basic defense, special defenses, tier and turns per round of a creature of your choice.
- Learn the percentage of life, percentage of essence and the number of stacks of exhaustion that another creature is at.
- Learn either the arms, armor, or trinket affixes of a creature of your choice. If you choose to learn their trinket affixes, you also learn the skills that they are proficient in.
- Learn the details of a single power of your choice. You have advantage on this skill check if this power was used since your last turn or a creature is currently concentrating on the power.
- Learn the tactics and motivation of a creature of your choice.
- Learn the details of a vantage of your choice. If you do not specify a particular vantage, learn one of the worldspeaker's choice instead. The skill used depends on the details of the vantage, and is specified in its description.

### Swap (1 AP)

Change the set of arms that you currently have equipped. This also allows you to completely stow your arms (and have no arms equipped), or draw them when you do not have arms equipped.

### Let Go (0 AP)

Stop grappling a target you have grappled.
