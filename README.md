# Skitgubbe Simulator

When some nerdy friends were playing a card game they argued over who could create the best algorithm to beat each other and so this simulator was created in order to virtually play the game and establish who was the best. 

The simulator works by hosting a websocket server that can simulate multiple games at a time. Information is fed to players/ algorithms via the websocket (such as whose turn, what cards you have, etc). Players are placed into a queue upon joining and when enough players are in the queue a game will start. 

## What is it

"Skitgubbe" is a Swedish card game that translates to "Dirty Old Man" in English. It is a trick-taking game played with a standard deck of 52 cards. The game is typically designed for three or more players and involves elements of strategy and skill in winning tricks and avoiding certain cards. The rules may vary, but the game generally follows a trick-taking format where players try to win as many tricks as possible while avoiding specific cards that carry penalty points. Skitgubbe is a popular and social card game in Sweden, often played for entertainment and friendly competition.

Alternate names:
- Gammelholk
- Ruckspel
- Skittpoker
- Skitstövel

# TODO

- Database connection and design
- ELO system
- Fair queue system
    - make sure players aren't playing against the same people every time (hopefully the ELO system will help with this)
- Handle timeouts for players in game not responding
- Setup system to debug if crashes occur in production
