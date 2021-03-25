# Feature Name: customization-overhaul

## Summary

Customization in *Fonts of Power* has some fundamental issues centering around character identity and analysis paralysis.
With a much better idea of the core systems of the game, we're ready to take a holistic look at how the pieces fit together.

## Motivation

Quite a few current customization systems exist. Our current analysis of these systems shows several critical problems, particularly around gear:

1. Species
   1. Very limited impact: often feels like a flavor choice and becomes irrelevant at higher levels
   2. No species-specific hooks exist
2. Class
   1. Subclasses are cool, but characters of the same subclass often end up feeling very similar
   2. No class-specific hooks exist
3. Power selection
   1. Very interesting decisions
   2. Only some classes have powers to select
   3. Some analysis paralysis exists
4. Combat talent
   1. Very cool and character defining
   2. Overlap with gear affixes is confusing
   3. No way to get more at higher levels except through gear affixes (weird from a ludonarrative perspective)
5. Skill talent
   1. Really interesting flavor choice with fringe adventure benefits
6. Skill proficiencies
   1. Very interesting for combat, problem solving and flavor
   2. Number of skill proficiencies is small enough to feel like a meaningful customization, but large enough to allow for exploration
   3. Fail-forward rules really help these feel impactful without causing huge number gaps like in Pathfinder
7. Attributes
   1. Very interesting trade-offs for most classes
   2. Easy to understand and allocate
   3. Rolled attributes are a great character seed
8. Ideals
   1. Fantastic flavor hook for both players and NPCs
   2. Simple-ish mechanical integration with Deals and Ideals
9. Gear
   1. Overwhelming complexity
      1. Largely due to interactions and number of options
      2. Promotes intense theory-crafting that make it hard to adapt to realities of gameplay
      3. Managing gear on the tabletop is very challenging; creation is nearly impossible without specialized digital tools
      4. Swapping gear can completely change playstyles
   2. Character identity weakening: abilities are defined by the gear they have, not who they are
      1. Swapping gear makes your character feel very different, reducing willingness to experiment due to flavor concerns
      2. Characters of the same class (and especially subclass) can feel very similar. This can create intraparty conflict and reduce replayability.
      3. Free selection of affixes really weakens the experience of discovering who your character is
   3. Balancing difficulties
      1. Some affixes are very challenging to balance between different classes
      2. Negative and 0 cost affixes are prone to munchkin-like abuse and can lead to very high complexity
      3. Crafting gear takes a huge amount of downtime with so-so roleplaying impact
      4. Ease of swapping increases ability to hard counter specific encounters, particularly when combined with free selection
      5. Getting new gear is not very impactful due to already-optimized builds
      6. Gear and essence crystal economy is very important due to power budget tied up in gear
   4. Miscellaneous
      1. Half-point affixes are clunky
      2. Problems with gear cascade to monster building
      3. Free selection of affixes limits player interest in testing out alternative affixes in a niche they care about
10. Consumable choice
    1. Complexity, character identity and balancing issues of gear, but weaker
    2. "Signature consumable" pattern is not supported very well
    3. Discovering new consumables isn't exciting due to free selection of affixes

The rolling respec on level-up is a fantastic addition to almost all of these choices, and helps them feel like part of your character without being truly permanent.
As a result, gear feels both more persistent than other character customizations because it costs a scarce resource (downtime activities) to create, but also less persistent, since you can tweak it as many times as you'd like or instantly pick up a new piece of gear.

## Guide-level explanation

Explain the proposal as if it was already included in the game and you were teaching it to a reasonably-experienced player. That generally means:

For designer RFCs:

- Introducing new named concepts.
- Discuss how it relates to other existing systems in the game.
- Describe the player fantasy that we're trying to hit.

For technical RFCs:

- Explaining how developers should *think* about the feature, and how it should impact the way they use the code base. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, explain how this feature compares to similar existing features, and in what situations the user would use each one.

## Reference-level explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

## Drawbacks

Why should we *not* do this?

## Rationale and alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
- Why is this important to implement as a feature of *Fonts of Power* itself, rather than in splat material?

## [Optional] Prior art

Discuss prior art, both the good and the bad, in relation to this proposal.
This can include:

- Does this feature exist in other libraries and what experiences have their community had?
- Papers: Are there any published papers or great posts that discuss this?

This section is intended to encourage you as an author to think about the lessons from other tools and provide readers of your RFC with a fuller picture.

Note that while precedent set by other engines is some motivation, it does not on its own motivate an RFC.

## Unresolved questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before the feature PR is merged?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?

## [Optional] Future possibilities

Think about what the natural extension and evolution of your proposal would
be and how it would affect Bevy as a whole in a holistic way.
Try to use this section as a tool to more fully consider other possible
interactions with the engine in your proposal.

This is also a good place to "dump ideas", if they are out of scope for the
RFC you are writing but otherwise related.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future RFC; such notes should be
in the section on motivation or rationale in this or subsequent RFCs.
If a feature or change has no direct value on its own, expand your RFC to include the first valuable feature that would build on it.
