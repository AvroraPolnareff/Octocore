# Abstract
What is sequencer? If we try to describe it in the hardware/software world, the word "sequencer" can be associated with many things - notes on the midi track, automation flow, steps, arrengement, tracker's table, p-locks and many other things. But what really is sequencer? When we look closer at all of these things, boom, the basic idea is simple: sequencer at it's core is event dispatcher with an controllable flow of events. Isn't it simple? The steps, p-locks, notes on piano roll is just events. tracks are well... chains of sequences? separate sequencers? patterns are just grouped sequences with some common props: length, speed, sounds.

# What sequencer is good for push 2 interface?
native push 2 interface (ableton)
experience with software/hardware sequencers (elektron/ableton session mode/electron)

# how tracks should work
## elektron 
all tracks are sequences (+ sound) with fixed amount of steps. Step are the smallest event there. Group of tracks - pattern.
track parameters tied to pattern
octatrack - track parameters tied to parts
pros:
- hands-on experience while editing.
- consistent behaviour when changing patterns
- timing between steps can be easily adjusted for each track
cons:
- write-only approach, hard to edit or rearrange things
- need to remember a lot or dive a lot to actually understand how the sequence works
- unable to preiew sequence from given step
- editing long sequences is painful
## daw
daw - track parameters tied to track
### ableton session view
track


# Flow
