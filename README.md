# enzo

**IGNORE DOCUMENTATION BELOW. THIS IS PURELY EXPERIMENTAL RIGHT NOW**

A good analogy is Makefile and using make

**flow**
A file that describes the actions that a developer needs to perform to configure a repo

**flow.yaml**
A file that contains various flows!

// TODO: document the syntax for describing flows

## Some commands and what I expect them to do

```
enzo new event-site
```
@Short
Creates a new project packaged with tools to make a stunning event site!

@Long
This command clones the event site template contained in [est](https://www.github.com/HackGT/est) and walks through the flow contained in its enzo.yaml.
It will install dependencies needed, and will present you with some default tasks to perform when building the event site.
More details on the dependencies, and how they fit in with the template can be found at [est](https://www.github.com/HackGT/est).


```
enzo deploy [repo-name] [branch] [url] [--dev]
```
@Short
Deploys [repo-name] (defaults to current directory) using beekeeper!

@Long
Looks for the corresponding yaml file in beekeeper. 
If the file is found, it will simply update the url and branch if needed
If the file is not found, it will ask if you want to create it.
This command will clone beekeeper into your HackGT workspace and will commit any necessary changes.


```
enzo tasks [repo-name]
```
Prints out a list of tasks (with completed status) contained in the enzo.yaml file of [repo-name].

## Describing a flow using yaml

Let's think about an example flow to figure out what keys, values, and other stuff you can support

**Iteration #1**
Example flow file for setting up piranha
```yaml
config:
  Build .env file:
    - question:
        prompt: Are you using a development environment? 
        options:
          - true
          - false
        response:
          store in: DEBUG
    - question:
        prompt: Enter the SECRET_KEY
        hints:
          - Generate using url. You'll need to put val in quotes
        response:
          store in: SECRET_KEY
  Finishing steps:
    - run:
       - # enter virtual env
       - python mange.py migrate
       - python manage.py createsuperuser
       - python manage.py runserver
       - command:
           exec: npm install wowo
           confirm: true
           description: installs the wowo dependency
    - env:
        name: .env
        data:
          - DEBUG
          - SECRET_KEY

tasks:
  - Do this for me:
      - sub tasks
      - sub tasks
  - Do this also
  - Also this:
      - sub task
      - sub task
```

**Iteration #2**
```yaml
config:
  - section 1:
    - question:
        prompt: <the prompt for the question>
        hints:
          - hints to answer the question correctly
        response:
          # what you want to do with the response
          store in: VAR_NAME
    - run:
        - bash
        - commands
        - you can also reference VAR_NAME as $VAR_NAME
    - env:
        # specify key-value pairs for your env file
        name: .env
        data:
          KEY: $VAR_NAME
          KEY1: some value
  - section 2
  - section 3

# what should developers do?
tasks:
  - task 1
  - task 2:
      - notes: notes in here
      - resources: resources in here
      - sub-task 1
      - sub-task 2
  - task 3
```

## Config file

```
workspace = ~/Documents/projects/hackgt
remote = https://www.github.com
default_org = HackGT

alias est = HackGT/event-site
```

// need to write a parser for
* getting the key value pairs
* parsing the values of each key

