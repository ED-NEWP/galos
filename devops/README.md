# DevOps

# AWS ECR Docker registry

We store our release build Dockerized services in the AWS ECR registry.

These are the addresses of the per service repositories, the address is static:

| Service     | Docker repository address |
| ------------|---------------------------|
| galos-sync  | 261923651564.dkr.ecr.eu-central-1.amazonaws.com/galos-sync |

## ECR credentials configuration

Obtain the `aws_access_key_id` and `aws_secret_access_key` for the ECR repository.  
Place the credentials on the Docker host in `~/.aws/credentials` file. It should look like this:

```toml
[default]
aws_access_key_id=KEY_ID
aws_secret_access_key=SECRET_KEY
```

Every ECR repository is located in a specific region that must be specified in the `~/.aws/config` file:

```toml
[default]
region=ECR_REPOSITORY_REGION
```

To configure Docker to work with the ECR repository, you need to install ECR credential helper for Docker:

```sh
$ apt-get update -y
$ apt-get install docker-credential-ecr-login
```

Then, _add_ the fllowing to the `~/.docker/config.json` file:

```json
{
	"credsStore": "ecr-login",
	"credHelpers": {
		"261923651564.dkr.ecr.eu-central-1.amazonaws.com/galos-sync": "ecr-login"
	}
}
```

This tells Docker to use the ECR credential helper for the specific registry.

# Service configuration

The `galos-sync` service requires configuration in the form of environment variables. Place the values in the `~/.env` file, `docker-compose` will pick it up and inject the environment variables into the container.

```
DATABASE_URL=postgresql://<<hostname>>/elite_development
```

# Container operations

## Updating the Docker image

To update the `latest` tag of the `galos-sync` Docker image, run the following command:
To update the service Docker to the latest version, run the following:

```sh
$ docker-compose pull
```

This will pull the `latest` image tag without updating the container. To update the container, refer to the [starting](#starting) section.

## Starting

> If a database migration is required, refer to the [Database Migration](#database-migration) section **before** you proceed.

To start a new service conatiner, run:

```sh
$ docker-compose up -d
```

If you updated the Docker image beforehand, this command will use the new image.

To start a stopped container, run:

```sh
$ docker-compose start
```

This will only start a stopped conatiner without updating it to the latest image version.

## Stopping

To stop the service, run:

```sh
$ docker-compose down
```

Use this command if you intend to update the container image. It will stop the container and delete it.

To stop the container without deleting, run:

```sh
$ docker-compose stop
```

Use this command if you need to temporarily stop the container.

# Database migration

When a database migration is required, do the following:

1. [Stop](#stopping) the container.
1. Run database migrations and make sure it _completes successfully_.
1. [Update](#updating-the-docker-image) the container to the latest version.
1. [Start](#starting) the container.