1. question: is the server in this shard?
<!-- 2. question: how to create job in rustlang -->
3. todo: command middleware (check if slash commands or prefix commands are enabled)
4. todo: localization
<!-- 5. question: how to create caching in rustlang (maybe HashMap again?) -->
<!-- 6. question: how to use sharding with cache -->
- embed: add buttons or reactions
- embed: edit message on end
- event: listen to ReactionAdd and ButtonInteraction
- db: connect the prisma client
- db: save giveaway
- db: edit status of the giveaway to ended: true