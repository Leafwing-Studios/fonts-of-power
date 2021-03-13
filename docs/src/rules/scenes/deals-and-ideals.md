# Deals and Ideals
Significant social interactions follow the framework of making **deals** with NPCs. Every deal begins with a **pitch:** what do the player characters want to get out of the interaction, and why should the NPC care. There are three factors that combine in order to determine the NPC’s **motivation** after you have made a pitch to them: their **loyalty** to you, their calculation of their **self-interest,** and the **morality** of the deal, reflecting the alignment of the deal with their **ideals.** Deals can be offered by any character, but player characters may choose freely whether to accept or reject a deal.

## Loyalty
**Loyalty** tracks the trust that other characters (either individuals like Innkeeper Bob or groups like the local Wizard’s Guild) have in the player characters, which drives their willingness to cooperate. By default, loyalty begins at zero, and changes according to the following rules, with the number in brackets indicating the change in loyalty:
* **Great Deal (+1):** A deal is made with a motivation greater than 1. 
* **Done Deal (+1 / +2):** The party completes a deal that tangibly benefits the NPC or their allies.
* **Failed Deal (-1):** The party fails a deal that they had made.
* **No Deal (-1):** The NPC proposed a deal to the party, and a deal could not be reached.
* **Broken Trust (-1 / -2):** The party’s actions tangibly hurt the NPC or their allies. This could be betraying them, aiding their rivals, or failing a deal in a way that leaves them in a bind.
* **Liar (-1):** The players fail a Presence (Charm) skill check to lie about a deal by 5 or more.
* **Moralizer (-1):** The players fail a Presence (Guidance) skill check to temporarily change the ideals of the NPC by 10 or more.
* **Call in a Favor (Special):** The characters spend loyalty points above 0 in order to get a bonus to the motivation of the NPC for this particular deal equal to the number of loyalty points spent. Their loyalty decreases by the same amount after the deal is made.

## Self-Interest
Self-interest is the selfish calculation of “what’s in it for me”. There are two factors that make up this calculation: **payoff,** which compares what the NPC has to gain from the deal relative to what they have to lose, and **plausibility,** which reflects how likely they feel the plan is to succeed. Payoff and plausibility are calculated according to the table below, and that decision is made by the worldspeaker alone based on the NPC’s unique perspective and needs, although morality should not enter into this calculation. Sum together the selected modifiers for payoff and plausibility to arrive at a final value for self-interest.

**Payoff and Plausibility modifiers**

<div class="three-column">

Modifier | Payoff | Plausibility
-- | -- | --
-2 | The benefit of success is much less than the cost of failure. | Success is dramatically less likely than failure.
-1 | The benefit of success is less than the cost of failure. | Success is less likely than failure.
0 | The benefit of success is about the same as the cost of failure. | Success and failure are about equally likely.
1 | The benefit of success is greater than the cost of failure. | Success is more likely than failure.
2 | The benefit of success is much greater than the cost of failure. | Success is dramatically more likely than failure.

</div>

## Morality

Ideals reflect the NPC’s moral values: their willingness to help out because it’s the right thing to do, or refuse to help because it would be wrong to do so. In Fonts of Power, there are five **core ideals** that both players and NPCs follow to some degree: Equality, Harmony, Integrity, Liberty, and Progress. These are detailed in the Attributes and Ideals chapter of this book.

Different people care about these ideals to different degrees, based on the values of their culture and their own sense of morality. Each ideal is measured by a score between 0 and 5 (inclusive), reflecting the degree to which the character cares about that ideal. Most people are neither terribly moral nor selfish, and so their ideals sum to 10. The ideals of particularly selfish characters may sum to 5, and the ideals of a particularly self-sacrificing character may sum to 15. For most NPCs you might create, beginning with a value of 2 in each ideal is a great starting point.

<div class="infobox">

**Moral dilemmas**

Many deals have moral factors that weigh both for and against them. For example, you may be attempting to convince a local miner to give you explosives that you can use to destroy the machinery that belongs to the oppressive owner of the mine who’s forcing orphans to work for him. This would be a moral good from the perspective of Equality and Liberty, but a moral harm from the perspective of Harmony and Progress.

Suppose that the miner’s ideals are [4,3,2,1,0] in the standard alphabetical order. The value of 4 in Equality is higher than his value of 1 in Liberty, so we add 4 to the morality of the deal. However, he has a value of 3 in Harmony (which is higher than his value of 0 in Progress), so we subtract 3 from the morality of the deal. The final morality assessment is thus 1, reflecting his mixed feelings about the righteousness of the plan.
</div>

When the player characters propose a plan that either promotes or runs against these ideals, the morality of the deal changes from 0. If a plan significantly improves things along at least one of these ideals, select the relevant ideal that the NPC cares about the most and add its value to the morality score for this deal. If a plan significantly worsens things along at least one of these ideals,  select the relevant ideal that the NPC cares about the most and subtract its value to the morality score for this deal. Only one ideal in each direction matters for computing the morality of a deal, but both directions may be in play at a time.

## Motivation
Finally, motivation is calculated as the sum of loyalty, self-interest and morality. If, after hearing the player’s pitch, their motivation is at least one, they will act to help you. Otherwise, they will refuse to act.

If their motivation is between -3 and 0, the NPC is willing to **renegotiate,** seeking to sweeten the deal in their own favor by modifying their calculation of self-interest until the motivation for the new deal is 1. However, the NPC will not seek to renegotiate if their net motivation is greater than 1, and instead walks away content with the bargain they have scored; causing their loyalty to increase by 1.

## Influencing Deals
Players can influence the outcome of a deal by using their skills and resources in several ways

### Assess
As a group, make a Presence (Insight) skill check to learn the loyalty that the NPC in question has towards your party. If the result is 10 or higher, you learn whether it is positive, negative or zero. If it is 15 or higher, you learn the exact value.

### Call in a Favor
If the party desperately needs a favor, they can sacrifice the loyalty they have earned. Each point of loyalty they spend increases the motivation by 1, stacking with the original bonus due to their loyalty. However, once used, this loyalty is permanently lost and must be rebuilt. You can only spend loyalty in this way if the resulting loyalty would be at least zero.

### Empathy
If a player succeeds on a difficulty 15 Presence (Insight) skill check during conversation with an NPC, they learn the highest and lowest ideals that they hold. The worldspeaker should pick randomly in the case of a tie.

### Ethnography
If a player succeeds on a difficulty 15 Expertise (Humanities) skill check, they know the typical motivations of a group of their choice. Remember however that most individuals exist at the intersection of several groups, and have their own unique experiences as well.

### Deception
The party may choose to lie about their deal, completely fabricating a new story. It may appeal to or go against different ideals than their actual deal. The party must succeed on a Presence (Charm) skill check to convince the NPC of their story.
* The story begins with a modifier of 0 in both payoff and plausibility and requires a difficulty 15 skill check.
* The party can “buy” additional points in either payoff or plausibility by increasing the difficulty by 5. Likewise, they can “sell” these points to reduce the difficulty by 5. Like usual, payoff and plausibility must be between -2 and +2, inclusive. For example, a story with a +2 self interest score would have a final difficulty of 25 (15 + 10), and a story with -1 self interest score would have a final difficulty of 10 (15 - 5). 
* If you fail this skill check, the NPC finds something off about your story and refuses to help. If you fail this skill check by 5 or more, the NPC realizes you are lying and their loyalty decreases by 1.

### Moral Appeal

If a player succeeds on a difficulty 20 Presence (Guidance) skill check, they can attempt to temporarily change the NPCs ideals by 1 while assessing the deal, causing them to either care more strongly about the moral benefits of the task, or overlook any moral questionability. 

NPCs can spot insincere moral arguments: the character making the check has advantage on this skill check if their personal value for that ideal is more extreme than the new value for the NPC’s ideal, and disadvantage if it is less extreme. 

<div class="infobox">

**Sincerity by example**

To illustrate the mechanics of Moral Appeal, suppose the player character’s Integrity is 1, and the NPC’s is 3. If they are trying to convince an NPC to overlook the fact that a deal would involve directly lying, they would attempt to temporarily lower the NPC’s Integrity by 1. Because their integrity is more extreme in the appropriate direction (decreasing, in this case), they have advantage on this skill check.

On the other hand, suppose the player character was attempting to convince the NPC to take responsibility for the crimes they’d committed, again engaging Integrity. As they try to temporarily increase the NPC’s Integrity by 1, they have disadvantage on this skill check. because their integrity is less extreme in the appropriate direction (now, increasing).
</div>

The party can repeat this process until they fail, but Ideals are always between 0 and 5. If they fail by 10 or more, the NPC takes offense and their loyalty is reduced by 1.

### Salesmanship
If the party did not lie about the deal, they can attempt to persuade the NPC about how beneficial the deal would be by making a Presence (Charm) skill check. They can increase the NPC’s perception of the payoff or plausibility of the deal by 1 point per 10 points of their skill check result, rounded down. Like always, each of these values is capped at +2.
