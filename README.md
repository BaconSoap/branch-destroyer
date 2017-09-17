# BRANCH DESTROYER

[![Build Status](https://travis-ci.org/BaconSoap/branch-destroyer.svg?branch=master)](https://travis-ci.org/BaconSoap/branch-destroyer)
[![Build status](https://ci.appveyor.com/api/projects/status/ueqx9gk8nu3ferqn?svg=true)](https://ci.appveyor.com/project/BaconSoap/branch-destroyer)


Deletes fully merged branches (0 ahead of repository's default branch) from a repository.

## Usage

To see what would get deleted:

```sh
branch-destroyer -t <personal oauth token> -o baconsoap -r branch-destroyer --days 7
```

To really run the delete:

```sh
branch-destroyer -t <personal oauth token> -o baconsoap -r branch-destroyer --days 7 --for-real
```