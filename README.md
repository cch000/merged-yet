# merged-yet

Simple tool that uses the github api to check if a pull request has been merged in a branch of nixpkgs.
Feel free to contribute!

## Usage

```console
> merged-yet -a $GITHUB_API_KEY -f 378069
#378069
├ ✅ master
├ ✅ nixos-unstable
```

### Nix

#### Basic cli usage

```console
> nix run github:cch000/merged-yet -- [OPTIONS] <PR_NUMBER>
```

#### Adding it to your config

Useful if you want to use it frequently or for scripting.

1. Add the flake to your system flake inputs:

```Nix
merged-yet.url = "github:cch000/merged-yet";
```

2. Add it to your packages

```Nix
environment.systemPackages = [
  inputs.merged-yet.packages.merged-yet
];
```

### Non-Nix usage

```console
> git clone https://github.com/cch000/merged-yet
> cargo run -- [OPTIONS] <PR_NUMBER>
```

## Options

```
  -b, --branch <BRANCH>        Branch in which to look for the pull request [default: nixos-unstable]
  -m, --max-pages <MAX_PAGES>  Each page is one request [default: if no key was provided 5, else 100]
  -s, --scripting              Whether to output script-friendly values
  -f, --full                   Whether to output if the pr was first merged into master
  -a, --api-key <API_KEY>      Github api key
  -t, --threads <THREADS>      Number of threads [default: MAX]
  -h, --help                   Print help
  -V, --version                Print version

```
