# 2. language adapter

Date: 2020-10-25

## Status

2020-10-25 proposed

## Context

In Java or Kotlin cases, like KotlinPoet, it will allow user to pass `String::class` to get the type name, like
`java.lang.String`. In a MVP design, we can try to build a language adapter for different language ,the use CLI call it.

For example:

```
java -jar poet-java.jar String::class
```

in this case, it can return `Java.lang.String`.

For the future, we can use another tools to generate Java code to build Java artifact.

## Decision

Decision here...

## Consequences

Consequences here...
