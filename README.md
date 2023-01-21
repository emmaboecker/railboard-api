# Railboard API

**This Project is very WIP, unstable and far away from being completely functional, unexpected errors can occur**

This project generally is going to be the backend for my other project [railboard](https://github.com/StckOverflw/railboard), but the goal is to make an API
similar to the [bahn.expert API](https://github.com/marudor/bahn.expert) that marudor sadly shut down public support for and the documentation is not available any more.

If you have any questions, I'd love to talk about it with you! Hit me up on Mastodon or Twitter and send me your Discord (I usually don't accept Friend Requests from people that I dont share a Server with) so we can chat!

## Goal of This Project

The Goal of this project is to make the public transport API's of the German Train Service ([Deutsche Bahn](https://www.deutschebahn.com/)) more accessible
to use in custom projects. The few open APIs the company offers are very limited and the effort to use the APIs the Mobile Apps use is immense (source: trust me).

I think everyone that wants to build something cool should be able to. 

In addition to the REST API endpoints that I am building, the clients for the different APIs are completely usable without the rest of the Project.

## Documentation

Documentation is available at [https://api.rail.stckoverflw.net/docs](https://api.rail.stckoverflw.net/docs)

**But which API endpoint should I use?** \
The Iris endpoint is generally faster, only has data for the current day tho. If you want older/newer data refer to the Vendo Endpoint. \
If you want to also get journey details you should also use the Vendo API (for now, with the Ris/Hafas enpoints you will also be able to use those) since both APIs cannot really be used together..

## Roadmap
- [x] Major Vendo endpoints
- [x] Iris Station-Board
- [ ] Ris API endpoints
- [ ] Coach Sequence API
- [ ] Hafas API endpoints
- [ ] Custom endpoints with data from multiple sources

If you have any feature request of an idea feel free to [open an issue](https://github.com/StckOverflw/railboard-api/issues/new)

## Contribution 

Contributions are always welcome, I'd prefer if you contact me before opening a PR for now tho

_anyone that is willing to help with this project, feel free to hit me up on [Mastodon (@stck@chaos.social)](https://chaos.social)/[Twitter (@StckOverflw)](https://twitter.com/StckOverflw) 
or add me on Discord StckOverflw#2665_
