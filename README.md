# BRANCH DESTROYER

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