# **NOTE**:
## FLOWY IS MASSIVELY UNFINISHED AND DIDNT MAKE THE GAME JAM CUT OFF DUE TO LIFE...


# What is flowy?

Flowy is a simple 2D game made in the Bevy game engine. It was created as an entry into the `New Year New Skills` 2024 Game Jam hosted on Itch.io. The theme of Jam was `Creating Connections`.

# Whats the game about?

In flowy you play as a somewhat frustrated civil planner who is tasked with creating new water networks for small towns. You're goal is to connect all the houses and businesses to the water plant making sure there's enough for copious cups of tea whilst remaining in budget. Due to budget cut backs the days of unlimited spending on water networks are over and timescales are tight. You must now make sure that you don't go over budget and keep within the timer!

The pipes can be placed wherever you want, but you must make sure that the water flows correctly. If you make a mistake, you can always remove the pipe and try again. Each section of pipe costs money and this is dependent on the length of the pipe, the type of pipe, the size and amount of extra equipment (pumps, etc.) and the terrain that the pipe is placed on. You must make sure that you don't go over budget!

# How do I play?

# Controls

- Left Click: Start/End pipe placement
- Right Click: Delete equipment placement
- Mouse Wheel: Select Equipment to place


# Grid system
Flowy is built on a grid system. Each grid cell has a UUID all actions performed by the mouse are mapped to a grid cell using a simple, fast uuid look up instead of a series of complex calculations. This allows for a very fast and responsive game at the expense of some setup time.

