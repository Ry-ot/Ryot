# Ryot
Rust kanarY OT

## Customizable AI Module

https://github.com/zkat/big-brain/blob/main/src/actions.rs

### Finite State Machines (FSMs)
https://github.com/Zeenobit/moonshine_behavior/blob/main/examples/bird.rs
FSMs are one of the simplest ways to implement game AI. They work by having an AI character switch between a finite number of states (like patrol, chase, attack) based on certain triggers or conditions.
Relevance: Ideal for NPCs with simple behavior patterns. Easy to implement and understand.
Application: Commonly used in enemy AI in action games, patrol behaviors in stealth games, and basic decision-making in various genres.

### Behavior Trees
Behavior trees are more flexible and scalable than FSMs. They represent AI logic as a hierarchy of tasks, which can be sequences, selectors, or more complex structures.
Relevance: Allows for more complex and varied AI behaviors. Easier to maintain and extend compared to FSMs.
Application: Widely used in modern games for complex NPC behaviors, such as in strategy games or advanced enemy AI in action games.

### Pathfinding and Navigation
https://github.com/evenfurther/pathfinding

Essential for any AI character that needs to move around the game world, pathfinding algorithms (like A* or Dijkstra’s algorithm) calculate the best path from one point to another.
Relevance: Crucial for a wide range of games, ensuring that AI characters can navigate the world realistically and efficiently.
Application: Used in almost every genre, from RTS and RPG games for character movement to puzzle games for enemy navigation.

### Steering Behaviors
These are algorithms for autonomous character movement, controlling how an AI entity moves in response to its environment. Behaviors include seek, flee, arrive, wander, and more.
Relevance: Useful for creating natural and responsive movement patterns for AI entities.
Application: Common in open-world games, simulations, and any game where natural movement is important.

### Decision Trees
Decision trees are a structured way for AI to make a series of decisions based on game state, leading to different actions or outcomes.
Relevance: Offers a straightforward method for implementing AI decision-making, especially when the number of choices and conditions is manageable.
Application: Useful in strategy games, puzzle games, and certain types of role-playing games where AI needs to make strategic choices.

### Starting Strategy
- **Focus on Core Patterns:** Initially, focus on implementing FSMs and Behavior Trees, as they cover a wide range of AI needs and are fundamental in game AI.
- **Pathfinding as a Basic Necessity:** Include basic pathfinding since it's essential for any game with moving characters.
- **Expand Gradually:** As your framework develops, explore more complex patterns or specialized algorithms like Steering Behaviors and Decision Trees.

**Duration: 3-4 weeks**  
Activities:
Develop basic AI components like pathfinding, simple decision-making, and pursuit/evasion behaviors.
Test these behaviors in simple game scenarios.
Phase 3: Advanced AI Features and Flexibility

**Duration: 4-6 weeks**  
Activities:
Implement more complex AI behaviors and features, such as group tactics or learning mechanisms.
Ensure AI components are easily customizable and can interact with other game systems.

## Custom Scripting Support
https://github.com/amethyst/rlua
https://github.com/rhaiscript/rhai

Phase 1: Integration of Scripting Engine

Duration: 2-3 weeks
Activities:
Choose and integrate a scripting engine (like Lua) with the Bevy engine.
Create a basic API for scripts to interact with game entities and components.
Phase 2: Scripting API Development

Duration: 4-5 weeks
Activities:
Expand the scripting API to cover a wide range of game development needs.
Develop documentation and examples to help users get started with scripting.

## Procedural Content Generation Tools
   Phase 1: Core Algorithms Development

Duration: 3-4 weeks
Activities:
Implement core procedural generation algorithms (e.g., for terrain, level layout).
Test these algorithms with basic game scenarios.
Phase 2: Customization and Integration

Duration: 4-5 weeks
Activities:
Add customization options for procedural algorithms.
Ensure seamless integration of generated content with the Bevy engine and game logic.

## Networking and Multiplayer Support
   Phase 1: Preliminary Research and Design

Duration: 1-2 weeks
Activities:
Research multiplayer game networking concepts and existing Rust libraries.
Outline a basic design for how networking will integrate with Bevy and your custom systems.
Phase 2: Basic Networking Implementation

## (Future Development)
- Start with implementing basic networking capabilities like data synchronization between clients.
- Gradually build up to more complex multiplayer features.

## General Considerations
- Iterative Development: Adopt an iterative development approach, where you build a minimal viable version of each feature and then gradually expand and refine it.
- Testing and Feedback: Regularly test each component in real game scenarios and seek feedback from potential users or the Bevy community.
- Documentation and Community Engagement: Continuously update your documentation and engage with the community to ensure your enhancements are meeting real-world needs.