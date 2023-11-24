# rusty-llama
A rust implementation of a very small part of the ollama api, built for performance critical production environments.

## So what does it do?
As of now all it does is act as a stand in for the ```/generate``` route and tries to do it as fast as possible.
## What doesn't it do?
A whole lot -- this was built to scatch an itch I had in some of my production situations. it can only open one model at a time (declared on server start) and as of now does not have any of the push/pull/etc abilities.
## Why base it on ollama then?
I really like the ollama API and what it plans to do, I just don't think it's ideal for some production environments. My goal with this is to give people who want to build with ollama a better optomized version for production so they can build things that work locally too.

## TODO:
- [ ] Make it work
- [ ] Write scripts that implement the pull/push methods
- [ ] Write sample ```Dockerfile```'s