# Building Monsters

## Monster Attributes

Like players, monsters have the following attributes:

1. Life
2. Essence
3. Attributes
4. Skill proficiency (3)
5. Proficiency bonus
6. Basic attack and special attack bonus
7. Basic defense and special defense

They are primarily defined by:

1. Features
2. Powers
3. Arms and armor
4. Motivation
5. Spell card

However monsters do not have:

1. Exhaustion
2. Access to the Essence Tap feature
3. Death's door
4. Exploration-focused powers or features
5. Ideals
6. Trinkets

## Scaling monsters

Each monster can scale in two ways:

1. Turns per round (scales with number of players to produce a balanced action economy)
2. Tier (scales with player proficiency bonus to keep parity in features and statistics)

Scaling turns per round increases:

1. The number of turns taken in each round
2. Max life (multiply by number of turns)
3. Max essence (multiply by number of turns)
4. The number of powers known (1 + 1/2 turns per round, rounded down)

It may also unlock a spell card!

Scaling tier is somewhat more complex, increasing:

1. Attributes (5 + tier)
2. Proficiency bonus (tier)
3. Number of features (tier)
4. Gear point total on arms and armor (8 + 2 * tier)

## Spell Cards

Spell cards are powerful, once-per-fight effects.
Each monster has an associated spell-card: it unlocks only if the number of turns per round is strictly greater than half the number of players (as always, round up).

## Monstrous Features

Montstrous features work just like player features, and are intended to be equivalent power to feats and non-capstone class features.

Many simple options exist, and when designing a monster, you should generally be quite mindful of the complexity budget.
Stick to one or two interesting and self-synergistic ideas about how a monster will fight.

Unlike for players, monster have access to negative features.
When a negative feature is taken, the monster may take an additional feature to compensate.

## Monstrous Powers

These work just like players.

## Notes for the monster manual 

- We are going to make the monster manual. By default, making custom monsters is handled by the hacking guide
- Allow worldpspeakers to reflavor monsters as they see fit:
	- Change look and feel
	- Change any non-physical damage type for another one 
	- Swap any ailment for another
	- Swap any affliction for another
	- Swap skill proficiencies
	- Change special defenses that their powers target (if applicable)
	- Spell cards?
	- Definitely do not let them swap powers or features (balance concerns)
- The monster manual should give an explicit order in which powers and features are gained. Maybe let the WS reorder these? (If it becomes inconvenient for us to design around this, then remove this ability.)

## Notes for the hacking guide regarding monsters

- Mention that you shouldn't hard dump attributes without penalty
	- If you make a monster without any special attacks, we recommend keeping its Expertise between -1 and +1
	- Same for Presence and Presence scaling effects (usually power damage)
- Note the weird quirk with monster features:
	- Features scale precisely with tier, which is meant to match the players' proficiency bonus
	- This results in mosters getting slightly more features than player have feats, and getting them slightly earlier (on certain levels)
	- This tradeoff was done intentionally to avoid making the math really annoying
	- But it means that a well optimized moster build will start to outscale players at high levels
	- The monster manual monsters are intentionally slightly suboptimal to account for this, with the assumption that a high level player build will usually be pretty optimal
- You are allowed to get pretty funky with monster powers.
	- For example, passive effects can be 0AP, 1 essence, and the monster must use it on the beginning of its turn if able.
	- You're allowed to have powers that work like reactions, instead of normal actions, with whatever trigger makes sense.
- Note that the difference between powers and features is that powers always cost at least 1 essence to use. However, features are allowed to introduce essence costs. Use your judgement.
- Monsters should have fairly low complexity budgets. Most of them will only be around for a few turns!
