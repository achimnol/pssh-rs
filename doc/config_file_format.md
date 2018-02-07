Config file format
==================

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
