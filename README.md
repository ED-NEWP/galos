[![CI](https://github.com/ED-NEWP/galos/actions/workflows/ci.yml/badge.svg)](https://github.com/ED-NEWP/galos/actions/workflows/ci.yml) [![CD](https://github.com/ED-NEWP/galos/actions/workflows/cd.yml/badge.svg)](https://github.com/ED-NEWP/galos/actions/workflows/cd.yml)

Somewhere between reality and the flight sim E:D.

**NOTE: This is a WIP, the information in here may be unimplemented.**


### Database Setup

Read the [galos_db documentation].

### DevOps - Services

Read the [DevOps](./devops/README.md) documentation for the following:

- Docker container registry configuration and operation.
- Operation of services using Docker.
- Database migrations.

### Continuous Integration

We use GitHub Actions to build and run tests on _every push_ to the `master` branch and to any pull request. You should not create a PR until your code is ready for integration into the `master` branch. If you do, every push to your working branch will cause the CI workflow to run.

The CI workflow is defined in [.github/workflows/ci.yml](.github/workflows/ci.yml).

### Continuous Delivery

We use GitHub Actions to build a Dockerized release version of `galos-sync` on _every push_ to the `master` branch.

The build process runs in the GitHub Actions worker and the binary is then copied into a small [distroless](https://github.com/GoogleContainerTools/distroless) Docker image.

The Docker image is pushed to an AWS ECR repository after the image is created.
The AWS account secrets are managed in the repository's settings, under `secrets`.

The CD workflow is defined in [.github/workflow/cd.yml](.github/workflow/cd.yml).
