import argparse
import json


# Classes and data structures

class ModelConfig:
    def __init__(self, mirostat=0, mirostat_eta=0.1, mirostat_tau=5.0, num_ctx=2048,
                 num_gqa=None, num_gpu=None, num_thread=None, repeat_last_n=64,
                 repeat_penalty=1.1, temperature=0.8, seed=0, stop=None,
                 tfs_z=1.0, num_predict=128, top_k=40, top_p=0.9):
        self.mirostat = mirostat
        self.mirostat_eta = mirostat_eta
        self.mirostat_tau = mirostat_tau
        self.num_ctx = num_ctx
        self.num_gqa = num_gqa
        self.num_gpu = num_gpu
        self.num_thread = num_thread
        self.repeat_last_n = repeat_last_n
        self.repeat_penalty = repeat_penalty
        self.temperature = temperature
        self.seed = seed
        self.stop = stop
        self.tfs_z = tfs_z
        self.num_predict = num_predict
        self.top_k = top_k
        self.top_p = top_p

    def to_json(self):
        return json.dumps(self, default=lambda o: o.__dict__,
                          sort_keys=True, indent=4)


class ModelDefinition:
    def __init__(self, path, name, config=None):
        self.path = path
        self.name = name
        self.config = config if config is not None else ModelConfig()

    def to_json(self):
        return json.dumps(self, default=lambda o: o.__dict__,
                          sort_keys=True, indent=4)


class ModelfileParser:
    def __init__(self, modelfile_content):
        self.modelfile_content = modelfile_content
        self.config = ModelConfig()  # Assuming ModelConfig class is defined as before
        self.from_content = None  # Placeholder for storing 'FROM' related data

    def parse(self):
        for line in self.modelfile_content.splitlines():
            self._parse_line(line.strip())

        return self.config

    def _parse_line(self, line):
        if line.startswith('PARAMETER'):
            self._parse_parameter(line)
        elif line.startswith('FROM'):
            self._parse_from(line)
        # Add parsing for TEMPLATE, SYSTEM, ADAPTER, LICENSE as needed

    def _parse_parameter(self, line):
        parts = line.split()
        if len(parts) < 3:
            return  # Invalid line format

        parameter, value = parts[1], parts[2]
        if hasattr(self.config, parameter):
            value = self._convert_value(getattr(self.config, parameter), value)
            setattr(self.config, parameter, value)

    def _parse_from(self, line):
        from_content = ' '.join(line.split()[1:])
        # TODO: Get the model name and trace it up in to a link(?) using the pull methods
        # TODO: make the from statements grab more info like the model config params

        self.from_content = from_content

    def _convert_value(self, current_value, new_value_str):
        if isinstance(current_value, int):
            return int(new_value_str)
        elif isinstance(current_value, float):
            return float(new_value_str)
        elif isinstance(current_value, str):
            return new_value_str
        else:
            return new_value_str  # Default case, possibly needs handling for other types

    def to_json(self):
        # Custom function to skip None values
        def skip_none(obj):
            return {k: v for k, v in obj.__dict__.items() if v is not None}

        return json.dumps(skip_none(self), sort_keys=True, indent=4)


def pull_models(model_names):
    o = []
    for i in model_names:
        model_config = ModelConfig()
        model_def = ModelDefinition(path=i, name="test", config=model_config)
        o.append(model_def)
    return o


def from_files(file_names):
    o = []
    for i in file_names:
        model_config = ModelConfig()
        model_def = ModelDefinition(path="/path/to/model", name=i, config=model_config)
        o.append(model_def)
    return o


def main():
    parser = argparse.ArgumentParser(description='Ollama Modelfiles Processing Script')
    subparsers = parser.add_subparsers(dest='command')

    # Parser for 'pull' command
    pull_parser = subparsers.add_parser('pull', help='Pulls the listed models and modelfiles')
    pull_parser.add_argument('model_names', nargs='+', help='Model names to pull')

    # Parser for 'from' command
    from_parser = subparsers.add_parser('from', help='Work off of local modelfiles')
    from_parser.add_argument('file_names', nargs='+', help='File names to process')

    args = parser.parse_args()

    if args.command == 'pull':
        config = pull_models(args.model_names)
    elif args.command == 'from':
        config = from_files(args.file_names)
    else:
        parser.print_help()
        return

    print(json.dumps(config, indent=4))


if __name__ == "__main__":
    main()
