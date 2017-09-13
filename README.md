# diagnostic

The purpose of this Rust project is to provide a runtime for the [loq-ql](https://github.com/schultyy/log-ql) project. It provides a commandline interface which accepts queries from users and executes them for a specified file.

## Usage

```
$ diagnostic "SELECT severity FROM 'travis_log_last_2000.log' WHERE severity = 'INFO'"
```

This returns all lines from the `travis_log_last_2000.log` file where the row's severity is `INFO`.

## Configuration

diagnostic needs to have a configuration file in place which helps it to find out which columns exist for a given file (type).

This configuration file can look like this:

```
{
  "file_type": "travis_log_last_2000.log",
  "columns": [
    {
      "name": "date",
      "capture_group": 0,
      "regex": "\\[(\\d+-\\d+-\\d+T\\d+:\\d+:\\d+\\+\\d+)\\]"
    },
    {
      "name": "component",
      "capture_group": 0,
      "regex": "\\[((\\w+-?:?)+)\\]"
    },
    {
      "name": "message",
      "capture_group": 2,
      "regex": "\\[\\d+-\\d+-\\d+T\\d+:\\d+:\\d+\\+\\d+\\]\\[(\\w+-?:?)+\\](.+)"
    },
    {
      "name": "severity",
      "capture_group": 0,
      "regex": "(ERROR|INFO)"
    }
  ]
}
```

It has a `file_type` and a list of columns where each column got its own `name`, a `regex` and a `capture_group`. The `capture_group` parameter tells it which value it should take from the groups produced by the regular expression.
