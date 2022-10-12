# General advice
False positives are a possibility. Consult the [glossary](https://wiki.project-tamriel.com/wiki/Glossary) if you don't know what a word means.

# Records

## Does not match a known ID scheme
This ID does not match the [guidelines](https://wiki.project-tamriel.com/wiki/Modding_guidelines#ID_Guidelines).

## Has a Tamriel Data ID
Either this is a dirty edit, or someone did not follow the guidelines.

## Is auto calculated
This spell is auto calculated which means that any auto calculated NPC can have it. Including vanilla NPCs.

## Is dead but does not have corpse persists checked
This actor is going to disappear after three days. Possibly without the player having ever seen the body.
This actor will also play a death scream when it enters the active grid.

## Is missing a sound gen
This creature does not have a sound gen defined.
This doesn't necessarily mean it doesn't play the right sound as the [rules](https://gitlab.com/OpenMW/openmw/-/issues/4813) are somewhat complicated,
but it might mean the creature uses the wrong sound effects.

## Shares its ID with a record of type X
This ID is used for multiple different things. Not generally harmful, but definitely confusing.

## Should have ID b_v_X_head_01
See [this bug](https://github.com/TD-Addon/TD_Addon/issues/110).

## References X
This object contains or creates an object that is supposed to be unique to the vanilla game.
This includes things like quest rewards and vendor chests.

## Has a fog density of 0
This can cause [graphical issues](https://en.uesp.net/wiki/Morrowind_Mod:Fogbug).

# References

## Persistent object used multiple times
[Persist objects](https://wiki.project-tamriel.com/wiki/Scripting#References_Persist) are meant to be used in scripts.
There should only ever be one instance of these objects. If an object is persistent but not used in any scripts, it should not be persistent.

## Duplicate references
Having two of the same object occupying the same position is redundant. The warning contains the position of both references in case the distance threshold is raised.

# Supply chests

## Not available to all ranks
Supply chests should be available to all faction members.

## Not owned by the faction
Supply chests should use a unique record for each faction and they should always be owned by the faction they were made for.

# NPCs
See the [NPC guidelines](https://wiki.project-tamriel.com/wiki/Modding_guidelines#NPC_Guidelines).

## Does not have a script
Any NPC not on Vvardenfell must have a script. NPCs on Vvardenfell may still be assigned scripts.

## Uses unknown script
The script this NPC uses is not defined in this file and does not start with `T_`. It may or may not have the requisite variables.

## Uses script which does not define
The script this NPC uses is missing one or more required local variables.

## Has class X, which should be Y
This NPC has a class that is not suitable for use in this province.

## Has auto calculated stats and spells
Spells have names that are visible to the player (either when buying them or when being affected by them.)
These names imply a certain culture, meaning NPCs outside of Morrowind should not receive vanilla spells.

## Is not using unique head/hair
This NPC should be using the asset designed for it.

## Is using head/hair
This head or hair is inappropriate for use by this NPC; usually for cultural reasons.

# Keys
A misc item is a key if it has the key flag. This is a property of the record and determines if it can be sold to merchants and detected by Detect Key.

## Key not defined in this file
If the key record is not defined in this file, it might not have been flagged as a key by the CS.

## Is not a key
This misc item has `key` in its ID, but is not flagged as a key.

# Services

## Buys magic items but does not have a barter menu
Flagging an NPC or Creature as being a vendor of magic items does not enable the barter button in the dialogue window. This requires them to be a vendor of one of the other types.

## Does not barter
A class or NPC that implies barter services does not offer them.

## Does not have any barter gold
This NPC barters but does not have any gold to buy items with.

## Does not offer travel services
Certain classes have voice lines and greetings that imply they can transport the player.
If an NPC with such a class does not offer travel services, they should be assigned another class.

# Orphaned objects
These are probably leftovers from previous releases. Of course, they might also be intended for an unfinished or unmerged quest.

## Script never started
A script that is never started is likely to be unused. It is possible to use a script as a collection of variables, akin to declaring global variables.
In this case the script is not unused, but the approach should still be reworked.

## Unused records
A record is considered unused if it is not present in the world, not in any inventory or leveled list, and not spawned via script.
This check is not performed in TD mode.

Be aware that, when checking a claim file, it is possible for the record to be used by the section file the claim is meant to be merged into.

# Text

## Contains odd character
These characters tend not to look very good in game.

## Contains a single hyphen
Vanilla sometimes uses one `-`, and sometimes it uses `--`. We always use the latter.

## Contains a short/an overlong ellipsis
An ellipsis should be composed of three dots.

## Contains double spaces
This is a game, not a piece of paper composed on a typewriter.

## Contains doubled up punctuation
A typo that should be fixed.

## Contains punctuation preceded by whitespace
Common in certain languages, but not in English.

# Dialogue

## Has an unnecessary (Not) class/faction/race/sex/id filter
This line that is assigned to a specific actor has a filter that checks something about that actor that cannot ever change.

## Has a NoLore/T_Local_NoLore/T_Local_Khajiit/T_Local_NPC filter
Like the above, but for local variables.

## Does not have a T_Local_NoLore filter
This line is available to NPCs who should not have lore.

## Has a Not Local NoLore filter
All NPCs added by these projects should have a NoLore variable meaning this line is not available to them.

## Has a Local/Not Local X filter
This variable comparison is incorrect.

## Has a class filter
This class is not appropriate for use in this province. The line should be inaccessible.

## Does not have a known project specific local filter
This line is missing a filter that unambiguously ties it to the project, meaning it might show up on NPCs added by other mods.
Filter the line to a local variable that starts with your province's ID prefix or use one of the T_D variables.

If this line is meant to override a vanilla line that does not have a NoLore filter, add `;SV: overrides vanilla` to the result script.

## Has no text
So why does it exist?