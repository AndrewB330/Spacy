# Spacy
Spacy

# TODO
- [ ] Server-client
  - [ ] Basic server-client setup
  - [ ] Initial terrain synchronization
  - [ ] Player synchronization
  - [ ] Continuous terrain synchronization
  - [ ] Destruction synchronization
  - [ ] Planets synchronization
  - [ ] Other objects synchronization
  - ...
- [ ] Planet
  - [ ] Planet unloading on client side
  - [ ] Planet saving on server side
  - [ ] Planet coordinate system change
- [ ] Terrain generation
  - [ ] Basic terrain generation
  - [ ] Basic terrain textures
  - [ ] Caves generation
  - [ ] Minerals & ores generation
- [ ] Destruction
  - [ ] Basic destruction
  - [ ] Terrain breaking off and cleaning
  - [ ] Cleaning small pieces to improve perf
  - [ ] Minerals and ores drop
  - [ ] Tools
- [ ] Player
  - [ ] Default player + controls
  - [ ] Server side player + synchronization
  - [ ] Inventory
  - [ ] Drop pick up
  - [ ] Tool equipment
  - [ ] Tool usage
  - [ ] AI player controls
- [ ] Gameplay infrastructure
  - [ ] Mining mechanic
  - [ ] Miners
  - [ ] Mineral processing
  - [ ] Transportation - pipes/conveyors
  - [ ] Inter-planet transportation
  - [ ] Energy sources
  - [ ] Energy accumulation
  - [ ] Energy transportation - cables/packets
  - [ ] Base building
  - [ ] Plants, food
- [ ] Terraforming _LOW PRIORITY_
- [ ] Artificial satellites _LOW PRIORITY_
- [ ] Mobs _LOW PRIORITY_
- [ ] Bots _SUPER LOW PRIORITY_
- [ ] Radiation _SUPER LOW PRIORITY_
- [ ] Mount _SUPER LOW PRIORITY_
- [ ] Meteor showers _SUPER LOW PRIORITY_
- [ ] Polar auroras _SUPER LOW PRIORITY_

# Features
1. You spawn on a **planet**
2. Planet may have other **satellite-planets** (I think we will limit it to up to 3 planets per scene)
3. Planets are **fully destructible** (except for the core probably). But the destruction speed is limited
   so I do not expect player to destroy big parts of the planet.
4. Player may equip **jetpack** or other "flying gadgets" so he can fly above the surface of the planet, 
   or to the other planet if power of the jetpack is high enough.
   It can also run out of fuel so you should recharge it.
5. Player may equip tools that will help him destroy or move things. 
   It may be "melee" (aka **pickaxe**) tools or "ranged" tools (aka **lasers**) or even **explosives**.
6. Player may use "mounts" for faster movement (**rovers** or **hexapods**)
7. **Artificial satellites**. Player may launch satellites which may provide communication, observation or even defense
   of some structures. 
8. There are minerals under the surface of planet, you can mine them by hand. Or you can use automated miners.
   Miners are just spitting out the resource nearby.
9. Minerals processing machines may smelt, combine, form and transform minerals into some materials that you will need
   for crafting.
10. To power all these machines you need a power sources - main power source is solar panels.
11. You can attach transportation pipes between all the mechanism to interconnect them. You can also just load everything manually.
12. Inter-planet transportation infrastructure: spin-launchers, space-cannons
13. Radiation, you should hide from radiation under the surface or inside the base.
14. Base! It may be made from different rooms and connection between them, you can place different mechanisms inside, they will be protected
    from radiation and micro-meteors. You can plant and grow food in there!
15. Helper bots - they may transport items, may look after plants and do other things.
16. Terraforming! You may change the atmosphere of the planet, change the density or chemical mixture.
    You may change the ground, you may add artificial rivers or hydration system, grow trees and plants right on the ground!
17. Meteor showers! If the atmosphere is dense enough you can see falling meteors
18. Polar lights! (Aurora)
19. Mobs!

# Gameplay
You should terraform the planet! You can build your base, build the infrastructure, mines, pipes, launch satellites.
Each planetary system may have different features: one planet with all the resources, planet and sattelite with important resources,
planet with coal and sattelite where you should burn this coal, send packets of energy back to planet etc. etc.

# History~~
Because of wrong gravity assist you ended up on an unknown planet in space.