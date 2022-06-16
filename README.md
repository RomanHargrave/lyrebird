Lyrebird
========

Lyrebird is a small application designed to assist in the testing of
monitoring agents.

It provides platform-agnostic interfaces to create processes, modify
files, and simulate (basic) exfiltration. Each of these activities is
logged to a file for later comparison against the telemetry collected
by the agent under test.

## Usage

Lyrebird exposes features through subcommands, one for each feature.
Full usage details are provided by Lyrebird itself, by running
`lyrebird help`.

## Logging

Events are logged as a series of records, specifically as a series of
newline-separate JSON objects. This approach is informally known as
[newline-delimited JSON](http://ndjson.org/) and is frequently used
for applications such as logging or for exchange of large non-uniform
datasets.

By default, the log file is located as described below for each
platform; however, a different location may be specified by setting
the `LYREBIRD_LOG` environment variable.

- On Linux, the log file is stored at `/tmp/lyrebird.log`
- On Windows, the log file is stored at `%TEMP%/lyrebird.log`

### Log Record Structure

Log records conform to the following structure:

```json
{
    "type": "RecordType",
    "time": "1970-01-01T00:00:00Z",
    "pid": 1234,
    "cmd": ["lyrebird", ...],
    "user": "root",
    "data": {
        ...
    }
}
```

`type` may contain the following values, for which `data` has
differing members:

- `StartProcess` events are logged when process creation (execution)
  is attempted. Logged data includes the child process command line,
  as well as the child process PID, as follows:
  ```json
  {
      "data": {
          "cmd": ["child-exe", ...],
          "pid": 4567
      },
      ...
  }
  ```
  
- `File` events are logged when file modification is attempted. File
  events include the following data, where `.data.action` may be one
  of `create`, `modify`, or `delete`:
  ```json
  {
      "data": {
          "action": "create|modify|delete",
          "file": "/path/to/file"
      },
      ...
  }
  ```

- `NetSend` events are logged when data exfiltration is attempted.
  These events include the following data:
  ```json
  {
      "data": {
          "proto": "TCP",
          "src_addr": "169.254.0.1",
          "src_port": 65535,
          "dst_addr": "169.254.0.2",
          "dst_port": 25,
          "bytes": 1024
      },
      ...
  }
  ```

