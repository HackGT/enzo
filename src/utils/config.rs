pub fn default_config() -> String {
    String::from(r#"config:
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
"#)
}
