### tmplr ###
- Templating made easy!!!

person.tmpl
```
Hello, {{ name }}!
You are {{ age }} years old.
```

varaiables.yaml
```
name: John Doe
age: 24
```


# Usage:
```
docker run -v $(PWD):/tmp sivaramsajeev/tmplr -t /tmp/front.tmpl -v /tmp/vars.yaml -o /tmp/front.html
```
