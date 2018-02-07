pssh - simple SSH connection utility
====================================

[![pipeline status](https://gitlab.com/Srynetix/pssh-rs/badges/master/pipeline.svg)](https://gitlab.com/Srynetix/pssh-rs/commits/master)

## Key features

- Define all of your connection configurations in only **one YAML file**.
- Each configuration is organized in **namespaces** (*and nested namespaces*).
- **Default values** system with namespace-specific and machine-specific **overrides**.

## Simple example

#### Configuration file content (config.yml)
```yaml
defaults:
  $:
    user: test
    port: 22
    identity: ~/.ssh/id_rsa
  work:
    $:
      user: workuser
      port: 2233
      identity: ~/.ssh/work_id_rsa

machines:
  localhost:
    $:
      ip: localhost
  work:
    test01:
      $:
        ip: test01.work.dev
    test02:
      $:
        ip: test02.work.dev
        port: 2244
```

#### Machine configurations output

- **localhost**
    - **IP**: localhost
    - **User** test *(from defaults)*
    - **Port**: 22 *(from defaults)*
    - **Identity**: ~/.ssh/id_rsa *(from defaults)*


- **work:test01**
    - **IP**: test01.work.dev
    - **User**: workuser *(from defaults:work)*
    - **Port**: 2233 *(from defaults:work)*
    - **Identity**: ~/.ssh/work\_id\_rsa *(from defaults:work)*


- **work:test02**
    - **IP**: test02.work.dev
    - **User**: workuser *(from defaults:work)*
    - **Port**: 2244 *(override)*
    - **identity**: ~/.ssh/work\_id\_rsa *(from defaults:work)*

## Usage

By default, `pssh` try to load the `config.yml` file in your `~/.pssh` folder. You can specify another configuration file using the `-f` or `--file` argument before your command.

- List available machines.
    - ```pssh list```


- Connect to the `test01` machine from a `work` namespace.
    - ```pssh connect work:test01```


- Show the `test02` configuration status from a `work` namespace.
    - ```pssh show work:test02```


- Push file to `localhost` machine.
    - ```pssh push localhost ./pouet.txt /tmp/pouet.txt```


- Pull file from `localhost` machine.
    - ```pssh pull localhost /tmp/pouet.txt ./pouet.txt```
