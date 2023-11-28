# Scripts
A collection of scripts to help perform a lot of tasks that I think simply don't belong in the rust program because it's a lot of work

```ollama_modelfiles.py``` accepts various actions around Ollamas modfile format and prints a rusty-llama json file to stdout
Arguments include:
- ```pull <model names>``` pulls the listed models and modelfiles saves the models to ```$HOME/models/$name``` and outputs a json config for rusty-llama
- ```from <file names>``` much the same as pull accepts it work off of local modelfiles
