# Dockerized Example

## Running The Example/Demo

### 1. Ensures

- You have [python3](https://www.python.org/downloads/), [python3-venv](https://docs.python.org/3/tutorial/venv.html), and [python3-pip](https://packaging.python.org/tutorials/installing-packages/#install-pip-setuptools-and-wheel) installed.
- [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/) installed.
- [Github Packages Docker Authenticated](https://docs.github.com/en/packages/guides/configuring-docker-for-use-with-github-packages#authenticating-to-github-packages).
- Configured the [environment variables](https://github.com/Tapalogi/tapa-micro-mailer/blob/176c4233fd2c3339cc6454f9848c8e00af7e25f3/example/local.env#L6-L13) correctly.
- Cloned this repository
- Activated the `venv` and installed the required python packages.

### 2. Activate Python3 Virtual Environment

```bash
> python3 -m venv .venv
> source .venv/bin/activate
> pip install -r example/requirements.txt
```

### 3. Running NATS & Mailer Service

```bash
> docker-compose -f example/docker-compose.yml up
```

### 4. Running Inspector (for viewing produced events)

```bash
> python example/inspector.py
```

### 5. Running Drafter (for producing email drafts)

```bash
> python example/drafter.py
```

## Demonstration Video

[![Tapalogi's Mailer Microservice - Demonstration](http://img.youtube.com/vi/4OCYvMJv3g4/0.jpg)](http://www.youtube.com/watch?v=4OCYvMJv3g4 "Tapalogi's Mailer Microservice - Demonstration")
