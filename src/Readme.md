# Understanding Basics Of Game Development
- we find three fields: world, schedule, and runner. The world field stores all of our game's data, the schedule holds the systems that operate on this data (and the order in which they do so) and the runner interprets the schedule to control the broad execution strategy
- Bevy uses the Entity Component System paradigm
- Entities are unique "things" that are assigned groups of Components, which are then processed using Systems.