Thoughts on enzo

goals
* make it easier to create event sites, live sites, teaser sites, deploy websites
* make it easy to create new instances of a tool
* easy access to HackGT documentation

streamline how we run our events from a tech perspective


sample commands that I want to see:

each command initiates a flow
flow can be described using a yaml file

enzo new event-site
    initiates event-site config flow
        name of the event
        year
        will you be using cms?
            if yes
                cms deployment flow
                    deploy a dev instance
                    fields are event based
                copy boilerplate
    creates default tasks
        developer can keep track of what to do
            eg: talk to design
            get figma assets
            perform lighthouse checks
            responsitivity
    developer uses enzo to finish the job in time!
    developer can also get links to resources for things he doesn't know

enzo new cms
    initiates the cms flow
        to be used for creating new instances
        is this a dev instance, or a prod one
        if exists
            do you want to switch to prod?
        ask for url and stuff

enzo new live-site
enzo deploy <whatever tool is to be deployed>
enzo help 
enzo docs
// command for creating flows; a flow builder


enzo is built on flows
    doesn't require me to keep pushing changes to enzo and updating it
    rather, it depends on HackGT projects having a "flow.yaml" file or something of that sorts that
    goes over how to configure and deploy the whole thing
    reads the "flow.yaml" and can autogenerate
        cli userflows
        markdown documentation

Let's start defining some of the terms that I used!!!


A good analogy is Makefile and using make

**flow**
A file that describes the actions that a developer needs to perform to configure a repo

**enzo.yaml**
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
