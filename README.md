[![CI](https://github.com/ED-NEWP/galos/actions/workflows/ci.yml/badge.svg)](https://github.com/ED-NEWP/galos/actions/workflows/ci.yml) [![CD](https://github.com/ED-NEWP/galos/actions/workflows/cd.yml/badge.svg)](https://github.com/ED-NEWP/galos/actions/workflows/cd.yml)

Somewhere between reality and the flight sim E:D.

**NOTE: This is a WIP, the information in here may be unimplemented.**


### Database setup

Read the [galos_db documentation].

### Continuous Integration

We use GitHub Actions to build and run tests on _every push_ to the `master` branch and to any pull request. You should not create a PR until your code is ready for integration into the `master` branch. If you do, every push to your working branch will cause the CI workflow to run.

The CI workflow is defined in `.github/workflows/ci.yml`.

### Continuous Delivery

We use GitHub Actions to build a Dockerized release version of `galos-sync` on _every push_ to the `master` branch.

The build process runs in the GitHub Actions worker and the binary is then copied into a small [distroless](https://github.com/GoogleContainerTools/distroless) Docker image.

The Docker image is pushed to an AWS ECR repository after the image is created.
The AWS account secrets are managed in the repository's settings, under `secrets`.

The CD workflow is defined in `.github/workflow/cd.yml`.

### Running the container

> If a database migration is required, refer to the [Database Migration](#database-migration) section **before** you proceed.

The container requires configuration to run, place the values in a `.env` file and reference the file in the `docker run` command (see below).

Container configuration:

```
DATABASE_URL=postgresql://<<hostname>>/elite_development
```

Pull the latest image:

```sh
$ docker pull 261923651564.dkr.ecr.eu-central-1.amazonaws.com/galos-sync
```

Run the container with a [restart policy](https://docs.docker.com/config/containers/start-containers-automatically/) to auto-restart in case the container crashes:

```sh
$ docker run                \
  --detach                  \
  --restart unless-stopped  \
  --env-file ~/.env         \
  --name galos-sync         \
  261923651564.dkr.ecr.eu-central-1.amazonaws.com/galos-sync
```

### Database migration

When a database migration is needed, do the following:

1. Stop the container:

```sh
$ docker stop galos-sync
```

2. Run database migrations and make sure it _completes successfully_:

```sh
$ sqlx migrate run
```

3. Update the docker image and start the container as described in the [Running the container](#running-the-container) section.