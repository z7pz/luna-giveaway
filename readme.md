# Todo

## Stage 1
- [X] hydrate from database on restart
- [X] dynamic prefix
- [X] enable/disable commands on check_command
- [X] custom roles that have access to the commands
- [X] custom start embed and end embed
- [X] custom end message
- [X] reaction


## Stage 2
- [X] Create API
- [X] Discord auth
	- /login
	- /redirect?code
	- user

## Stage 3
- [ ] pause/resume commands
- [ ] webhook (premium plan)
- [ ] debounce

## Bugs
- [ ] dont allow bots to interact
- [ ] 



# Giveaway settings
- timer (required)
- prize (required)
- winners count (required)
- create rules (account age, active messages in specific room, invite counter) (optional)
- type (reaction/button) (optional default guild settings)


# User Profile
- show user giveaways (active/finished/paused) (rules required)


# Routes
### Server only
- [ ] GET /server/:id/settings to get all server's settings
- [ ] POST /server/:id/settings to set settings to the server
- [ ] GET /server/:id/giveaways to list all giveaways

## Public
- [X] GET /public/commands to get all bot's commands
- [X] GET /public/giveaways to list all active giveaways

## User 
- [ ] GET /giveaways to get joined giveaway
